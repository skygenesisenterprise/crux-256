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

Example:

```rust
use crux_256::{encrypt_block, decrypt_block};

let key = [0u8; 32];
let mut block = [1u8; 32];

encrypt_block(&mut block, &key);
// block is now encrypted

decrypt_block(&mut block, &key);
// block is back to original
```

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