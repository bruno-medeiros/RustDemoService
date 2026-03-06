-- Catalog items table for CRUD operations.
-- category stored as text: 'Books' | 'Electronics'
-- date is date-only (YYYY-MM-DD); created_at/modified_at are UTC timestamps without timezone metadata.
-- price is fixed-point decimal (e.g. 99,999,999.99).
CREATE TABLE catalog_items (
    item_id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    category VARCHAR(32) NOT NULL,
    date DATE NOT NULL,
    brand VARCHAR(255),
    price NUMERIC(10, 2) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    modified_at TIMESTAMP NOT NULL
);

CREATE INDEX idx_catalog_items_created_at ON catalog_items (created_at);
