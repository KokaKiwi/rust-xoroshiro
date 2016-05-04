rust-xoroshiro
==============

Pure Rust implementation of the Xoroshiro PRNG algorithm.

From: http://xoroshiro.di.unimi.it

Benchmarks
----------

```
$ cargo bench

test bench_xorshiro128plus ... bench:       3,312 ns/iter (+/- 190) = 2415 MB/s
```

TO-DO
-----

- [x] Xoroshiro128+ implementation
- [ ] XorShift* implementation
- [ ] XorShift+ implementation
