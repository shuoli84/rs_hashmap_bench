use ahash::AHasher;

#[derive(Debug, Clone, Default)]
pub struct FixedState;

impl std::hash::BuildHasher for FixedState {
    type Hasher = AHasher;

    #[inline]
    fn build_hasher(&self) -> AHasher {
        AHasher::new_with_keys(
            0b1001010111101110000001001100010000000011001001101011001001111000,
            0b1100111101101011011110001011010100000100001111100011010011010101,
        )
    }
}

type StdFxHash = std::collections::HashMap<u64, u64, fxhash::FxBuildHasher>;
type StdAHash = std::collections::HashMap<u64, u64, ahash::RandomState>;
type StdAHashFixed = std::collections::HashMap<u64, u64, FixedState>;
type HashBrown13FxHash = hashbrown_13::HashMap<u64, u64, fxhash::FxBuildHasher>;
type HashBrown13AHash = hashbrown_13::HashMap<u64, u64, ahash::RandomState>;
type HashBrown13AHashFixed = hashbrown_13::HashMap<u64, u64, FixedState>;
type HashBrown12FxHash = hashbrown_12::HashMap<u64, u64, fxhash::FxBuildHasher>;
type HashBrown12AHash = hashbrown_12::HashMap<u64, u64, ahash::RandomState>;
type HashBrown12AHashFixed = hashbrown_12::HashMap<u64, u64, FixedState>;

#[no_mangle]
#[inline(never)]
fn hashbrown_13_ahash(map: &HashBrown13AHash, key: u64) -> Option<&u64> {
    map.get(&key)
}

#[no_mangle]
#[inline(never)]
fn hashbrown_13_ahash_fixed(map: &HashBrown13AHashFixed, key: u64) -> Option<&u64> {
    map.get(&key)
}

#[no_mangle]
#[inline(never)]
fn hashbrown_13_fxhash(map: &HashBrown13FxHash, key: u64) -> Option<&u64> {
    map.get(&key)
}

#[no_mangle]
#[inline(never)]
fn hashbrown_12_ahash(map: &HashBrown12AHash, key: u64) -> Option<&u64> {
    map.get(&key)
}

#[no_mangle]
#[inline(never)]
fn hashbrown_12_ahash_fixed(map: &HashBrown12AHashFixed, key: u64) -> Option<&u64> {
    map.get(&key)
}

#[no_mangle]
#[inline(never)]
fn hashbrown_12_fxhash(map: &HashBrown12FxHash, key: u64) -> Option<&u64> {
    map.get(&key)
}

#[no_mangle]
#[inline(never)]
fn std_fxhash(map: &StdFxHash, key: u64) -> Option<&u64> {
    map.get(&key)
}

#[no_mangle]
#[inline(never)]
fn std_ahash(map: &StdAHash, key: u64) -> Option<&u64> {
    map.get(&key)
}

#[no_mangle]
#[inline(never)]
fn std_ahash_fixed(map: &StdAHashFixed, key: u64) -> Option<&u64> {
    map.get(&key)
}

trait HashMapTest: Default {}

impl HashMapTest for HashBrown13FxHash {}
impl HashMapTest for HashBrown13AHash {}
impl HashMapTest for HashBrown13AHashFixed {}
impl HashMapTest for HashBrown12FxHash {}
impl HashMapTest for HashBrown12AHash {}
impl HashMapTest for HashBrown12AHashFixed {}
impl HashMapTest for StdFxHash {}
impl HashMapTest for StdAHash {}
impl HashMapTest for StdAHashFixed {}

fn test_hash<H: HashMapTest, F: Fn(&H, u64) -> Option<&u64>>(get_f: F) {
    let map = H::default();
    let value = get_f(&map, 1).copied();
    println!("{:?}", value);
}

fn main() {
    test_hash(hashbrown_12_ahash);
    test_hash(hashbrown_12_ahash_fixed);
    test_hash(hashbrown_12_fxhash);
    test_hash(hashbrown_13_ahash);
    test_hash(hashbrown_13_ahash_fixed);
    test_hash(hashbrown_13_fxhash);
    test_hash(std_fxhash);
    test_hash(std_ahash);
    test_hash(std_ahash_fixed);
}
