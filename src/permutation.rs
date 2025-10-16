// Bit permutation for diffusion

/// Permute words in a 256-bit block (4 u64) for diffusion
/// Simple rotation: shift left by 1 word
pub fn permute_words(words: &mut [u64; 4]) {
    let temp = words[0];
    words[0] = words[1];
    words[1] = words[2];
    words[2] = words[3];
    words[3] = temp;
}

/// Inverse permutation for decryption
pub fn inverse_permute_words(words: &mut [u64; 4]) {
    let temp = words[3];
    words[3] = words[2];
    words[2] = words[1];
    words[1] = words[0];
    words[0] = temp;
}