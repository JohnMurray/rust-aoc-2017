# Day 2

After the failure in day 1 to capture idiomatic rust, I am attempting to improve my skills by
taking a more functional approach to the problem solving. One aspect that I still need to better
understand is the `?.` operator and how the macro scopes the result value. 

## Results

You can find the solution in `src/main.rs` as well as the tests. I've also generated a large random
input in `src/input.rs` (generated with `gen_input.sh`). The benchmark results for the small test
input as well as the large input are:

```
▶ cargo bench
   Compiling day2 v0.1.0 (file:///home/jmurray/Dropbox/projects/playground/rust-learning/aoc/day2)
    Finished release [optimized] target(s) in 1m 04s
     Running target/release/deps/day2-b65734f79a48a526

running 4 tests
test tests::bench_evenly_div             ... bench:         114 ns/iter (+/- 1)
test tests::bench_evenly_div_large_input ... bench:   7,436,406 ns/iter (+/- 2,168,565)
test tests::bench_min_max                ... bench:          11 ns/iter (+/- 0)
test tests::bench_min_max_large_input    ... bench:     165,599 ns/iter (+/- 1,218)
```

## Fun Findings

In part one I used a function `min_max_tuple` that used basic if statemets to determine if the given
value was smaller than the first element in the tuple, or larger than the last. It would return a modified
version of the tuple if so, or the original tuple. When converting this function to use a `match` statement
I was able to shave of 10us (10k ns) from the bench-time of the randomly generated test-set.