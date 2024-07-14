# Rheia

## Server
```
cargo r -p rheia-server
cargo test -p rheia-server
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
let now = std::time::Instant::now();
let elapsed = now.elapsed();
log::debug!(target: "test", "Chunk {} generated: {:.2?}", chunk_position, elapsed);

if elapsed > std::time::Duration::from_millis(1) {
log::debug!(target: "test", "Process: {:.2?}", elapsed);
}
```
