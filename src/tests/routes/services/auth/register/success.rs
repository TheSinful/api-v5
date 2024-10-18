use crate::routes::services::auth::register::register;
use crate::utils::database::connect_to_db;
use actix_web::{test, App};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde_json::json;

#[actix_rt::test]
async fn test_register_success() {
    let mut app = test::init_service(App::new().service(register)).await;
    assert!(connect_to_db().await.is_ok());

    let random_username: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    let req_body = json!({
        "username": random_username,
        "password": "password123",
    });

    let req = test::TestRequest::post()
        .uri("/register")
        .set_json(&req_body)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
}