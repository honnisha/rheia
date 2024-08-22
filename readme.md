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

## Debug
```
    #[cfg(feature = "trace")]
    let _span = bevy_utils::tracing::info_span!("send_chunks").entered();
}
```
