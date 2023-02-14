# Introduction
This is a benchmark designed to figure out which HashMap is the most performant one for key u64. 

# Run 
```bash
cargo bench --bench bench_hashmap
```

# Performance Results
std::collections::HashMap + RandomeState for sip/fxhash/ahash. FxHash is faster in both insert and query.

``` bash

get_std_default_2000    time:   [21.192 µs 21.905 µs 22.608 µs]

get_std_rustc_hash_2000 time:   [4.4766 µs 4.6242 µs 4.7721 µs]

get_hashbrown13_rustc_hash_2000
                        time:   [4.5464 µs 4.6937 µs 4.8410 µs]

get_std_fxhash_2000     time:   [4.7066 µs 4.8594 µs 5.0119 µs]

get_hashbrown13_fxhash_2000
                        time:   [4.6495 µs 4.8122 µs 4.9803 µs]

get_std_ahash_2000      time:   [6.8689 µs 7.0993 µs 7.3292 µs]

get_hashbrown13_ahash_2000
                        time:   [6.5932 µs 6.8030 µs 7.0245 µs]

get_std_default_10000   time:   [114.81 µs 118.54 µs 122.35 µs]

get_std_rustc_hash_10000
                        time:   [31.251 µs 32.275 µs 33.312 µs]

get_hashbrown13_rustc_hash_10000
                        time:   [31.862 µs 32.872 µs 33.858 µs]

get_std_fxhash_10000    time:   [30.424 µs 31.363 µs 32.320 µs]

get_hashbrown13_fxhash_10000
                        time:   [30.151 µs 31.113 µs 32.089 µs]

get_std_ahash_10000     time:   [41.191 µs 42.493 µs 43.852 µs]

get_hashbrown13_ahash_10000
                        time:   [40.547 µs 41.790 µs 43.109 µs]

insert_std_default_2000 time:   [21.478 µs 22.218 µs 22.978 µs]

insert_std_rustc_hash_2000
                        time:   [5.1010 µs 5.2689 µs 5.4422 µs]

insert_hashbrown13_rustc_hash_2000
                        time:   [4.9022 µs 5.0641 µs 5.2325 µs]

insert_std_fxhash_2000  time:   [4.9189 µs 5.0734 µs 5.2293 µs]

insert_hashbrown13_fxhash_2000
                        time:   [5.0153 µs 5.1911 µs 5.3666 µs]

insert_std_ahash_2000   time:   [10.376 µs 10.748 µs 11.108 µs]

insert_hashbrown13_ahash_2000
                        time:   [10.305 µs 10.655 µs 11.003 µs]

insert_std_default_10000
                        time:   [119.20 µs 122.86 µs 126.50 µs]

insert_std_rustc_hash_10000
                        time:   [33.706 µs 34.859 µs 36.017 µs]

insert_hashbrown13_rustc_hash_10000
                        time:   [33.158 µs 34.241 µs 35.329 µs]

insert_std_fxhash_10000 time:   [33.885 µs 34.964 µs 36.023 µs]

insert_hashbrown13_fxhash_10000
                        time:   [34.076 µs 35.171 µs 36.344 µs]

insert_std_ahash_10000  time:   [62.946 µs 65.116 µs 67.370 µs]

insert_hashbrown13_ahash_10000
                        time:   [64.993 µs 67.299 µs 69.505 µs]
```

# Dissemble result

## Run

```bash
make && ls -al dis/
```

## Result

Under arch arm64, with lto = 'fat', opt-level as 3, fxhash's code size is also smaller.

``` bash
std_fxhash around 52 instructions. 
std_ahash around 76 instructions.
```

# Conclusion
For u64 as key, fxhash both faster and smaller.

