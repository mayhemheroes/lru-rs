#![no_main]
use libfuzzer_sys::fuzz_target;
use lru::LruCache;
use std::num::NonZeroUsize;

fuzz_target!(|value: (u8, usize, &[u8])| {
    let (key, size, val) = value;
    if size == 0 || size > 100000 {
        return
    }
    let mut cache = LruCache::new(NonZeroUsize::new(size).unwrap());
    for _ in 0..size {
        cache.put(key, val);
    }
});