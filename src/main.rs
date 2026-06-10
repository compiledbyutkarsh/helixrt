mod core;
mod runtime;
mod network;
mod simulation;
mod robot;
mod navigation;
mod sensors;
mod utils;
mod state;
mod bus;
mod tasks;
mod telemetry;
mod ws;
mod autonomy;
mod logging;
mod fusion;
mod persistence;

use runtime::engine::RuntimeEngine;

#[tokio::main]
async fn main() {
    let mut runtime =
        RuntimeEngine::new();

    runtime.boot().await;
}