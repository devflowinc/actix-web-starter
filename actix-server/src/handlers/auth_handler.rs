use crate::data::models::{OrgUserLink, RedisPool};
use crate::operators::user_operator::create_user_query;
use crate::{
    data::models::{PgPool, User},
    errors::ServiceError,
    operators::user_operator::get_user_by_id_query,
};
use actix_identity::Identity;
use actix_web::{web, Error, FromRequest, HttpMessage as _, HttpRequest, HttpResponse};
use bb8_redis::redis::AsyncCommands;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier,
    RedirectUrl, Scope, TokenResponse,
};
use openidconnect::core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata};
use openidconnect::{AccessTokenHash, ClientId, IssuerUrl, Nonce};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::read_to_string;
use utoipa::{IntoParams, ToSchema};

pub type AuthedUser = User;

impl FromRequest for AuthedUser {
    type Error = actix_web::Error;
    type Future = std::future::Ready<Result<AuthedUser, actix_web::Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        std::future::ready(
            req.extensions()
                .get::<AuthedUser>()
                .cloned()
                .ok_or(ServiceError::Unauthorized.into()),
        )
    }
}
#[derive(Deserialize, Debug, IntoParams)]
pub struct OpCallback {
    pub state: String,
    pub session_state: String,
    pub code: String,
}

pub type AuthedOrgMembership = OrgUserLink;

impl FromRequest for AuthedOrgMembership {
    type Error = actix_web::Error;
    type Future = std::future::Ready<Result<AuthedOrgMembership, actix_web::Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        std::future::ready(
            req.extensions()
                .get::<OrgUserLink>()
                .cloned()
                .ok_or(ServiceError::Unauthorized.into()),
        )
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
struct AFClaims {}

#[tracing::instrument]
pub async fn build_oidc_client() -> CoreClient {
    let issuer_url =
        std::env::var("OIDC_ISSUER_URL").expect("Issuer URL for OpenID provider must be set");
    let client_id =
        std::env::var("OIDC_CLIENT_ID").expect("Client ID for OpenID provider must be set");
    let auth_redirect_url = std::env::var("OIDC_AUTH_REDIRECT_URL")
        .expect("Auth redirect URL for OpenID provider must be set");
    let client_secret =
        std::env::var("OIDC_CLIENT_SECRET").expect("Client secret for OpenID provider must be set");
    let base_server_url =
        std::env::var("BASE_SERVER_URL").expect("Server hostname for OpenID provider must be set");

    //build OpenId Connect client
    let meta_data = CoreProviderMetadata::discover_async(
        IssuerUrl::new(issuer_url.clone()).expect("IssuerUrl for OpenID provider must be set"),
        async_http_client,
    )
    .await
    .expect("Failed to discover OpenID provider");

    CoreClient::new(
        ClientId::new(client_id.clone()),
        Some(ClientSecret::new(client_secret.clone())),
        IssuerUrl::new(issuer_url.clone()).expect("IssuerUrl for OpenID provider must be set"),
        AuthUrl::new(auth_redirect_url.clone()).expect("Auth configuration is not a valid URL"),
        meta_data.token_endpoint().cloned(),
        meta_data.userinfo_endpoint().cloned(),
        meta_data.jwks().to_owned(),
    )
    .set_redirect_uri(
        RedirectUrl::new(format!("{}/api/auth/callback", base_server_url))
            .expect("Redirect URL for OpenID provider must be set"),
    )
}

#[tracing::instrument(skip(pg_pool))]
pub async fn create_account(
    email: String,
    name: String,
    user_id: uuid::Uuid,
    pg_pool: web::Data<PgPool>,
) -> Result<User, ServiceError> {
    let user_org = create_user_query(user_id, email, Some(name), pg_pool).await?;

    Ok(user_org)
}

#[derive(Deserialize, Debug, IntoParams)]
pub struct LogoutRequest {
    pub redirect_uri: Option<String>,
}

/// Logout
///
/// Invalidate your current auth credential stored typically stored in a cookie. This does not invalidate your API key.
#[utoipa::path(
    delete,
    path = "/auth",
    context_path = "/api",
    params(
        LogoutRequest
    ),
    tag = "auth",
    responses(
        (status = 204, description = "Confirmation that your current auth token has been invalidated. This does not invalidate your API key."),
        (status = 401, description = "The user is already logged out / does not have an account"),
    ),
)]
#[tracing::instrument(skip(id))]
pub async fn logout(
    id: Identity,
    data: web::Query<LogoutRequest>,
    req: HttpRequest,
) -> HttpResponse {
    id.logout();
    let issuer_url =
        std::env::var("OIDC_ISSUER_URL").expect("Issuer URL for OpenID provider must be set");
    let client_id =
        std::env::var("OIDC_CLIENT_ID").expect("Client ID for OpenID provider must be set");

    let logout_url = format!(
        "{}/protocol/openid-connect/logout?post_logout_redirect_uri={}&client_id={}",
        issuer_url,
        data.redirect_uri.clone().unwrap_or(
            req.headers()
                .get("Referer")
                .map_or("/", |h| h.to_str().unwrap_or("/"))
                .to_string()
        ),
        client_id
    );

    HttpResponse::Ok().json(json!({
        "logout_url": logout_url,
    }))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenIdConnectState {
    pub pkce_verifier: PkceCodeVerifier,
    pub csrf_token: CsrfToken,
    pub nonce: Nonce,
}

const OIDC_SESSION_KEY: &str = "oidc_state";

#[derive(Deserialize, Debug, ToSchema, IntoParams)]
#[schema(
    example = json!({"redirect_uri": "https://api.trieve.ai"}),
)]
pub struct AuthQuery {
    /// URL to redirect to after successful login
    pub redirect_uri: Option<String>,
    /// Code sent via email as a result of successful call to send_invitation
    pub inv_code: Option<uuid::Uuid>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginState {
    /// URL to redirect to after successful login
    pub redirect_uri: String,
}

/// Login
///
/// This will redirect you to the OAuth provider for authentication with email/pass, SSO, Google, Github, etc.
#[utoipa::path(
    get,
    path = "/auth",
    context_path = "/api",
    tag = "auth",
    params(AuthQuery),
    responses(
        (status = 303, description = "Response that redirects to OAuth provider through a Location header to be handled by browser."),
        (status = 400, description = "OAuth error likely with OIDC provider.", body = ErrorRespPayload),
    )
)]
#[tracing::instrument(skip(oidc_client, redis_pool))]
pub async fn login(
    req: HttpRequest,
    redis_pool: web::Data<RedisPool>,
    data: web::Query<AuthQuery>,
    oidc_client: web::Data<CoreClient>,
) -> Result<HttpResponse, Error> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token, nonce) = oidc_client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .add_scope(Scope::new("openid".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    let oidc_state = OpenIdConnectState {
        pkce_verifier,
        csrf_token,
        nonce,
    };

    let mut redis_conn = redis_pool.get().await.unwrap();
    let _: () = redis_conn
        .set(
            OIDC_SESSION_KEY,
            serde_json::to_string(&oidc_state).unwrap(),
        )
        .await
        .map_err(|_| {
            ServiceError::InternalServerError("Failed to set OIDC session state".into())
        })?;

    let redirect_uri = match data.redirect_uri.clone() {
        Some(redirect_uri) => redirect_uri,
        None => req
            .headers()
            .get("Referer")
            .map(|h| h.to_str().unwrap_or("/"))
            .unwrap_or("/")
            .to_string(),
    };

    let login_state = LoginState { redirect_uri };

    let _: () = redis_conn
        .set("login_state", serde_json::to_string(&login_state).unwrap())
        .await
        .map_err(|_| ServiceError::InternalServerError("Failed to set login state".into()))?;

    //redirect to OpenIdProvider for authentication
    Ok(HttpResponse::SeeOther()
        .insert_header(("Location", auth_url.as_str()))
        .finish())
}

/// OpenID Connect callback
///
/// This is the callback route for the OAuth provider, it should not be called directly. Redirects to browser with set-cookie header.
#[utoipa::path(
    get,
    path = "/auth/callback",
    context_path = "/api",
    tag = "auth",
    params(
        OpCallback
    ),
    responses(
        (status = 303, description = "Response that returns with set-cookie header"),
        (status = 400, description = "Email or password empty or incorrect", body = ErrorRespPayload),
    )
)]
#[tracing::instrument(skip(redis_pool, oidc_client, pg_pool))]
pub async fn callback(
    req: HttpRequest,
    redis_pool: web::Data<RedisPool>,
    oidc_client: web::Data<CoreClient>,
    pg_pool: web::Data<PgPool>,
    query: web::Query<OpCallback>,
) -> Result<HttpResponse, Error> {
    let mut redis_conn = redis_pool
        .get()
        .await
        .map_err(|_| ServiceError::InternalServerError("Could not get redis connection".into()))?;

    let opt_state: Option<String> =
        redis_conn
            .get(OIDC_SESSION_KEY.to_string())
            .await
            .map_err(|_| {
                ServiceError::InternalServerError("Could not get OIDC session state".into())
            })?;

    let state: OpenIdConnectState = match opt_state {
        Some(state) => serde_json::from_str(&state).map_err(|_| {
            ServiceError::InternalServerError("Could not deserialize OIDC session state".into())
        })?,
        None => Err(ServiceError::Unauthorized)?,
    };

    let code_verifier = state.pkce_verifier;
    let code = query.code.clone();
    let nonce = state.nonce;

    let token_response = oidc_client
        .exchange_code(AuthorizationCode::new(code))
        .set_pkce_verifier(code_verifier)
        .request_async(async_http_client)
        .await
        .map_err(|e| match e {
            oauth2::RequestTokenError::ServerResponse(e) => {
                ServiceError::InternalServerError(e.to_string())
            }
            _ => ServiceError::InternalServerError("Unknown error".into()),
        })?;

    let id_token = token_response
        .extra_fields()
        .id_token()
        .ok_or_else(|| ServiceError::InternalServerError("Empty ID Token".into()))?;

    let id_token_verifier = oidc_client.id_token_verifier();
    let claims = id_token
        .claims(&id_token_verifier, &nonce)
        .map_err(|_| ServiceError::InternalServerError("Claims Verification Error".into()))?;

    match claims.access_token_hash() {
        None => Err(ServiceError::BadRequest(
            "Missing access token hash".to_string(),
        ))?,
        Some(given_token_hash) => {
            let calculated_token_hash = AccessTokenHash::from_token(
                token_response.access_token(),
                &id_token.signing_alg().map_err(|_| {
                    ServiceError::BadRequest("ID token hash unavailable".to_string())
                })?,
            )
            .map_err(|_| ServiceError::BadRequest("ID token hash unavailable".to_string()))?;

            if calculated_token_hash != *given_token_hash {
                Err(ServiceError::BadRequest(
                    "ID token hash invalid".to_string(),
                ))
            } else {
                Ok(())
            }
        }
    }?;

    let user_id = claims
        .subject()
        .to_string()
        .parse::<uuid::Uuid>()
        .map_err(|_| {
            ServiceError::InternalServerError("Failed to parse user ID from claims".into())
        })?;

    let email = claims.email().ok_or_else(|| {
        ServiceError::InternalServerError("Failed to parse email from claims".into())
    })?;

    let name = claims.name().ok_or_else(|| {
        ServiceError::InternalServerError("Failed to parse name from claims".into())
    })?;

    let opt_login_state: Option<String> = redis_conn
        .get("login_state")
        .await
        .map_err(|_| ServiceError::InternalServerError("Failed to get login state".into()))?;

    let login_state: LoginState = match opt_login_state {
        Some(login_state) => serde_json::from_str(&login_state).map_err(|_| {
            ServiceError::InternalServerError("Failed to deserialize login state".into())
        })?,
        None => Err(ServiceError::Unauthorized)?,
    };

    let user = match get_user_by_id_query(&user_id, pg_pool.clone()).await {
        Ok(user) => user,
        Err(_) => {
            create_account(
                email.to_string(),
                name.iter().next().unwrap().1.to_string(),
                user_id,
                pg_pool.clone(),
            )
            .await?
        }
    };

    let user_string = serde_json::to_string(&user).map_err(|_| {
        ServiceError::InternalServerError("Failed to serialize user to JSON".into())
    })?;

    Identity::login(&req.extensions(), user_string).expect("Failed to set login state for user");

    Ok(HttpResponse::SeeOther()
        .insert_header(("Location", login_state.redirect_uri))
        .finish())
}

/// Health Check
///
/// Confirmation that the service is healthy
#[utoipa::path(
    get,
    path = "/health",
    context_path = "/api",
    tag = "health",
    responses(
        (status = 200, description = "Confirmation that the service is healthy"),
        (status = 400, description = "Service error relating to overall service health", body = ErrorRespPayload),
    ),
)]
#[tracing::instrument]
pub async fn health_check() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().finish())
}

/// Local login page for cli
pub async fn login_cli() -> Result<HttpResponse, ServiceError> {
    let html_page = read_to_string("src/public/login.html").map_err(|e| {
        ServiceError::InternalServerError(format!("Could not read login page {}", e))
    })?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html_page))
}

/// Get Currently Auth'ed User
///
/// Get the currently auth'ed user. This will return the user object for the currently auth'ed user.
#[utoipa::path(
    get,
    path = "/auth/whoami",
    context_path = "/api",
    tag = "auth",
    responses(
        (status = 200, description = "JSON body containing the user object", body = User),
        (status = 400, description = "Service error relating to getting the currently auth'ed user", body = ErrorRespPayload),
    ),
    security(
        ("ApiKey" = ["readonly"]),
    )
)]
pub async fn whoami(user: AuthedUser) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().json(user))
}
