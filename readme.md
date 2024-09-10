# Rheia

## Server
```
cargo r -p rheia-server
cargo test -p rheia-server -- --nocapture
```

## Client
```
cargo b -p rheia-client
cargo test -p rheia-client
```

## Common lib
```
cargo test -p common
```

## Client bevy
```
cargo run -p rheia-client-bevy
cargo run --features bevy/dynamic_linking -p rheia-client-bevy
cargo run --features bevy/trace_tracy -p rheia-client-bevy
```

[Module tracing::span](https://docs.rs/tracing/latest/tracing/span/index.html)
