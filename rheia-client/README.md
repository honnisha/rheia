## Godot client rust code
```shell
cargo b -p rheia-client
cargo test -p rheia-client
```

## Build for Windows from Linux
```shell
rustup target add x86_64-pc-windows-gn
cargo b -p rheia-client --release --target x86_64-pc-windows-gnu

godot --export-release windows_desktop ~/godot/windows-build-001/Rheia.exe
```

## Run game from the console

```shell
godot --path ./ ./scenes/main.tscn
```
