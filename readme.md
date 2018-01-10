My results for derangement calculations and a cached method.

```Rust
test tests::bench_derange_cached ... bench:         155 ns/iter (+/- 13)
test tests::bench_derange_fast   ... bench:       4,250 ns/iter (+/- 100)
test tests::bench_derange_i      ... bench:       5,460 ns/iter (+/- 113)
test tests::bench_derange_u      ... bench:       5,293 ns/iter (+/- 180)
```