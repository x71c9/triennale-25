# Triennale 2025

```bash
nix-shell

cd composer/

cargo run lights <ID> <on/off>

cargo run sparklings <ID> <on/off>

cargo run robots <ID> init

cargo run robots <ID> move <POS> <SPEED>

cargo run installation <start/stop>
```

```bash
cargo run lights 6 on

cargo run sparklings 3 on

cargo run robots 4 init

cargo run robots 3 move 5.0 1.0

cargo run installation start
```

```bash
cargo run lights 5 off --no-dry-run

cargo run robots 2 move 0.0 0.1 --no-verbose
```

