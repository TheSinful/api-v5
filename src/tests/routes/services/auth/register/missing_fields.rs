use crate::routes::services::auth::register::register;
use crate::utils::database::connect_to_db;
use actix_web::{test, App};
use serde_json::json;

#[actix_rt::test]
async fn test_register_missing_fields() {
    let mut app = test::init_service(App::new().service(register)).await;
    assert!(connect_to_db().await.is_ok());

    let req_body = json!({
        "username": "testuser",
        "serials": {
            "motherboard": "mb123",
            "disk": "disk123",
            "cpu": "cpu123"
        }
    });

    let req = test::TestRequest::post()
        .uri("/register")
        .set_json(&req_body)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);
}
