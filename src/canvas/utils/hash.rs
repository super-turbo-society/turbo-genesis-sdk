use std::collections::BTreeMap;

static mut TURBO_FNV1A_REVERSE_LOOKUP: BTreeMap<u64, Vec<u8>> = BTreeMap::new();
pub fn fnv1a(data: &[u8]) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;
    let mut hash = FNV_OFFSET;
    for &byte in data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    #[allow(static_mut_refs)]
    unsafe {
        if !TURBO_FNV1A_REVERSE_LOOKUP.contains_key(&hash) {
            TURBO_FNV1A_REVERSE_LOOKUP.insert(hash, data.to_vec());
        }
    }
    hash
}
pub fn lookup_fnv1a<'a>(hash: u64) -> Option<&'a [u8]> {
    #[allow(static_mut_refs)]
    unsafe {
        TURBO_FNV1A_REVERSE_LOOKUP.get(&hash).map(|v| &**v)
    }
}
