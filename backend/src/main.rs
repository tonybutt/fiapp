use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use dotenv::dotenv;
// use envload::{Envload, LoadEnv};
use plaid::{
    model::{LinkTokenCreateRequestUser, LinkTokenCreateResponse},
    PlaidClient,
};
use serde::Deserialize;
// use serde::Serialize;
// use std::str::FromStr;
use std::sync::Arc;
use tower_http::services::{ServeDir, ServeFile};

// #[derive(Envload)]
// struct Environment {
//     plaid_client_id: String,
//     plaid_secret: String,
//     plaid_env: Option<String>,
//     plaid_products: Option<String>,
//     plaid_country_codes: Option<String>,
//     plaid_redirect_uri: String,
// }

#[derive(Clone)]
struct AppState {
    plaid_client: Arc<PlaidClient>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    // let environment = <Environment as LoadEnv>::load_env();
    let static_dir = ServeDir::new("static");
    let index = ServeFile::new("static/index.html");
    let state = AppState {
        plaid_client: Arc::new(PlaidClient::from_env()),
    };
    let app = Router::new()
        .route("/api/create_link_token", post(link_token))
        .route("/api/set_access_token", post(set_token))
        .fallback_service(static_dir.fallback(index))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("did not establish listener");

    axum::serve(listener, app)
        .await
        .expect("could not run server");
}

#[derive(Deserialize)]
struct SetTokenRequest {
    public_token: String,
}
async fn set_token(
    State(AppState { plaid_client }): State<AppState>,
    Json(request): Json<SetTokenRequest>,
) -> Result<(StatusCode, String), StatusCode> {
    let client = plaid_client.clone();
    match client
        .item_public_token_exchange(request.public_token.as_str())
        .await
    {
        Ok(_) => Ok((StatusCode::OK, "Successfully exchanged token!".to_string())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
async fn link_token(
    State(AppState { plaid_client }): State<AppState>,
) -> Result<Json<LinkTokenCreateResponse>, StatusCode> {
    let client = plaid_client.clone();
    let request = plaid::request::LinkTokenCreateRequired {
        client_name: "Plaid Test App",
        country_codes: &["US"],
        language: "en",
        user: LinkTokenCreateRequestUser {
            client_user_id: "1337".to_string(),
            ..Default::default()
        },
    };

    match client
        .link_token_create(request)
        .products(vec!["auth", "transactions"])
        .await
    {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            eprintln!("error creating link token {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
