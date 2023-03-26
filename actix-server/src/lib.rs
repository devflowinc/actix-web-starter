#![allow(clippy::get_first)]

#[macro_use]
extern crate diesel;
use crate::{
    errors::ServiceError,
    handlers::auth_handler::build_oidc_client
};
use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{config::PersistentSession, storage::RedisSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    middleware,
    web::{self, PayloadConfig},
    App, HttpServer,
};
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

pub mod middleware;
pub mod data;
pub mod errors;
pub mod handlers;
pub mod operators;

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
            url = "https://github.com/devflowinc/trieve/blob/main/LICENSE.txt",
        ),
        version = "0.0.1",
    ),
    servers(
        (url = "https://api.trieve.ai",
        description = "Production server"),
        (url = "http://localhost:8090",
        description = "Local development server"),
    ),
    modifiers(&SecurityAddon),
    paths(
        handlers::auth_handler::health_check,
    ),
    components(
        schemas(
            errors::ErrorResponseBody,
        )
    ),
    tags(
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

    let database_url = get_env!("DATABASE_URL", "DATABASE_URL should be set");
    let redis_url = get_env!("REDIS_URL", "REDIS_URL should be set");

    run_migrations(database_url);

    actix_web::rt::System::new().block_on(async move {
        // create db connection pool
        let mut config = ManagerConfig::default();
        config.custom_setup = Box::new(establish_connection);

        let mgr = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new_with_config(
            database_url,
            config,
        );

        let pool = diesel_async::pooled_connection::deadpool::Pool::builder(mgr)
            .max_size(10)
            .build()
            .unwrap();

        let oidc_client = build_oidc_client().await;

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
                .app_data(web::Data::new(pool.clone()))
                .app_data(web::Data::new(oidc_client.clone()))
                .wrap(sentry_actix::Sentry::with_transaction())
                .wrap(af_middleware::auth_middleware::AuthMiddlewareFactory)
                .wrap(
                    IdentityMiddleware::builder()
                        .login_deadline(Some(std::time::Duration::from_secs(SECONDS_IN_DAY)))
                        .visit_deadline(Some(std::time::Duration::from_secs(SECONDS_IN_DAY)))
                        .build(),
                )
                .wrap(Cors::permissive())
                .wrap(middleware::Logger::default())
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
                    web::scope("/api").service(
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
