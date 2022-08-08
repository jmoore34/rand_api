use rand_api::evaluate;
use axum::{
    routing::get,
    Router, extract::Path,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/:query", get(root));

    axum::Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root(Path(query): Path<String>) -> String {
    evaluate(&query)
}