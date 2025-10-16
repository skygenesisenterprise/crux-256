# CRUX-256

CRUX-256 is an experimental symmetric block cipher designed for research purposes, combining Substitution-Permutation Network (SPN) and Add-Rotate-XOR (ARX) architectures. It operates on 256-bit blocks with 256-bit keys, featuring dynamic S-boxes, adaptive rounds, and strong bit diffusion.

## Warning

This cipher is not cryptographically proven secure and should not be used in production environments. It is intended for educational and research purposes only. For secure applications, use established standards like AES.

## Features

- 256-bit block and key size
- Dynamic S-boxes generated from the key
- Adaptive number of rounds (8-16)
- Hybrid SPN-ARX architecture for balanced security and performance

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
crux-256 = "0.1.0"
```

### Basic Block Encryption

```rust
use crux_256::{encrypt_block, decrypt_block};

let key = [0u8; 32];
let mut block = [1u8; 32];

encrypt_block(&mut block, &key);
// block is now encrypted

decrypt_block(&mut block, &key);
// block is back to original
```

### Modes for Variable-Length Data

```rust
use crux_256::{cbc_encrypt, cbc_decrypt, ctr_encrypt, ctr_decrypt, pkcs7_pad, pkcs7_unpad};

let key = [0u8; 32];
let iv = [0u8; 32];
let nonce = [0u8; 16];
let mut data = b"Hello, world!".to_vec();

// CBC mode
pkcs7_pad(&mut data, 32);
let ciphertext = cbc_encrypt(&data, &key, &iv);
let mut plaintext = cbc_decrypt(&ciphertext, &key, &iv);
pkcs7_unpad(&mut plaintext).unwrap();

// CTR mode
let ciphertext = ctr_encrypt(b"Hello, world!", &key, &nonce);
let plaintext = ctr_decrypt(&ciphertext, &key, &nonce);
```

### WebAssembly (Web)

Compile with `cargo build --target wasm32-unknown-unknown --features wasm`, then use in JS:

```javascript
import { wasm_encrypt_block, wasm_decrypt_block } from 'crux-256';

const key = new Uint8Array(32);
const block = new Uint8Array(32).fill(1);
const encrypted = wasm_encrypt_block(block, key);
```

### C/Python Integration

Use FFI bindings. For Python, use `ctypes` to call `crux_encrypt_block`.

### Database Example

For encrypting columns in a DB like PostgreSQL, use the FFI to create a UDF.

### Messaging Protocol

For secure messaging, use CTR with key derivation (e.g., HKDF) and AEAD (future GCM mode).

## Building

```bash
cargo build --release
```

## Testing

```bash
cargo test
```

## License

This Project use the MIT Licence [LICENSE](LICENSE)