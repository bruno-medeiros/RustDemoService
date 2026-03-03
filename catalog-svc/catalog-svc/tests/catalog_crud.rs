//! Integration tests for the catalog API: real server instance and catalog-svc-client.

use std::time::Duration;

use catalog_svc::config::AppConfig;
use catalog_svc::server;
use catalog_svc_client::types::{Category, CreateCatalogItemBody, UpdateCatalogItemBody};
use catalog_svc_client::Client;
use rust_demo_commons::util::tests;

#[tokio::test]
async fn catalog_crud() {
    tests::init_logging();

    let mut app_config = AppConfig::load_tests();
    let app_state = server::build_app(&app_config).await;
    app_config.server.port = 3031;

    let (state, _handle, _addr) = server::start_service_and_serve(app_state, app_config)
        .await
        .expect("bind");
    tokio::time::sleep(Duration::from_millis(100)).await;

    let base = format!("http://127.0.0.1:{}", 3031);
    let client = Client::new(&base);

    // Create
    let create_body = CreateCatalogItemBody {
        name: "Rust Book".to_string(),
        description: "Learn Rust".to_string(),
        category: Category::Books,
        date: "2025-03-01".to_string(),
        brand: Some("O'Reilly".to_string()),
        price: "49.99".to_string(),
    };
    let created = client
        .create_catalog_item(&create_body)
        .await
        .expect("create should succeed");
    let item = created.into_inner();
    let item_id = item.item_id;

    assert_eq!(item.name, "Rust Book");
    assert_eq!(item.description, "Learn Rust");
    assert_eq!(item.category, Category::Books);
    assert_eq!(item.price, "49.99");
    assert_eq!(item.brand.as_deref(), Some("O'Reilly"));

    // List (verify create persisted)
    let list = client
        .list_catalog_items(None, None)
        .await
        .expect("list should succeed");
    let list_body = list.into_inner();
    assert!(!list_body.items.is_empty());
    assert!(list_body.items.iter().any(|i| i.item_id == item_id));

    // Get
    let got = client
        .get_catalog_item(&item_id)
        .await
        .expect("get should succeed");
    let got_item = got.into_inner();
    assert_eq!(got_item.item_id, item_id);
    assert_eq!(got_item.name, "Rust Book");

    // Update
    let update_body = UpdateCatalogItemBody {
        name: "Rust Book (2nd ed)".to_string(),
        description: "Learn Rust, updated".to_string(),
        category: Category::Books,
        date: "2025-03-01".to_string(),
        brand: Some("O'Reilly".to_string()),
        price: "54.99".to_string(),
    };
    let updated = client
        .update_catalog_item(&item_id, &update_body)
        .await
        .expect("update should succeed");
    let updated_item = updated.into_inner();
    assert_eq!(updated_item.name, "Rust Book (2nd ed)");
    assert_eq!(updated_item.price, "54.99");

    // Delete
    client
        .delete_catalog_item(&item_id)
        .await
        .expect("delete should succeed");

    // Get after delete -> 404
    let get_after = client.get_catalog_item(&item_id).await;
    assert!(get_after.is_err());

    state.server_shutdown.cancel();
}
