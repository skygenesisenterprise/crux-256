// Padding schemes for variable-length messages

/// PKCS7 padding
pub fn pkcs7_pad(data: &mut Vec<u8>, block_size: usize) {
    let pad_len = block_size - (data.len() % block_size);
    for _ in 0..pad_len {
        data.push(pad_len as u8);
    }
}

/// Remove PKCS7 padding
pub fn pkcs7_unpad(data: &mut Vec<u8>) -> Result<(), &'static str> {
    if data.is_empty() {
        return Err("Data is empty");
    }
    let pad_len = data[data.len() - 1] as usize;
    if pad_len == 0 || pad_len > data.len() {
        return Err("Invalid padding");
    }
    for &byte in data.iter().rev().take(pad_len - 1) {
        if byte != pad_len as u8 {
            return Err("Invalid padding");
        }
    }
    data.truncate(data.len() - pad_len);
    Ok(())
}