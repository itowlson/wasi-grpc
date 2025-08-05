# Spin wasi-grpc RouteGuide Example

This example is adapted from the [`routeguide`](https://github.com/hyperium/tonic/blob/v0.13.1/examples/src/routeguide/client.rs) client in the [`tonic`](https://github.com/hyperium/tonic/tree/v0.13.1) repository to run in a Spin application.

This example demonstates making unary and uni-direction streaming gRPC requests.

To test, start the routeguide-server from the tonic repository:

```console
$ cargo run --bin routeguide-server
...
RouteGuideServer listening on: [::1]:10000
```

In another terminal, build and run the example:
```
$ SPIN_OUTBOUND_H2C_PRIOR_KNOWLEDGE=[::1]:10000 spin up --build
```

**NOTE** Omit the `SPIN_OUTBOUND_H2C_PRIOR_KNOWLEDGE` environment variable if testing against a server that is using tls.

In another terminal, send a request:
```console
$ curl localhost:3000/get_feature
Feature: Feature { name: "Berkshire Valley Management Area Trail, Jefferson, NJ, USA", location: Some(Point { latitude: 409146138, longitude: -746188906 }) }