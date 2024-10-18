use crate::routes::services::auth::login::login;
use crate::utils::database::connect_to_db;
use actix_web::{test, App};
use serde_json::json;

#[actix_rt::test]
async fn test_login_invalid_pass() {
    let mut app = test::init_service(App::new().service(login)).await;
    assert!(connect_to_db().await.is_ok());

    let req_body = json!({
        "username": "testuser",
        "password": "password", // original password123
    });

    let req = test::TestRequest::get()
        .uri("/login")
        .set_json(&req_body)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::UNAUTHORIZED)
}
