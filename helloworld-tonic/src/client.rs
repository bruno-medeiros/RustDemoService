use crate::hello_world::HelloReply;
use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use tonic::Response;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response: Response<HelloReply> = client.say_hello(request).await?;

    println!("RESPONSE={:?}", &response);

    Ok(())
}
