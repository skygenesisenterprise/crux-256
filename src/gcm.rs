// GCM mode for authenticated encryption

use crate::cipher::encrypt_block;
use crate::errors::Result;

// Simplified GHASH for demonstration (not optimized)
fn ghash(_h: &[u8; 16], data: &[u8]) -> [u8; 16] {
    let mut result = [0u8; 16];
    for chunk in data.chunks(16) {
        for i in 0..16 {
            result[i] ^= chunk.get(i).unwrap_or(&0);
        }
        // Multiply by H (simplified, real GHASH uses GF(2^128))
        // For now, just XOR for demo
    }
    result
}

pub fn gcm_encrypt(key: &[u8; 32], plaintext: &[u8], aad: &[u8], nonce: &[u8; 16]) -> Result<(Vec<u8>, [u8; 16])> {
    // Compute H = E_K(0)
    let mut h_block = [0u8; 32];
    encrypt_block(&mut h_block, key)?;
    let mut h = [0u8; 16];
    h.copy_from_slice(&h_block[0..16]);

    // CTR for encryption
    let ciphertext = crate::modes::ctr_encrypt(plaintext, key, nonce);

    // Compute tag
    let mut auth_data = aad.to_vec();
    auth_data.extend_from_slice(&ciphertext);
    let tag = ghash(&h, &auth_data);

    Ok((ciphertext, tag))
}

pub fn gcm_decrypt(key: &[u8; 32], ciphertext: &[u8], aad: &[u8], nonce: &[u8; 16], tag: &[u8; 16]) -> Result<Vec<u8>> {
    // Compute H
    let mut h_block = [0u8; 32];
    encrypt_block(&mut h_block, key)?;
    let mut h = [0u8; 16];
    h.copy_from_slice(&h_block[0..16]);

    // Verify tag
    let mut auth_data = aad.to_vec();
    auth_data.extend_from_slice(ciphertext);
    let computed_tag = ghash(&h, &auth_data);
    if computed_tag != *tag {
        return Err(crate::errors::CruxError::InvalidTag);
    }

    // CTR for decryption
    let plaintext = crate::modes::ctr_encrypt(ciphertext, key, nonce);
    Ok(plaintext)
}