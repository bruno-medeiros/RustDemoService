$version: "2"

namespace com.github.bruno_medeiros

use aws.protocols#restJson1

/// Catalog service
@title("Coffee Shop Service")
@restJson1
service CatalogService {
    version: "2026-01-01"
    operations: [
        HelloWorld
    ]
    //    resources: [
    //        CatalogItem
    //    ]
}

/// Returns a hello world string
@http(method: "GET", uri: "/hello")
@readonly
operation HelloWorld {
    output := {
        message: String
    }
}
