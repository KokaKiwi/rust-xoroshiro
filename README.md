rust-xoroshiro
==============

Pure Rust implementation of the Xoroshiro PRNG algorithm.

Source: http://xoroshiro.di.unimi.it

API Documentation: http://kokakiwi.github.io/rust-xoroshiro/xoroshiro/index.html

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
