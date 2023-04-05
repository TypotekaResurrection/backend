mod db;
mod graphql;
mod utils;

use std::net::SocketAddr;
use entity::async_graphql;
use tower_http::cors::{Any, Cors, CorsLayer};

use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
    Router,
    routing::get,
};

use graphql::schema::{build_schema, AppSchema};
use once_cell::sync::Lazy;
use async_graphql::Request;
use crate::utils::auth::Token;

use dotenv::dotenv;
use reqwest::Method;

async fn graphql_handler(user: Token, schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner().data(user)).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        "/api/graphql",
    )))
}

static KEYS: Lazy<utils::jwt::Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "1234".to_owned());
    utils::jwt::Keys::new(secret.as_bytes())
});

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv().ok();

    let schema = build_schema().await;

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app = Router::new()
        .route(
            "/api/graphql",
            get(graphql_playground).post(graphql_handler),
        )
        .layer(cors)
        .layer(Extension(schema));

    println!("Playground: http://localhost:3000/api/graphql");

    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);

    let address = SocketAddr::from(([0, 0, 0, 0], port));

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}