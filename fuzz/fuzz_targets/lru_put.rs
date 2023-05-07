#![no_main]
use libfuzzer_sys::fuzz_target;
use lru::LruCache;
use std::num::NonZeroUsize;

fuzz_target!(|value: (u8, &[u8])| {
    let (key, val) = value;
    let mut cache = LruCache::new(NonZeroUsize::new(1).unwrap());
    cache.put(key, val);
});