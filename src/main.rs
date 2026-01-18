mod config;
mod handler;
mod metrics;
mod switchbot;

use crate::config::Config;
use crate::handler::metrics_route;
use crate::metrics::Metrics;
use crate::switchbot::fetch::run_switchbot_fetcher;

// [`tracing`] is a framework for instrumenting Rust programs to
// collect scoped, structured, and async-aware diagnostics. This example
// demonstrates how the `warp::trace` module can be used to instrument `warp`
// applications with `tracing`.
//
// [`tracing`]: https://crates.io/crates/tracing
#[deny(warnings)]
use tracing_subscriber::fmt::format::FmtSpan;

use warp::Filter;

#[tokio::main]
async fn main() {
    // Filter traces based on the RUST_LOG env var, or, if it's not set,
    // default to show the output of the example.
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "info,warp=debug".to_owned());

    // Configure the default `tracing` subscriber.
    // The `fmt` subscriber from the `tracing-subscriber` crate logs `tracing`
    // events to stdout. Other subscribers are available for integrating with
    // distributed tracing systems such as OpenTelemetry.
    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(filter)
        // Record an event when each span closes. This can be used to time our
        // routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let metrics = Metrics::new();
    let config = Config::new();

    // バックグラウンド起動
    tokio::spawn(run_switchbot_fetcher(metrics.clone(), config.clone()));

    let routes = metrics_route(metrics).with(warp::trace::request());

    warp::serve(routes).run(([0, 0, 0, 0], config.port)).await;
}
