use std::str::FromStr;

use axum::{routing::get, Router};
use dotenv::dotenv;
use envload::{Envload, LoadEnv};
use plaid::{
    model::LinkTokenCreateRequestUser, request::LinkTokenCreateRequest, FluentRequest, PlaidClient,
};
use tower_http::services::{ServeDir, ServeFile};

#[derive(Envload)]
struct Environment {
    plaid_client_id: String,
    plaid_secret: String,
    plaid_env: Option<String>,
    plaid_products: Option<String>,
    plaid_country_codes: Option<String>,
    plaid_redirect_uri: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let environment = <Environment as LoadEnv>::load_env();
    let client = PlaidClient::from_env();
    let static_dir = ServeDir::new("static");
    let index = ServeFile::new("static/index.html");

    let app = Router::new()
        .route("/api/create_link_token", get(link_token()))
        .fallback_service(static_dir.fallback(index));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("did not establish listener");

    axum::serve(listener, app)
        .await
        .expect("could not run server");
}
async fn link_token() {
    let client = PlaidClient::from_env();
    let response = client
        .link_token_create(plaid::request::LinkTokenCreateRequired {
            client_name: "Plaid Test App",
            country_codes: &["US"],
            language: "en",
            user: LinkTokenCreateRequestUser {
                address: None,
                client_user_id: String::from_str("1337").unwrap(),
                date_of_birth: None,
                email_address: None,
                email_address_verified_time: None,
                id_number: None,
                legal_name: None,
                name: None,
                phone_number: None,
                phone_number_verified_time: None,
                ssn: None,
            },
        })
        .await
        .unwrap();
}
