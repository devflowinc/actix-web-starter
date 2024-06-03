#![allow(clippy::get_first)]

#[macro_use]
extern crate diesel;
use crate::{
    errors::ServiceError, handlers::auth_handler::build_oidc_client, middleware::auth_middleware,
};
use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{config::PersistentSession, storage::RedisSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    middleware::Logger,
    web::{self, PayloadConfig},
    App, HttpServer,
};
use data::models;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::ManagerConfig;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use futures_util::future::BoxFuture;
use futures_util::FutureExt;
use openssl::ssl::SslVerifyMode;
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use tracing_subscriber::{prelude::*, EnvFilter, Layer};
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

pub mod data;
pub mod errors;
pub mod handlers;
pub mod middleware;
pub mod operators;
pub mod prefixes;

pub const SECONDS_IN_MINUTE: u64 = 60;
pub const SECONDS_IN_HOUR: u64 = 60 * SECONDS_IN_MINUTE;
pub const SECONDS_IN_DAY: u64 = 24 * SECONDS_IN_HOUR;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

fn run_migrations(url: &str) {
    use diesel::prelude::*;
    let mut conn = diesel::pg::PgConnection::establish(url).expect("Failed to connect to database");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
}

pub fn establish_connection(
    config: &str,
) -> BoxFuture<diesel::ConnectionResult<diesel_async::AsyncPgConnection>> {
    let fut = async {
        let mut tls = SslConnector::builder(SslMethod::tls()).unwrap();

        tls.set_verify(SslVerifyMode::NONE);
        let tls_connector = MakeTlsConnector::new(tls.build());

        let (client, conn) = tokio_postgres::connect(config, tls_connector)
            .await
            .map_err(|e| diesel::ConnectionError::BadConnection(e.to_string()))?;

        tokio::spawn(async move {
            if let Err(e) = conn.await {
                eprintln!("Database connection: {e}");
            }
        });
        diesel_async::AsyncPgConnection::try_from(client).await
    };
    fut.boxed()
}

use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi
            .components
            .as_mut()
            .expect("Safe because the component has already been registered at this point");
        components.add_security_scheme(
            "ApiKey",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
        );
    }
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Trieve API",
        description = "Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.", 
        contact(
            name = "Trieve Team",
            url = "https://trieve.ai",
            email = "developers@trieve.ai",
        ),
        license(
            name = "MIT",
            url = "https://github.com/devflowinc/actix-web-template/blob/main/LICENSE.txt",
        ),
        version = "0.0.1",
    ),
    servers(
        (url = "http://localhost:8090",
        description = "Local development server"),
    ),
    modifiers(&SecurityAddon),
    paths(
        handlers::auth_handler::login,
        handlers::auth_handler::logout,
        handlers::auth_handler::whoami,
        handlers::auth_handler::callback,
        handlers::api_key_handler::create_api_key,
        handlers::auth_handler::health_check,
        handlers::org_handler::create_org,
        handlers::org_handler::delete_org,
        handlers::org_handler::update_org,
        handlers::org_handler::get_orgs_for_authed_user,
        handlers::org_handler::leave_org,
        handlers::invitation_handler::post_invitation,
        handlers::invitation_handler::get_invitations,
        handlers::invitation_handler::delete_invitation,
        handlers::deal_handler::create_deal,
        handlers::deal_handler::delete_deal,
        handlers::deal_handler::update_deal,
        handlers::deal_handler::get_deal,
        handlers::contact_handler::create_contact,
        handlers::contact_handler::delete_contact,
        handlers::contact_handler::update_contact,
        handlers::contact_handler::get_contact,
    ),
    components(
        schemas(
            handlers::api_key_handler::CreateApiKeyRespPayload,
            handlers::api_key_handler::CreateApiKeyReqPayload,
            handlers::api_key_handler::CreateApiKeyReqPayload,
            handlers::org_handler::CreateOrgReqPayload,
            handlers::org_handler::UpdateOrgReqPayload,
            handlers::deal_handler::CreateDealReqPayload,
            handlers::deal_handler::UpdateDealReqPayload,
            handlers::contact_handler::CreateContactReqPayload,
            handlers::contact_handler::UpdateContactReqPayload,
            handlers::invitation_handler::InvitationResponse,
            handlers::invitation_handler::InvitationData,
            models::User,
            models::Invitation,
            models::Org,
            models::Note,
            errors::ErrorRespPayload,
            prefixes::PrefixedUuid<prefixes::OrgPrefix>,
            prefixes::PrefixedUuid<prefixes::OrgUserPrefix>,
            prefixes::PrefixedUuid<prefixes::UserPrefix>,
        )
    ),
    tags(
        (name = "auth", description = "Authentication endpoints. Used to authenticate users."),
        (name = "invitation", description = "Invitation endpoint. Exists to invite users to an organization."),
        (name = "orgs", description = "Organization endpoints. Used to manage organizations"),
        (name = "deals", description = "Deal endpoints. Used to manage deals"),
        (name = "api_key", description = "API Key endpoints. Used to manage user API keys."),
        (name = "health", description = "Health check endpoint. Used to check if the server is up and running."),
    ),
)]
pub struct ApiDoc;

#[tracing::instrument]
pub fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let sentry_url = std::env::var("SENTRY_URL");
    let _guard = if let Ok(sentry_url) = sentry_url {
        log::info!("Sentry monitoring enabled");

        let guard = sentry::init((
            sentry_url,
            sentry::ClientOptions {
                release: sentry::release_name!(),
                traces_sample_rate: 1.0,
                ..Default::default()
            },
        ));

        tracing_subscriber::Registry::default()
            .with(sentry::integrations::tracing::layer())
            .with(
                tracing_subscriber::fmt::layer().with_filter(
                    EnvFilter::from_default_env()
                        .add_directive(tracing_subscriber::filter::LevelFilter::INFO.into()),
                ),
            )
            .init();

        std::env::set_var("RUST_BACKTRACE", "1");
        Some(guard)
    } else {
        tracing_subscriber::Registry::default()
            .with(
                tracing_subscriber::fmt::layer().with_filter(
                    EnvFilter::from_default_env()
                        .add_directive(tracing_subscriber::filter::LevelFilter::INFO.into()),
                ),
            )
            .init();

        None
    };

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL should be set");

    run_migrations(&database_url);

    actix_web::rt::System::new().block_on(async move {
        // create db connection pool
        let mut config = ManagerConfig::default();
        config.custom_setup = Box::new(establish_connection);

        let mgr = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new_with_config(
            database_url,
            config,
        );

        let pg_pool = diesel_async::pooled_connection::deadpool::Pool::builder(mgr)
            .max_size(10)
            .build()
            .unwrap();

        let redis_store = RedisSessionStore::new(redis_url.clone())
            .await
            .expect("Failed to create redis store");

        let redis_manager =
            bb8_redis::RedisConnectionManager::new(redis_url).expect("Failed to connect to redis");

        let redis_connections: u32 = std::env::var("REDIS_CONNECTIONS")
            .unwrap_or("200".to_string())
            .parse()
            .unwrap_or(200);

        let redis_pool = bb8_redis::bb8::Pool::builder()
            .max_size(redis_connections)
            .build(redis_manager)
            .await
            .expect("Failed to create redis pool");

        let oidc_client = build_oidc_client().await;

        println!("{:?}", std::env::var("SECRET_KEY"));

        HttpServer::new(move || {
            App::new()
                .app_data(PayloadConfig::new(134200000))
                .app_data(
                    web::JsonConfig::default()
                        .limit(134200000)
                        .error_handler(|err, _req| {
                            ServiceError::BadRequest(format!("{}", err)).into()
                        }),
                )
                .app_data(
                    web::PathConfig::default().error_handler(|err, _req| {
                        ServiceError::BadRequest(format!("{}", err)).into()
                    }),
                )
                .app_data(web::Data::new(pg_pool.clone()))
                .app_data(web::Data::new(oidc_client.clone()))
                .app_data(web::Data::new(redis_pool.clone()))
                .wrap(sentry_actix::Sentry::new())
                .wrap(auth_middleware::AuthMiddlewareFactory)
                .wrap(
                    IdentityMiddleware::builder()
                        .login_deadline(Some(std::time::Duration::from_secs(SECONDS_IN_DAY)))
                        .visit_deadline(Some(std::time::Duration::from_secs(SECONDS_IN_DAY)))
                        .build(),
                )
                .wrap(Cors::permissive())
                .wrap(
                    SessionMiddleware::builder(
                        redis_store.clone(),
                        Key::from(
                            std::env::var("SECRET_KEY")
                                .unwrap_or_else(|_| "0123".repeat(16))
                                .as_bytes(),
                        ),
                    )
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(time::Duration::days(1)),
                    )
                    .cookie_name("actix-server".to_owned())
                    .cookie_same_site(SameSite::Lax)
                    .cookie_secure(false)
                    .cookie_path("/".to_owned())
                    .build(),
                )
                .wrap(Logger::default())
                .service(Redoc::with_url("/redoc", ApiDoc::openapi()))
                .service(
                    SwaggerUi::new("/swagger-ui/{_:.*}")
                        .url("/api-docs/openapi.json", ApiDoc::openapi()),
                )
                .service(web::redirect("/swagger-ui", "/swagger-ui/"))
                .service(
                    web::resource("/auth/cli")
                        .route(web::get().to(handlers::auth_handler::login_cli)),
                )
                .service(
                    web::scope("/api")
                        .service(
                            web::scope("/orgs")
                                .service(
                                    web::resource("")
                                        .route(web::post().to(handlers::org_handler::create_org))
                                        .route(
                                            web::get().to(
                                                handlers::org_handler::get_orgs_for_authed_user,
                                            ),
                                        ),
                                )
                                .service(
                                    web::resource("/leave/{org_id}")
                                        .route(web::delete().to(handlers::org_handler::leave_org)),
                                )
                                .service(
                                    web::resource("/{org_id}")
                                        .route(web::delete().to(handlers::org_handler::delete_org))
                                        .route(web::get().to(handlers::org_handler::get_org))
                                        .route(web::put().to(handlers::org_handler::update_org)),
                                ),
                        )
                        .service(
                            web::scope("/invitation")
                                .service(web::resource("").route(
                                    web::post().to(handlers::invitation_handler::post_invitation),
                                ))
                                .service(
                                    web::resource("/{invitation_id}").route(
                                        web::delete()
                                            .to(handlers::invitation_handler::delete_invitation),
                                    ),
                                )
                                .service(web::resource("/{organization_id}").route(
                                    web::get().to(handlers::invitation_handler::get_invitations),
                                )),
                        )
                        .service(
                            web::scope("/auth")
                                .service(
                                    web::resource("")
                                        .route(web::get().to(handlers::auth_handler::login))
                                        .route(web::delete().to(handlers::auth_handler::logout)),
                                )
                                .service(
                                    web::resource("/whoami")
                                        .route(web::get().to(handlers::auth_handler::whoami)),
                                )
                                .service(
                                    web::resource("/callback")
                                        .route(web::get().to(handlers::auth_handler::callback)),
                                ),
                        )
                        .service(
                            web::scope("/api_key").service(
                                web::resource("").route(
                                    web::post().to(handlers::api_key_handler::create_api_key),
                                ),
                            ),
                        )
                        .service(
                            web::scope("/deals")
                                .service(
                                    web::resource("")
                                        .route(web::post().to(handlers::deal_handler::create_deal)),
                                )
                                .service(
                                    web::resource("/{deal_id}")
                                        .route(
                                            web::delete().to(handlers::deal_handler::delete_deal),
                                        )
                                        .route(web::get().to(handlers::deal_handler::get_deal))
                                        .route(web::put().to(handlers::deal_handler::update_deal)),
                                ),
                        )
                        .service(
                            web::scope("/contacts")
                                .service(web::resource("").route(
                                    web::post().to(handlers::contact_handler::create_contact),
                                ))
                                .service(
                                    web::resource("/{contact_id}")
                                        .route(
                                            web::delete()
                                                .to(handlers::contact_handler::delete_contact),
                                        )
                                        .route(
                                            web::get().to(handlers::contact_handler::get_contact),
                                        )
                                        .route(
                                            web::put()
                                                .to(handlers::contact_handler::update_contact),
                                        ),
                                ),
                        )
                        .service(
                            web::resource("/health")
                                .route(web::get().to(handlers::auth_handler::health_check)),
                        ),
                )
        })
        .bind(("0.0.0.0", 8090))?
        .run()
        .await
    })?;

    Ok(())
}
