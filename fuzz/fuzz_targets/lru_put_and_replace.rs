#![no_main]
use libfuzzer_sys::fuzz_target;
use lru::LruCache;
use std::num::NonZeroUsize;

fuzz_target!(|input: (&[u8], &[u8], usize)| {
    let (keys, values, size) = input;
    if keys.len() == 0 || values.len() == 0 {
        return
    }
    if size == 0 || size > 100000 {
        return
    }
    let mut cache = LruCache::new(NonZeroUsize::new(size).unwrap());
    for i in 0..size {
        cache.put(keys[i.pow(2) % keys.len()], values[i % values.len()]);
        cache.get(&keys[(i + 1).pow(2) % keys.len()]);
        if i % 2 == 0 {
            cache.pop_lru();
        }
    }
});