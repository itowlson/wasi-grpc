# Spin wasi-grpc HelloWorld Example

This example is adapted from the [`helloworld`](https://github.com/hyperium/tonic/blob/v0.13.1/examples/src/helloworld/client.rs) client in the [`tonic`](https://github.com/hyperium/tonic/tree/v0.13.1) repository to run in a Spin application.

This example demonstrates making a unary gRPC request.

To test, start the helloworld-server from the tonic repository:

```console
$ cargo run --bin helloworld-server
...
GreeterServer listening on [::1]:50051
```

In another terminal, build and run the example:
```
$ SPIN_OUTBOUND_H2C_PRIOR_KNOWLEDGE=[::1]:50051 spin up --build
```

**NOTE** Omit the `SPIN_OUTBOUND_H2C_PRIOR_KNOWLEDGE` environment variable if testing against a server that is using tls.

In another terminal, send a request:
```console
$ curl localhost:3000/
Hello World!
```