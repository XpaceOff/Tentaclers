//use std::collections::HashMap;
use axum::{extract::Path, routing::get, Router};
use tentaclers_lib::{checker, errors::AppErrorText};
use tracing::info;
use tracing_subscriber::filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ---- Setup the logs ----
    let log_level = filter::LevelFilter::TRACE; //filter::LevelFilter::OFF;
    let log_subscriber = tracing_subscriber::fmt().with_max_level(log_level).finish();

    if let Err(err_msg) = tracing::subscriber::set_global_default(log_subscriber) {
        println!("Warning: {}", err_msg);
    }

    // ---- Make sure the app has what it needs ----
    // ---- Check process ----
    checker::check_core_dirs(true)?;

    // ---- Create the Router ----
    let app = Router::new()
        .route("/:version/*path", get(gate_get))
        .route("/", get(test_fn));

    info!("router created");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?; //.unwrap();

    Ok(())
}

async fn gate_get(Path((version, api_path)): Path<(String, String)>) -> String {
    format!(" version: {}, \n Path: {}", version, api_path)
}

async fn test_fn() -> AppErrorText<'static> {
    AppErrorText::NotFound(Some("index"))
}
