use tokio::net::TcpListener;
use crate::app::build_app;
use crate::shared::configuration::AppConfig;
use crate::shared::logging::log;

mod shared;
mod services;
mod app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
// async fn main() {

    // Initialize tracing
    // tracing_subscriber::fmt::init();
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();

    // Load configuration
    let config = AppConfig::default()?;

    log::init_from_config(config.is_prod);

    let app = build_app(config).await?;

    tracing::info!("Server running on http://{}", app.addr);
    log::info2(&format!("Server running on http://{}", app.addr));
    tracing::info!("Swagger UI available at http://{}/swagger-ui", app.addr);
    log::info2(&format!("Swagger UI available at http://{}/swagger-ui", app.addr));
    tracing::info!("Metrics available at http://{}/metrics", app.addr);
    log::info2(&format!("Metrics available at http://{}/metrics", app.addr));

    /*axum::Server::bind(&app.addr)
        .serve(app.router.into_make_service())
        .await?;
    */

    let listener = TcpListener::bind(app.addr).await?;
    axum::serve(listener, app.router).await?;

    Ok(())
}
