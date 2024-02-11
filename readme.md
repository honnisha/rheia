# Honny-Craft

## Server
```
cargo r -p honny-server
cargo test -p honny-server
```

## Client
```
cargo b -p honny-client
cargo test -p honny-client
```

## Common lib
```
cargo test -p common
```

## Client bevy
```
cargo b -p honny-client-bevy
cargo run -p honny-client-bevy
cargo run --features bevy/dynamic_linking -p honny-client-bevy
cargo run --features bevy/trace_tracy -p honny-client-bevy
```

## Debug
```
let now = std::time::Instant::now();
let elapsed = now.elapsed();
println!("Chunk {} generated: {:.2?}", chunk_position, elapsed);

if elapsed > std::time::Duration::from_millis(1) {
println!("Process: {:.2?}", elapsed);
}
```
