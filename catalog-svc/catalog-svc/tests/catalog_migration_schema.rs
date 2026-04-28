#[test]
fn catalog_item_timestamps_remain_timestamptz() {
    let migration = include_str!("../migrations/20250303120000_create_catalog_items.sql");

    assert!(migration.contains("created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()"));
    assert!(migration.contains("modified_at TIMESTAMPTZ NOT NULL DEFAULT NOW()"));
}
