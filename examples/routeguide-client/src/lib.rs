use futures::SinkExt;
use routeguide::{route_guide_client::RouteGuideClient, Point};
use spin_grpc::WasiGrpcEndpoint;
use spin_sdk::http::{Headers, IncomingRequest, OutgoingResponse, ResponseOutparam};
use spin_sdk::http_component;
use tonic::Request;

pub mod routeguide {
    tonic::include_proto!("routeguide");
}

#[http_component]
async fn handler(req: IncomingRequest, res: ResponseOutparam) {
   make_call(req, res).await.expect("Failed to make gRPC call");
}

async fn make_call(_req: IncomingRequest, res: ResponseOutparam) -> anyhow::Result<()> {
    let endpoint_uri = "http://[::1]:10000".parse().expect("Failed to parse endpoint URI");
    let endpoint = WasiGrpcEndpoint::new(endpoint_uri);

    let mut client = RouteGuideClient::new(endpoint);

    let response = client.get_feature(Request::new(Point {
        latitude: 409_146_138,
        longitude: -746_188_906,
    })).await?;

    println!("Response: {:#?}", response);

    let outgoing = OutgoingResponse::new(Headers::new());
    let mut body = outgoing.take_body();
    res.set(outgoing);

    let body_bytes = format!("Feature: {:?}\n", response.into_inner())
        .as_bytes()
        .to_vec();

    if let Err(err) = body.send(body_bytes).await {
        eprintln!("error send body: {err}");
    }

    Ok(())
}