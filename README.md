# Mobius Strip Generator

A surface with only one side, one edge and non-orientable.

Best expressed parametrically:

- `u ∈ [-1, 1]` = position across width
- `θ ∈ [0, 2π]` = angle around loop

3D coordinates:

```
x = (1 + (u/2) * cos(θ/2)) * cos(θ)
y = (1 + (u/2) * cos(θ/2)) * sin(θ)
z = (u/2) * sin(θ/2)
```

## Run

```
cargo build --release
./target/release/mobius -s 0.008
```

## Flags

```
-s, --steps <STEPS>  Controls density of points (smaller = more detailed) [default: 0.05]
-p, --path <PATH>    Path to save CSV output [default: data/mobius_points.csv]
-h, --help           Print help
-V, --version        Print version
```

## Output

```
MOBIUS STRIP SIMULATION
Step size     : 0.008
Points        : 197286
Output file   : data/mobius_points.csv
Time taken    : 79.49ms
```

## Visualization

![Mobius Strip Animation](./media/loop.mp4)
