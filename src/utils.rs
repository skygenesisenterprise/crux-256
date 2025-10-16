// Utility functions

use std::convert::TryInto;
use rand::Rng;

/// Convert 32 bytes to 4 u64
pub fn bytes_to_words(bytes: &[u8; 32]) -> [u64; 4] {
    bytes.chunks_exact(8).map(|chunk| u64::from_le_bytes(chunk.try_into().unwrap())).collect::<Vec<_>>().try_into().unwrap()
}

/// Convert 4 u64 to 32 bytes
pub fn words_to_bytes(words: &[u64; 4]) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    for (i, &word) in words.iter().enumerate() {
        bytes[i*8..(i+1)*8].copy_from_slice(&word.to_le_bytes());
    }
    bytes
}

/// Generate secure random IV (32 bytes for CBC)
pub fn generate_iv() -> [u8; 32] {
    let mut rng = rand::thread_rng();
    let mut iv = [0u8; 32];
    rng.fill(&mut iv);
    iv
}

/// Generate secure random nonce (16 bytes for CTR/GCM)
pub fn generate_nonce() -> [u8; 16] {
    let mut rng = rand::thread_rng();
    let mut nonce = [0u8; 16];
    rng.fill(&mut nonce);
    nonce
}