# Day 12: Hot Springs

<!-- ![Completed](completed.png) -->

## Part 1

```bash
cargo test
cargo run --bin part1
```

[instructions](https://adventofcode.com/2023/day/11)


## Part 2

```bash
cargo test
cargo run --bin part2
```

DO NOT USE!!!
Note: Although this version is running and very efficent, it's very messy and just a proof of concept.

There is a mixture of solutions working although one of them has an issue at the moment and producing the errors with problem lines.

HINT to myself: Rewrite!!! Use the pattern method to get correct line combinations. (last incorrect: 212844161482 (unknown start removed))

When sections length (vec_original) is 1 then the count is:
"original" * "original"?.pow(4)

When section length is >= 2 then the count is:
"section"[0] * section[1..length]?section[0] * section[1..length]
