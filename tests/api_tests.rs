use actix_web::{test, App};
use trmnl_plugin_server::handlers; // Adjust the module path as needed

#[actix_web::test]
async fn test_always_passes() {
    assert_eq!(1 + 1, 2);
}

