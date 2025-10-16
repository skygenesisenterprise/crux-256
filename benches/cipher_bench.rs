use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crux_256::{encrypt_block, decrypt_block, cbc_encrypt, cbc_decrypt, ctr_encrypt, ctr_decrypt};

fn bench_encrypt_block(c: &mut Criterion) {
    let key = [0u8; 32];
    let mut block = [1u8; 32];
    c.bench_function("encrypt_block", |b| b.iter(|| encrypt_block(black_box(&mut block), black_box(&key))));
}

fn bench_decrypt_block(c: &mut Criterion) {
    let key = [0u8; 32];
    let mut block = [1u8; 32];
    c.bench_function("decrypt_block", |b| b.iter(|| decrypt_block(black_box(&mut block), black_box(&key))));
}

fn bench_cbc_encrypt(c: &mut Criterion) {
    let key = [0u8; 32];
    let iv = [0u8; 32];
    let data = vec![1u8; 1024];
    c.bench_function("cbc_encrypt_1kb", |b| b.iter(|| cbc_encrypt(black_box(&data), black_box(&key), black_box(&iv))));
}

fn bench_ctr_encrypt(c: &mut Criterion) {
    let key = [0u8; 32];
    let nonce = [0u8; 16];
    let data = vec![1u8; 1024];
    c.bench_function("ctr_encrypt_1kb", |b| b.iter(|| ctr_encrypt(black_box(&data), black_box(&key), black_box(&nonce))));
}

criterion_group!(benches, bench_encrypt_block, bench_decrypt_block, bench_cbc_encrypt, bench_ctr_encrypt);
criterion_main!(benches);