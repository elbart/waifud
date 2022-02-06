#[macro_use]
extern crate tracing;

use axum::{
    routing::{get, post},
    AddExtensionLayer, Router,
};
use std::sync::Arc;
use tower::limit::ConcurrencyLimitLayer;
use tower_http::trace::TraceLayer;
use waifud::{Result, State};

#[tokio::main]
async fn main() -> Result {
    tracing_subscriber::fmt::init();

    waifud::migrate::run()?;

    let middleware = tower::ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(ConcurrencyLimitLayer::new(64))
        .layer(AddExtensionLayer::new(Arc::new(State::new()?)));

    // build our application with a route
    let app = Router::new()
        .route("/api/v1/distros", get(waifud::api::distros::get_distros))
        .route("/api/v1/instances", post(waifud::api::instances::make))
        .route("/api/v1/instances", get(waifud::api::instances::list))
        .route("/api/v1/instances/:id", get(waifud::api::instances::get))
        .route(
            "/api/v1/instances/:id/machine",
            get(waifud::api::instances::get_machine),
        )
        .route(
            "/api/v1/libvirt/machines",
            get(waifud::api::libvirt::get_machines),
        )
        .route(
            "/api/cloudinit/:id/meta-data",
            get(waifud::api::cloudinit::meta_data),
        )
        .route(
            "/api/cloudinit/:id/user-data",
            get(waifud::api::cloudinit::user_data),
        )
        .route(
            "/api/cloudinit/:id/vendor-data",
            get(waifud::api::cloudinit::vendor_data),
        )
        .layer(middleware);

    // run it
    let addr = &"[::]:23818".parse()?;
    info!("listening on {}", addr);
    axum::Server::bind(addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}