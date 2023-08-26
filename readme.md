# Honny-Craft

## Server
```
cargo r -p honny-craft-server
cargo test -p honny-craft-server
```

## Client bevy
```
cargo b -p honny-client-bevy
cargo run -p honny-client-bevy
cargo run --features bevy/dynamic_linking -p honny-client-bevy
cargo run --features bevy/trace_tracy -p honny-client-bevy
```

## Client Godot
```
cargo b -p honny-craft
```

## Debug
```
use std::time::Instant;
let now = Instant::now();
let elapsed = now.elapsed();
println!("Chunk {} generated: {:.2?}", chunk_position, elapsed);
```
