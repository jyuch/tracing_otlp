mod otl;

use crate::otl::init_tracing;
use tracing::{error, info, instrument};

#[instrument]
async fn start(x: i32, y: i32) -> Option<i32> {
    add(multiply(x, y).await, multiply(x, y).await).await
}

#[instrument]
async fn add(x: i32, y: i32) -> Option<i32> {
    let ans = x + y;

    if ans <= 10 {
        info!(ans = ans, "特に出すべきログがないからとりあえず適当なメッセージを出しています");
        Some(ans)
    } else {
        error!(ans = ans, "something went wrong");
        None
    }
}

#[instrument]
async fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

#[tokio::main]
async fn main() {
    let service = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");

    init_tracing(service, version);

    let value = start(1, 2).await;
    println!("{:?}", value);

    let value = start(10, 22).await;
    println!("{:?}", value);

    opentelemetry::global::shutdown_tracer_provider();
}
