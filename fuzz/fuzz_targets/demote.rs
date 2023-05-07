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

        let key_to_demote = &keys[(i + 1).pow(2) % keys.len()];
        cache.get(key_to_demote);

        if cache.contains(key_to_demote) {
            cache.demote(key_to_demote);
            let (k, _) = cache.pop_lru().unwrap();
            assert_eq!(k, *key_to_demote);
        }
    }
});