// Block cipher modes of operation

use crate::cipher::{encrypt_block, decrypt_block};


/// CBC mode encryption
pub fn cbc_encrypt(plaintext: &[u8], key: &[u8; 32], iv: &[u8; 32]) -> Vec<u8> {
    let mut ciphertext = Vec::new();
    let mut prev_block = *iv;
    let mut buffer = [0u8; 32];

    for chunk in plaintext.chunks(32) {
        buffer.copy_from_slice(chunk);
        if chunk.len() < 32 {
            // Assume padded externally
            for i in chunk.len()..32 {
                buffer[i] = 0; // Or handle padding
            }
        }
        for i in 0..32 {
            buffer[i] ^= prev_block[i];
        }
        encrypt_block(&mut buffer, key).unwrap();
        ciphertext.extend_from_slice(&buffer);
        prev_block = buffer;
    }
    ciphertext
}

/// CBC mode decryption
pub fn cbc_decrypt(ciphertext: &[u8], key: &[u8; 32], iv: &[u8; 32]) -> Vec<u8> {
    let mut plaintext = Vec::new();
    let mut prev_block = *iv;
    let mut buffer = [0u8; 32];

    for chunk in ciphertext.chunks(32) {
        buffer.copy_from_slice(chunk);
        let saved_block = buffer;
        decrypt_block(&mut buffer, key).unwrap();
        for i in 0..32 {
            buffer[i] ^= prev_block[i];
        }
        plaintext.extend_from_slice(&buffer);
        prev_block = saved_block;
    }
    plaintext
}

/// CTR mode encryption/decryption (symmetric)
pub fn ctr_encrypt(data: &[u8], key: &[u8; 32], nonce: &[u8; 16]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut counter = [0u8; 16];
    counter[0..16].copy_from_slice(nonce);

    for chunk in data.chunks(32) {
        // Generate keystream
        let mut counter_block = [0u8; 32];
        counter_block[0..16].copy_from_slice(&counter);
        encrypt_block(&mut counter_block, key).unwrap();

        // XOR with data
        for (i, &byte) in chunk.iter().enumerate() {
            result.push(byte ^ counter_block[i]);
        }

        // Increment counter
        for i in (0..16).rev() {
            if counter[i] == 255 {
                counter[i] = 0;
            } else {
                counter[i] += 1;
                break;
            }
        }
    }
    result
}

pub fn ctr_decrypt(ciphertext: &[u8], key: &[u8; 32], nonce: &[u8; 16]) -> Vec<u8> {
    ctr_encrypt(ciphertext, key, nonce) // CTR is symmetric
}