// Key schedule and subkey derivation

/// Derive up to 16 subkeys from master key
pub fn derive_subkeys(key: &[u8; 32]) -> Vec<[u64; 4]> {
    let mut subkeys = Vec::with_capacity(16);
    let mut current: [u64; 4] = key[..].chunks_exact(8).map(|chunk| u64::from_le_bytes(chunk.try_into().unwrap())).collect::<Vec<_>>().try_into().unwrap();
    for _ in 0..16 {
        subkeys.push(current);
        for i in 0..4 {
            current[i] = current[i].rotate_left(7) ^ key[i % 32] as u64;
        }
    }
    subkeys
}

/// Get number of rounds (adaptive)
pub fn num_rounds(key: &[u8; 32]) -> usize {
    8 + (key[0] as usize % 9)
}