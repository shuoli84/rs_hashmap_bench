use core::hash::BuildHasherDefault;
use fxhash::FxBuildHasher;
use rustc_hash::FxHasher;

pub type StdDefault = std::collections::HashMap<u64, u64>;
pub type StdRustcHash = std::collections::HashMap<u64, u64, BuildHasherDefault<FxHasher>>;
pub type StdFxHash = std::collections::HashMap<u64, u64, FxBuildHasher>;
pub type StdAHash = std::collections::HashMap<u64, u64, ahash::RandomState>;
pub type HashBrown13RustcHash = hashbrown_13::HashMap<u64, u64, BuildHasherDefault<FxHasher>>;
pub type HashBrown13FxHash = hashbrown_13::HashMap<u64, u64, FxBuildHasher>;
pub type HashBrown13AHash = hashbrown_13::HashMap<u64, u64, ahash::RandomState>;

#[no_mangle]
#[inline(never)]
pub fn hashbrown_13_ahash(map: &HashBrown13AHash, key: u64) -> Option<&u64> {
    map.get(&key)
}

#[no_mangle]
#[inline(never)]
pub fn hashbrown_13_rustc_hash(map: &HashBrown13RustcHash, key: u64) -> Option<&u64> {
    map.get(&key)
}

#[no_mangle]
#[inline(never)]
pub fn hashbrown_13_fxhash(map: &HashBrown13FxHash, key: u64) -> Option<&u64> {
    map.get(&key)
}

#[no_mangle]
#[inline(never)]
pub fn std_rustc_hash(map: &StdRustcHash, key: u64) -> Option<&u64> {
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

pub trait TestMap: Default {
    fn type_name() -> &'static str;

    fn test_insert(&mut self, key: u64, value: u64);

    fn test_get(&self, key: u64) -> Option<&u64>;
}

impl TestMap for HashBrown13RustcHash {
    #[inline]
    fn test_insert(&mut self, key: u64, value: u64) {
        self.insert(key, value);
    }

    #[inline]
    fn test_get(&self, key: u64) -> Option<&u64> {
        self.get(&key)
    }

    fn type_name() -> &'static str {
        "hashbrown13_rustc_hash"
    }
}

impl TestMap for HashBrown13FxHash {
    #[inline]
    fn test_insert(&mut self, key: u64, value: u64) {
        self.insert(key, value);
    }

    #[inline]
    fn test_get(&self, key: u64) -> Option<&u64> {
        self.get(&key)
    }

    fn type_name() -> &'static str {
        "hashbrown13_fxhash"
    }
}

impl TestMap for HashBrown13AHash {
    #[inline]
    fn test_insert(&mut self, key: u64, value: u64) {
        self.insert(key, value);
    }

    #[inline]
    fn test_get(&self, key: u64) -> Option<&u64> {
        self.get(&key)
    }

    fn type_name() -> &'static str {
        "hashbrown13_ahash"
    }
}

impl TestMap for StdDefault {
    #[inline]
    fn test_insert(&mut self, key: u64, value: u64) {
        self.insert(key, value);
    }

    #[inline]
    fn test_get(&self, key: u64) -> Option<&u64> {
        self.get(&key)
    }

    fn type_name() -> &'static str {
        "std_default"
    }
}

impl TestMap for StdRustcHash {
    #[inline]
    fn test_insert(&mut self, key: u64, value: u64) {
        self.insert(key, value);
    }

    #[inline]
    fn test_get(&self, key: u64) -> Option<&u64> {
        self.get(&key)
    }

    fn type_name() -> &'static str {
        "std_rustc_hash"
    }
}

impl TestMap for StdFxHash {
    #[inline]
    fn test_insert(&mut self, key: u64, value: u64) {
        self.insert(key, value);
    }

    #[inline]
    fn test_get(&self, key: u64) -> Option<&u64> {
        self.get(&key)
    }

    fn type_name() -> &'static str {
        "std_fxhash"
    }
}

impl TestMap for StdAHash {
    #[inline]
    fn test_insert(&mut self, key: u64, value: u64) {
        self.insert(key, value);
    }

    #[inline]
    fn test_get(&self, key: u64) -> Option<&u64> {
        self.get(&key)
    }

    fn type_name() -> &'static str {
        "std_ahash"
    }
}
