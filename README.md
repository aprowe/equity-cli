# Poker Equity Calculator

## Build and Run

```
cargo run AdAc KdKc
```

### With extra args:
```bash
cargo run -- --board=2c8dJd --iterations=100000 Jc4c KdQd

Hand 1: [Jc, 4c]
Hand 2: [Kd, Qd]
Board: [2c, 8d, Jd]
Running 100000 iterations...
Iteration: 0
Iteration: 10000
Iteration: 20000
Iteration: 30000
Iteration: 40000
Iteration: 50000
Iteration: 60000
Iteration: 70000
Iteration: 80000
Iteration: 90000
Hand 1: 47.19%
Hand 2: 52.81%
```

## Test
```
cargo test
```

