# Introduction
This is a benchmark designed to figure out which HashMap is the most performant one for key u64. 

# Run 
```bash
cargo bench --bench bench_hashmap
```

# Performance Results
std::collections::HashMap + RandomeState for sip/fxhash/ahash. FxHash is faster in both insert and query.

``` bash
get_std_default_2000    time:   [16.483 µs 16.498 µs 16.515 µs]
get_std_fxhash_2000     time:   [6.4908 µs 7.3761 µs 8.4999 µs]
get_std_ahash_2000      time:   [6.7078 µs 9.1841 µs 13.483 µs]

insert_std_default_2000 time:   [34.214 µs 34.263 µs 34.296 µs]
insert_std_fxhash_2000  time:   [5.0889 µs 5.0895 µs 5.0901 µs]
insert_std_ahash_2000   time:   [7.3801 µs 7.3832 µs 7.3858 µs]
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

