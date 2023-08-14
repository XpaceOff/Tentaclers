mod checker;
mod errors;

//use std::collections::HashMap;
use axum::{extract::Path, routing::get, Router};
use errors::AppErrorText;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/:version/*path", get(gate_get))
        .route("/", get(test_fn));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn gate_get(Path((version, api_path)): Path<(String, String)>) -> String {
    format!(" version: {}, \n Path: {}", version, api_path)
}

async fn test_fn() -> errors::AppErrorText<'static> {
    AppErrorText::NotFound(Some("index"))
}
