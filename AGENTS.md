# Agent guidance

## Smithy vs service types (Rust)

When integrating Smithy-generated types with application/domain types that share the same names (e.g. `CatalogItem`, `Category`):

- **Prefer module aliasing over type aliasing.** Refer to Smithy types by renaming the parent module, not the type. Use the original type names from the service/domain; only disambiguate Smithy types via the module path.
- **Do this only when there are name conflicts.** If there is no clash, use the types directly without aliasing.

**Example (when names conflict):**

```rust
// ✅ GOOD: alias the parent module, keep type names
use catalog_api::model as smithy;
use catalog_api::types as smithy_types;
use catalog_svc::catalog::api::{CatalogItem, Category};

// Use service types by name; use Smithy types with module prefix
fn map_category_from_smithy(value: smithy::Category) -> Category { ... }
fn service_item_to_smithy_item(value: CatalogItem) -> smithy::CatalogItem { ... }
```

```rust
// ❌ AVOID: renaming the types themselves (SmithyCatalogItem, ServiceCategory, etc.)
use catalog_api::model::{CatalogItem as SmithyCatalogItem, Category as SmithyCategory};
use catalog_svc::catalog::api::{CatalogItem as ServiceCatalogItem, Category as ServiceCategory};
```

This keeps service/domain types under their original names and makes Smithy types clearly scoped (e.g. `smithy::CatalogItem`, `smithy_types::DateTime`) without inventing new type names.
