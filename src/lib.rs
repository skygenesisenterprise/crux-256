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

pub use cipher::{Crux256, encrypt_block, decrypt_block};

#[cfg(test)]
mod tests;