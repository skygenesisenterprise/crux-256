// WebAssembly bindings

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn wasm_encrypt_block(block: &[u8], key: &[u8]) -> Vec<u8> {
    let mut b = [0u8; 32];
    b.copy_from_slice(&block[..32]);
    let mut k = [0u8; 32];
    k.copy_from_slice(&key[..32]);
    crate::encrypt_block(&mut b, &k);
    b.to_vec()
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn wasm_decrypt_block(block: &[u8], key: &[u8]) -> Vec<u8> {
    let mut b = [0u8; 32];
    b.copy_from_slice(&block[..32]);
    let mut k = [0u8; 32];
    k.copy_from_slice(&key[..32]);
    crate::decrypt_block(&mut b, &k);
    b.to_vec()
}