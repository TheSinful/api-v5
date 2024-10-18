use crate::utils::password::{hash_password, verify_password};

#[test]
fn test_verify_password() {
    let base = "test_pass_1".to_string();
    let hashed = hash_password(base.clone());

    assert!(verify_password(&hashed, &base.clone()))
}
