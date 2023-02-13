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

pub type StdDefault = std::collections::HashMap<u64, u64>;
pub type StdBTreeMap = std::collections::BTreeMap<u64, u64>;
pub type StdFxHash = std::collections::HashMap<u64, u64, fxhash::FxBuildHasher>;
pub type StdAHash = std::collections::HashMap<u64, u64, ahash::RandomState>;
pub type StdAHashFixed = std::collections::HashMap<u64, u64, FixedState>;
pub type HashBrown13FxHash = hashbrown_13::HashMap<u64, u64, fxhash::FxBuildHasher>;
pub type HashBrown13AHash = hashbrown_13::HashMap<u64, u64, ahash::RandomState>;
pub type HashBrown13AHashFixed = hashbrown_13::HashMap<u64, u64, FixedState>;
pub type HashBrown12FxHash = hashbrown_12::HashMap<u64, u64, fxhash::FxBuildHasher>;
pub type HashBrown12AHash = hashbrown_12::HashMap<u64, u64, ahash::RandomState>;
pub type HashBrown12AHashFixed = hashbrown_12::HashMap<u64, u64, FixedState>;

#[no_mangle]
#[inline(never)]
pub fn hashbrown_13_ahash(map: &HashBrown13AHash, key: u64) -> Option<&u64> {
    map.get(&key)
}

#[no_mangle]
#[inline(never)]
pub fn hashbrown_13_ahash_fixed(map: &HashBrown13AHashFixed, key: u64) -> Option<&u64> {
    map.get(&key)
}

#[no_mangle]
#[inline(never)]
pub fn hashbrown_13_fxhash(map: &HashBrown13FxHash, key: u64) -> Option<&u64> {
    map.get(&key)
}

#[no_mangle]
#[inline(never)]
pub fn hashbrown_12_ahash(map: &HashBrown12AHash, key: u64) -> Option<&u64> {
    map.get(&key)
}

#[no_mangle]
#[inline(never)]
pub fn hashbrown_12_ahash_fixed(map: &HashBrown12AHashFixed, key: u64) -> Option<&u64> {
    map.get(&key)
}

#[no_mangle]
#[inline(never)]
pub fn hashbrown_12_fxhash(map: &HashBrown12FxHash, key: u64) -> Option<&u64> {
    map.get(&key)
}

#[no_mangle]
#[inline(never)]
pub fn std_fxhash(map: &StdFxHash, key: u64) -> Option<&u64> {
    map.get(&key)
}

#[no_mangle]
#[inline(never)]
pub fn std_ahash(map: &StdAHash, key: u64) -> Option<&u64> {
    map.get(&key)
}

#[no_mangle]
#[inline(never)]
pub fn std_ahash_fixed(map: &StdAHashFixed, key: u64) -> Option<&u64> {
    map.get(&key)
}

pub trait TestMap: Default {
    fn type_name() -> &'static str;

    fn insert(&mut self, key: u64, value: u64);

    fn get(&self, key: u64) -> Option<&u64>;
}

impl TestMap for StdDefault {
    fn type_name() -> &'static str {
        "std_hashmap"
    }

    fn insert(&mut self, key: u64, value: u64) {
        self.insert(key, value);
    }

    fn get(&self, key: u64) -> Option<&u64> {
        self.get(&key)
    }
}

impl TestMap for StdBTreeMap {
    fn type_name() -> &'static str {
        "std_btree"
    }

    fn insert(&mut self, key: u64, value: u64) {
        self.insert(key, value);
    }

    fn get(&self, key: u64) -> Option<&u64> {
        self.get(&key)
    }
}

impl TestMap for HashBrown13FxHash {
    #[inline]
    fn insert(&mut self, key: u64, value: u64) {
        self.insert(key, value);
    }

    #[inline]
    fn get(&self, key: u64) -> Option<&u64> {
        self.get(&key)
    }

    fn type_name() -> &'static str {
        "hashbrown13_fxhash"
    }
}

impl TestMap for HashBrown13AHash {
    #[inline]
    fn insert(&mut self, key: u64, value: u64) {
        self.insert(key, value);
    }

    #[inline]
    fn get(&self, key: u64) -> Option<&u64> {
        self.get(&key)
    }

    fn type_name() -> &'static str {
        "hashbrown13_ahash"
    }
}

impl TestMap for HashBrown13AHashFixed {
    #[inline]
    fn insert(&mut self, key: u64, value: u64) {
        self.insert(key, value);
    }

    #[inline]
    fn get(&self, key: u64) -> Option<&u64> {
        self.get(&key)
    }

    fn type_name() -> &'static str {
        "hashbrown13_ahash_fixed"
    }
}

impl TestMap for HashBrown12FxHash {
    #[inline]
    fn insert(&mut self, key: u64, value: u64) {
        self.insert(key, value);
    }

    #[inline]
    fn get(&self, key: u64) -> Option<&u64> {
        self.get(&key)
    }

    fn type_name() -> &'static str {
        "hashbrwon12_fxhash"
    }
}

impl TestMap for HashBrown12AHash {
    #[inline]
    fn insert(&mut self, key: u64, value: u64) {
        self.insert(key, value);
    }

    #[inline]
    fn get(&self, key: u64) -> Option<&u64> {
        self.get(&key)
    }

    fn type_name() -> &'static str {
        "hashbrown12_ahash"
    }
}

impl TestMap for HashBrown12AHashFixed {
    #[inline]
    fn insert(&mut self, key: u64, value: u64) {
        self.insert(key, value);
    }

    #[inline]
    fn get(&self, key: u64) -> Option<&u64> {
        self.get(&key)
    }

    fn type_name() -> &'static str {
        "hashbrown12_ahash_fixed"
    }
}

impl TestMap for StdFxHash {
    #[inline]
    fn insert(&mut self, key: u64, value: u64) {
        self.insert(key, value);
    }

    #[inline]
    fn get(&self, key: u64) -> Option<&u64> {
        self.get(&key)
    }

    fn type_name() -> &'static str {
        "std_fxhash"
    }
}

impl TestMap for StdAHash {
    #[inline]
    fn insert(&mut self, key: u64, value: u64) {
        self.insert(key, value);
    }

    #[inline]
    fn get(&self, key: u64) -> Option<&u64> {
        self.get(&key)
    }

    fn type_name() -> &'static str {
        "std_ahash"
    }
}

impl TestMap for StdAHashFixed {
    #[inline]
    fn insert(&mut self, key: u64, value: u64) {
        self.insert(key, value);
    }

    #[inline]
    fn get(&self, key: u64) -> Option<&u64> {
        self.get(&key)
    }

    fn type_name() -> &'static str {
        "std_ahash_fixed"
    }
}
