#[cfg(test)]
mod tests {
    use crate::{encrypt_block, decrypt_block, sbox};

    #[test]
    fn test_encrypt_decrypt() {
        let key = [1u8; 32];
        let mut block = [1u8; 32];
        let original = block;

        encrypt_block(&mut block, &key);
        assert_ne!(block, original);

        decrypt_block(&mut block, &key);
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

    // Add more tests for each module
}