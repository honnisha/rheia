```shell
cd ~/godot/rheia/
cargo run -p network-test -- -t=server --ip=192.168.0.185:25565
cargo run -p network-test -- -t=client --ip=192.168.0.185:25565

cargo b -p network-test --release --target x86_64-pc-windows-gnu
```
