# Godot client rust code

## Run tests
```shell
cargo test -p rheia-client
```

## Build debug rust bynary for Linux and Windows
```shell
cargo b -p rheia-client
```
## Build project for Linux
```shell
godot --export-release linux_desktop ~/godot/Rheia.x86_64
```

## Build release for Windows from Linux
```shell
rustup target add x86_64-pc-windows-gn
cargo b -p rheia-client --release --target x86_64-pc-windows-gnu

godot --export-release windows_desktop ~/Dropbox/Rheia/Rheia.exe
```

## Run game from the console

```shell
godot --path ./ ./scenes/main_menu.tscn
```
