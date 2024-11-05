# Godot client rust code

## Tests
```shell
cargo test -p rheia-client
```

## Build for Linux
```shell
cargo b -p rheia-client
godot --export-release linux_desktop ~/godot/Rheia.x86_64
```

## Build for Windows from Linux
```shell
rustup target add x86_64-pc-windows-gn
cargo b -p rheia-client --release --target x86_64-pc-windows-gnu

godot --export-release windows_desktop ~/windows-build/Rheia.exe
```

## Run game from the console

```shell
godot --path ./ ./scenes/main_menu.tscn
```
