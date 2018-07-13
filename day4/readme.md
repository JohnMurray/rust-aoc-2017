# Day 4

Solution focuses on trying to eliminate copies. Benchmark, when compared to a functional
version (albeit using the `itertools` crate), significantly out-performs even for small
input (benched with AoC input).

```text
▶ cargo bench
    Finished release [optimized] target(s) in 0.03s
     Running target/release/deps/aoc-80170a2c2d31ebf8

running 4 tests
test testing::aoc_part_1 ... ignored
test testing::aoc_part_2 ... ignored
test testing::part_1_bench            ... bench:     461,933 ns/iter (+/- 18,494)
test testing::part_1_funcitonal_bench ... bench:     649,357 ns/iter (+/- 54,476)

test result: ok. 0 passed; 0 failed; 2 ignored; 2 measured; 0 filtered out
```