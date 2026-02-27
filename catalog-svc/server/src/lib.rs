pub mod config;
pub mod server;

use catalog_api::{error, input, output};

/// Handler for HelloWorld: returns "Hello World".
pub async fn hello_world(
    input: input::HelloWorldInput,
) -> Result<output::HelloWorldOutput, error::HelloWorldError> {
    // Temporary: return 500 when name contains "XXX" (for testing InternalServerError)
    if input.name.contains("XXX") {
        return Err(error::HelloWorldError::from(error::InternalServerError {
            message: Some("Internal server error".into()),
        }));
    }

    Ok(output::HelloWorldOutput {
        message: Some("Hello World".into()),
    })
}
