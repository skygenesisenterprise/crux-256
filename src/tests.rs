#[cfg(test)]
mod tests {
    use crate::{encrypt_block, decrypt_block, sbox};

    #[test]
    fn test_encrypt_decrypt() {
        let key = [1u8; 32];
        let mut block = [1u8; 32];
        let original = block;

        encrypt_block(&mut block, &key).unwrap();
        assert_ne!(block, original);

        decrypt_block(&mut block, &key).unwrap();
        assert_eq!(block, original);
    }

    #[test]
    fn test_sbox_generation() {
        let key = [0u8; 32];
        let sbox = sbox::generate_sbox(&key);
        assert_eq!(sbox.len(), 256);
    }

    #[test]
    fn test_sbox_inverse() {
        let key = [1u8; 32];
        let sbox = crate::sbox::generate_sbox(&key);
        let inv_sbox = crate::sbox::generate_inverse_sbox(&sbox);
        let mut data = [0u8; 32];
        for i in 0..32 {
            data[i] = i as u8;
        }
        let original = data;
        crate::sbox::substitute_bytes(&mut data, &sbox);
        crate::sbox::substitute_bytes(&mut data, &inv_sbox);
        assert_eq!(data, original);
    }

    #[test]
    fn test_cbc_encrypt_decrypt() {
        let key = [2u8; 32];
        let iv = [3u8; 32];
        let plaintext = b"Hello, CRUX-256 in CBC mode!";
        let mut padded = plaintext.to_vec();
        crate::pkcs7_pad(&mut padded, 32);
        let ciphertext = crate::cbc_encrypt(&padded, &key, &iv);
        let decrypted = crate::cbc_decrypt(&ciphertext, &key, &iv);
        crate::pkcs7_unpad(&mut decrypted.clone()).unwrap();
        assert_eq!(&decrypted[..plaintext.len()], plaintext);
    }

    #[test]
    fn test_ctr_encrypt_decrypt() {
        let key = [4u8; 32];
        let nonce = [5u8; 16];
        let plaintext = b"CRUX-256 CTR mode test";
        let ciphertext = crate::ctr_encrypt(plaintext, &key, &nonce);
        let decrypted = crate::ctr_decrypt(&ciphertext, &key, &nonce);
        assert_eq!(decrypted, plaintext);
    }

    // Add more tests for each module
}