// Utility functions

use std::convert::TryInto;

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