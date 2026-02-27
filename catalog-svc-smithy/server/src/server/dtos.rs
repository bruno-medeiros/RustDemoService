use catalog_api::model::CatalogItem;
use catalog_api::output;

/// Builds CreateCatalogItemOutput by moving fields from an owned CatalogItem.
pub fn shape_to_create_output(item: CatalogItem) -> output::CreateCatalogItemOutput {
    output::CreateCatalogItemOutput {
        name: item.name,
        description: item.description,
        category: item.category,
        date: item.date,
        brand: item.brand,
        price: item.price,
        item_id: item.item_id,
        created_at: item.created_at,
        modified_at: item.modified_at,
    }
}

/// Builds GetCatalogItemOutput by moving fields from an owned CatalogItem.
pub fn shape_to_get_output(item: CatalogItem) -> output::GetCatalogItemOutput {
    output::GetCatalogItemOutput {
        name: item.name,
        description: item.description,
        category: item.category,
        date: item.date,
        brand: item.brand,
        price: item.price,
        item_id: item.item_id,
        created_at: item.created_at,
        modified_at: item.modified_at,
    }
}

/// Builds UpdateCatalogItemOutput by moving fields from an owned CatalogItem.
pub fn shape_to_update_output(item: CatalogItem) -> output::UpdateCatalogItemOutput {
    output::UpdateCatalogItemOutput {
        name: item.name,
        description: item.description,
        category: item.category,
        date: item.date,
        brand: item.brand,
        price: item.price,
        item_id: item.item_id,
        created_at: item.created_at,
        modified_at: item.modified_at,
    }
}
