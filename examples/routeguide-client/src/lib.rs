use futures::SinkExt;
use wasi_grpc::WasiGrpcEndpoint;
use spin_sdk::http::{Headers, IncomingRequest, OutgoingResponse, ResponseOutparam, OutgoingBody};
use spin_sdk::http_component;
use tonic::Request;

use routeguide::{route_guide_client::RouteGuideClient, Point, Rectangle};
pub mod routeguide {
    tonic::include_proto!("routeguide");
}

#[http_component]
async fn handler(req: IncomingRequest, res: ResponseOutparam) {
    match req.path_with_query().as_deref() {
        Some("/print_features") => print_features(req, res)
            .await
            .expect("Failed to print features"),
        Some("/get_feature") => get_feature(req, res)
            .await
            .expect("Failed to get feature"),
        _ => bad_request(res),
    }
}

async fn get_feature(_req: IncomingRequest, res: ResponseOutparam) -> anyhow::Result<()> {
    let endpoint_uri = "http://[::1]:10000".parse().expect("Failed to parse endpoint URI");
    let endpoint = WasiGrpcEndpoint::new(endpoint_uri);
    let mut client = RouteGuideClient::new(endpoint);

    let response = client.get_feature(Request::new(Point {
        latitude: 409_146_138,
        longitude: -746_188_906,
    })).await?;

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

async fn print_features(_req: IncomingRequest, res: ResponseOutparam) -> anyhow::Result<()> {
    let endpoint_uri = "http://[::1]:10000".parse().expect("Failed to parse endpoint URI");
    let endpoint = WasiGrpcEndpoint::new(endpoint_uri);
    let mut client = RouteGuideClient::new(endpoint);

    let rectangle = Rectangle {
        lo: Some(Point {
            latitude: 400_000_000,
            longitude: -750_000_000,
        }),
        hi: Some(Point {
            latitude: 420_000_000,
            longitude: -730_000_000,
        }),
    };

    let mut stream = client
        .list_features(Request::new(rectangle))
        .await?
        .into_inner();

    let outgoing = OutgoingResponse::new(Headers::new());
    let mut body = outgoing.take_body();
    res.set(outgoing);

    while let Some(feature) = stream.message().await? {
        let body_bytes = format!("FEATURE = {feature:?}\n")
            .as_bytes()
            .to_vec();

        if let Err(err) = body.send(body_bytes).await {
            eprintln!("error send body: {err}");
        }
    }

    Ok(())
}

fn bad_request(response_out: ResponseOutparam) {
    respond(400, response_out)
}

fn respond(status: u16, response_out: ResponseOutparam) {
    let response = OutgoingResponse::new(Headers::new());
    response.set_status_code(status).unwrap();

    let body = response.body().expect("response should be writable");

    response_out.set(response);

    OutgoingBody::finish(body, None).expect("OutgoingBody::finish should succeed");
}