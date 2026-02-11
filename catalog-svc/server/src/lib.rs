use catalog_api::{input, output};

/// Handler for HelloWorld: returns "Hello World".
pub async fn hello_world(_input: input::HelloWorldInput) -> output::HelloWorldOutput {
    output::HelloWorldOutput {
        message: Some("Hello World".into()),
    }
}
