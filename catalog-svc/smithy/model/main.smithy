$version: "2"

namespace com.github.bruno_medeiros

use aws.protocols#restJson1
use smithy.framework#ValidationException

/// Catalog service
@title("Coffee Shop Service")
@restJson1
service CatalogService {
    version: "2026-01-01"
    operations: [
        HelloWorld
    ]
    resources: [
        CatalogItemResource
    ]
}

/// Returns a hello world string
@http(method: "GET", uri: "/hello")
@readonly
operation HelloWorld {
    output := {
        message: String
    }
}

// ---------------------------------------------------------------------------
// CatalogItemResource
// ---------------------------------------------------------------------------
/// Catalog item category
enum Category {
    BOOKS = "Books"
    ELECTRONICS = "Electronics"
}

@mixin
structure CatalogItemBodyMixin {
    @required
    name: String

    @required
    description: String

    @required
    category: Category

    @required
    date: DateOnly

    brand: String

    /// Price as decimal string (e.g. "19.99"); use String until smithy-rs supports BigDecimal (issue 312).
    @required
    price: String
}

/// Catalog item representation
structure CatalogItem with [CatalogItemBodyMixin] {
    @required
    itemId: Uuid

    @required
    createdAt: Timestamp

    @required
    modifiedAt: Timestamp
}

/// Create input: catalog item body (server assigns itemId)
structure CreateCatalogItemInput with [CatalogItemBodyMixin] {}

@http(method: "POST", uri: "/catalog/items")
operation CreateCatalogItem {
    input: CreateCatalogItemInput
    output: CatalogItem
    errors: [
        ValidationException
    ]
}

@readonly
@http(method: "GET", uri: "/catalog/items/{itemId}")
operation GetCatalogItem {
    input := {
        @required
        @httpLabel
        itemId: Uuid
    }

    output: CatalogItem

    errors: [
        ValidationException
    ]
}

/// Update input: resource id plus catalog item body (same shape as CatalogItem)
structure UpdateCatalogItemInput with [CatalogItemBodyMixin] {
    @required
    @httpLabel
    itemId: Uuid
}

@http(method: "POST", uri: "/catalog/items/{itemId}")
operation UpdateCatalogItem {
    input: UpdateCatalogItemInput
    output: CatalogItem
    errors: [
        ValidationException
    ]
}

@idempotent
@http(method: "DELETE", uri: "/catalog/items/{itemId}")
operation DeleteCatalogItem {
    input := {
        @required
        @httpLabel
        itemId: Uuid
    }

    output: Unit

    errors: [
        ValidationException
    ]
}

list CatalogItemList {
    member: CatalogItem
}

/// List of catalog items with optional pagination token
structure ListCatalogItemsOutput {
    @required
    items: CatalogItemList

    nextToken: String
}

@readonly
@paginated(inputToken: "nextToken", outputToken: "nextToken", pageSize: "maxResults", items: "items")
@http(method: "GET", uri: "/catalog/items")
operation ListCatalogItems {
    input := {
        @httpQuery("maxResults")
        maxResults: Integer

        @httpQuery("nextToken")
        nextToken: String
    }

    output: ListCatalogItemsOutput

    errors: [
        ValidationException
    ]
}

resource CatalogItemResource {
    identifiers: {
        itemId: Uuid
    }
    create: CreateCatalogItem
    read: GetCatalogItem
    list: ListCatalogItems
    update: UpdateCatalogItem
    delete: DeleteCatalogItem
}
