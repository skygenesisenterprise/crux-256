// FFI bindings for C/Python integration



#[no_mangle]
pub extern "C" fn crux_encrypt_block(block: *const u8, key: *const u8, output: *mut u8) {
    let mut b = [0u8; 32];
    unsafe {
        std::ptr::copy_nonoverlapping(block, b.as_mut_ptr(), 32);
        let mut k = [0u8; 32];
        std::ptr::copy_nonoverlapping(key, k.as_mut_ptr(), 32);
        crate::encrypt_block(&mut b, &k).unwrap();
        std::ptr::copy_nonoverlapping(b.as_ptr(), output, 32);
    }
}

#[no_mangle]
pub extern "C" fn crux_decrypt_block(block: *const u8, key: *const u8, output: *mut u8) {
    let mut b = [0u8; 32];
    unsafe {
        std::ptr::copy_nonoverlapping(block, b.as_mut_ptr(), 32);
        let mut k = [0u8; 32];
        std::ptr::copy_nonoverlapping(key, k.as_mut_ptr(), 32);
        crate::decrypt_block(&mut b, &k).unwrap();
        std::ptr::copy_nonoverlapping(b.as_ptr(), output, 32);
    }
}