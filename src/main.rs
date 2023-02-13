use hash_perf::*;

pub fn test_hash<H: TestMap, F: Fn(&H, u64) -> Option<&u64>>(get_f: F) {
    let map = H::default();
    let value = get_f(&map, 1).copied();
    println!("{:?}", value);
}

fn main() {
    test_hash(hashbrown_13_ahash);
    test_hash(hashbrown_13_fxhash);
    test_hash(std_fxhash);
    test_hash(std_ahash);
}
