mod routes;
mod utils;
use crate::routes::create_link;
use crate::routes::health;
use crate::routes::redirect;
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use axum_prometheus::PrometheusMetricLayer;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::error::Error;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "link_shortener=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL is a required environment variable");

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await?;

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    let app: Router = Router::new()
        .route("/create", post(create_link))
        .route("/:id", get(redirect))
        .route("/metric", get(|| async move { metric_handle.render() }))
        .route("/health", get(health))
        .layer(TraceLayer::new_for_http())
        .layer(prometheus_layer)
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Could not initialize TcpListener");

    tracing::debug!(
        "listening on {}",
        listener
            .local_addr()
            .expect("Could no convert listener address to local address")
    );

    axum::serve(listener, app)
        .await
        .expect("Could not successfully create server");
    Ok(())
}
