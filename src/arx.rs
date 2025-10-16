// Add-Rotate-XOR operations



/// ARX mix on 4 u64 words (256 bits)
pub fn arx_mix(block: &mut [u64; 4], subkey: &[u64; 4]) {
    for i in 0..4 {
        block[i] = block[i].wrapping_add(subkey[i]);
        block[i] = block[i].rotate_left(13); // ChaCha20 style rotation
        block[i] ^= subkey[(i + 1) % 4];
    }
}

/// Inverse ARX for decryption
pub fn inverse_arx_mix(block: &mut [u64; 4], subkey: &[u64; 4]) {
    for i in (0..4).rev() {
        block[i] ^= subkey[(i + 1) % 4];
        block[i] = block[i].rotate_right(13);
        block[i] = block[i].wrapping_sub(subkey[i]);
    }
}