# Honny-Craft

## Server
```
cargo r -p honny-craft-server
cargo test -p honny-craft-server
```

## Client
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
