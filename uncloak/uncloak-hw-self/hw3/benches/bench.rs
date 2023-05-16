//! Benches. To use, import functions of interest, and `cargo bench`.
//!
//! https://bheisler.github.io/criterion.rs/book/iai/iai.html
#![allow(unused_imports)]
use std::{thread, time};

use criterion::{black_box as bb, criterion_group, criterion_main, Criterion};
use ring::digest::{SHA1_FOR_LEGACY_USE_ONLY, SHA256};
use sha3::{Digest, Sha3_256};

const TEXT: &[u8] = b"some text";

// My machine:
// 86ns
fn bench_blake(c: &mut Criterion) {
  c.bench_function("blake3", |g| {
    g.iter(|| {
      let mut hasher = blake3::Hasher::new();
      hasher.update(bb(TEXT));
      hasher.finalize();
    })
  });
}

// 256ns
fn bench_sha(c: &mut Criterion) {
  c.bench_function("sha2", |g| {
    g.iter(|| {
      let hasher = ring::digest::digest(&SHA256, bb(TEXT));
      hasher
    });
  });
}

// 630ns
fn bench_sha3(c: &mut Criterion) {
  c.bench_function("sha3", |g| {
    g.iter(|| {
      let mut hasher = Sha3_256::new();
      hasher.update(bb(TEXT));
      hasher.finalize();
    });
  });
}

fn bench_sha1(c: &mut Criterion) {
  c.bench_function("shit_hash", |g| {
    g.iter(|| {
      let hasher = ring::digest::digest(&SHA1_FOR_LEGACY_USE_ONLY, bb(TEXT));
      hasher
    });
  });
}
criterion_group!(my_benches, bench_blake, bench_sha, bench_sha3, bench_sha1);
criterion_main!(my_benches);
