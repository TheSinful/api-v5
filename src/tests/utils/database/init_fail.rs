use crate::utils::database::get_client;

#[test]
fn test_init_fail() {
    assert!(get_client().is_err())
}
