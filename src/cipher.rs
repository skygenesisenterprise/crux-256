// Core cipher implementation

use crate::sbox::{generate_sbox, substitute_bytes, generate_inverse_sbox};
use crate::permutation::{permute_words, inverse_permute_words};
use crate::arx::{arx_mix, inverse_arx_mix};
use crate::key_schedule::{derive_subkeys, num_rounds};
use crate::utils::{bytes_to_words, words_to_bytes};

/// Encrypt a 256-bit block with 256-bit key
pub fn encrypt_block(block: &mut [u8; 32], key: &[u8; 32]) {
    let sbox = generate_sbox(key);
    let subkeys = derive_subkeys(key);
    let rounds = num_rounds(key);

    for round in 0..rounds {
        substitute_bytes(block, &sbox);
        let mut words = bytes_to_words(block);
        permute_words(&mut words);
        arx_mix(&mut words, &subkeys[round]);
        *block = words_to_bytes(&words);
        for i in 0..32 {
            block[i] ^= key[i % 32];
        }
    }
}

/// Decrypt a 256-bit block with 256-bit key
pub fn decrypt_block(block: &mut [u8; 32], key: &[u8; 32]) {
    let sbox = generate_sbox(key);
    let inv_sbox = generate_inverse_sbox(&sbox);
    let subkeys = derive_subkeys(key);
    let rounds = num_rounds(key);

    for round in (0..rounds).rev() {
        for i in 0..32 {
            block[i] ^= key[i % 32];
        }
        let mut words = bytes_to_words(block);
        inverse_arx_mix(&mut words, &subkeys[round]);
        inverse_permute_words(&mut words);
        *block = words_to_bytes(&words);
        substitute_bytes(block, &inv_sbox);
    }
}

/// Crux256 cipher struct for stateful operations if needed
pub struct Crux256 {
    key: [u8; 32],
}

impl Crux256 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn encrypt(&self, block: &mut [u8; 32]) {
        encrypt_block(block, &self.key);
    }

    pub fn decrypt(&self, block: &mut [u8; 32]) {
        decrypt_block(block, &self.key);
    }
}