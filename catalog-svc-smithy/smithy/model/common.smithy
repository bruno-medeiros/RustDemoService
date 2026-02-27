namespace com.github.bruno_medeiros

/// Internal server error.
@error("server")
@httpError(500)
structure InternalServerError {
    message: String
}

/// Resource not found.
@error("client")
@httpError(404)
structure NotFoundError {
    message: String
}

/// An identifier to describe a unique resource
@length(min: 1, max: 128)
@pattern("^[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}$")
string Uuid

/// Date with day resolution only (YYYY-MM-DD). Time components are not used.
@pattern("^[0-9]{4}-[0-9]{2}-[0-9]{2}$")
@length(min: 10, max: 10)
string DateOnly
