# spin-grpc

⚠️ This is an experimental crate for enabling gRPC rust clients to work with Spin and along side the spin-rust-sdk built on fermyon/wasi-hyperium

To test:
```
spin up --build -f examples/routeguide-client
```

To test with a non-TLS gRPC server:
```
SPIN_OUTBOUND_H2C_PRIOR_KNOWLEDGE=<authority> spin up --build -f examples/routeguide-client
```

E.g.,
```
SPIN_OUTBOUND_H2C_PRIOR_KNOWLEDGE=[::1]:10000
```
