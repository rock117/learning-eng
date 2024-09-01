mod subtitle;
mod upload_subtitles;
mod search_engine;

use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::net::SocketAddr;
use std::ops::Index;
use std::time::Duration;
use crate::subtitle::ass;
use crate::subtitle::srt::Srt;
use crate::subtitle::Subtitle;

use anyhow::{anyhow, Context};
use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::BoxError;
use axum::{routing::get, Router};

use http::header::HeaderName;
use tower::{timeout::TimeoutLayer, ServiceBuilder};
use tower_http::propagate_header::PropagateHeaderLayer;
use tower_http::request_id::SetRequestIdLayer;
use tower_http::trace::TraceLayer;
use tracing::level_filters::LevelFilter;
use tracing::{info, warn};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, EnvFilter, Registry};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_log_context()?;
    // schedule::start_schedule();
    start().await
}

fn init_log_context() -> anyhow::Result<()> {
    // TODO
    let file_appender = tracing_appender::rolling::daily(
        "C:/rock/coding/code/my/rust/programmer-investment-research/api/tmp",
        "app.log",
    );
    let filter = EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into());
    let subscriber = Registry::default()
        .with(
            fmt::layer()
                .compact()
                .with_ansi(true)
                .with_file(false)
                .with_line_number(false)
                .with_thread_ids(false)
                .with_target(false)
            // .with_span_events(FmtSpan::CLOSE),
        )
        .with(
            fmt::layer()
                .with_ansi(false)
                .with_writer(file_appender)
                .with_file(false)
                .with_line_number(false)
                .with_thread_ids(false)
                .with_target(false)
            //  .with_span_events(FmtSpan::CLOSE),
        )
        .with(filter);
    tracing::subscriber::set_global_default(subscriber).map_err(|e| anyhow!(e))?;
    Ok(())
}

async fn start() -> anyhow::Result<()> {
    let x_request_id = HeaderName::from_static("x-request-id");
    let layers = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: BoxError| async {
            StatusCode::REQUEST_TIMEOUT
        }))
        //.layer(PropagateHeaderLayer::new(x_request_id))
        .layer(TimeoutLayer::new(Duration::from_secs(
            30,
        )));

    let app = register_routes().layer(layers);
    let address = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum_server::bind(address)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

fn register_routes() -> Router {
    Router::new()
        //  .typed_route(controller::tmp::item_handler)
        .route("/", get(root))
          // .on_response(|response: &Response, latency: Duration, _: &'_ _| {})
}


async fn root() -> String {
    return "Welcome!".to_string();
}
