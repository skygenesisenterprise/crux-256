// Traits for modularity

pub trait BlockCipher {
    const BLOCK_SIZE: usize;
    const KEY_SIZE: usize;

    fn encrypt_block(&self, block: &mut [u8]);
    fn decrypt_block(&self, block: &mut [u8]);
}

pub trait AuthenticatedEncryption {
    fn encrypt_aead(&self, plaintext: &[u8], aad: &[u8], nonce: &[u8]) -> (Vec<u8>, Vec<u8>);
    fn decrypt_aead(&self, ciphertext: &[u8], aad: &[u8], nonce: &[u8], tag: &[u8]) -> Result<Vec<u8>, &'static str>;
}