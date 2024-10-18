use crate::utils::database::connect_to_db;

#[tokio::test]
async fn connect_test() {
    assert!(connect_to_db().await.is_ok())
}
