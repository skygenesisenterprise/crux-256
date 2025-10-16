//! # CRUX-256
//!
//! CRUX-256 is an experimental symmetric block cipher with 256-bit blocks and keys,
//! combining Substitution-Permutation Network (SPN) and Add-Rotate-XOR (ARX) architectures.
//! It features dynamic S-boxes, adaptive rounds, and strong bit diffusion.
//!
//! ## Warning
//!
//! This is a research cipher and not cryptographically proven secure. Use at your own risk.
//! For production, prefer established ciphers like AES.

pub mod cipher;
pub mod key_schedule;
pub mod sbox;
pub mod permutation;
pub mod arx;
pub mod utils;
pub mod padding;
pub mod modes;
pub mod ffi;
#[cfg(feature = "wasm")]
pub mod wasm;

pub use cipher::{Crux256, encrypt_block, decrypt_block};
pub use modes::{cbc_encrypt, cbc_decrypt, ctr_encrypt, ctr_decrypt};
pub use padding::{pkcs7_pad, pkcs7_unpad};

#[cfg(test)]
mod tests;