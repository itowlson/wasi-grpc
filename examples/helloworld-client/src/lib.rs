use anyhow::Context;
use spin_sdk::http::{self, IntoResponse};
use spin_sdk::http_component;
use tonic::Request;
use wasi_grpc::WasiGrpcEndpoint;

use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[http_component]
async fn handler(_req: http::Request) -> anyhow::Result<impl IntoResponse> {
    let endpoint_uri = "http://[::1]:50051".parse().context("Failed to parse endpoint URI")?;
    let endpoint = WasiGrpcEndpoint::new(endpoint_uri);
    let mut client = GreeterClient::new(endpoint);

    let request = Request::new(HelloRequest {
        name: "World".to_string(),
    });

    let message = client
        .say_hello(request)
        .await?
        .into_inner()
        .message;

    let response = http::Response::new(200, message);

    Ok(response)
}
