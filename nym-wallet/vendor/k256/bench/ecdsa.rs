//! secp256k1 scalar arithmetic benchmarks

use criterion::{criterion_group, criterion_main, Criterion};
use ecdsa_core::{
    elliptic_curve::group::prime::PrimeCurveAffine,
    hazmat::{SignPrimitive, VerifyPrimitive},
};
use k256::{
    elliptic_curve::{generic_array::arr, group::ff::PrimeField},
    AffinePoint, Scalar,
};

fn test_scalar_d() -> Scalar {
    Scalar::from_repr(arr![u8;
        0xbb, 0x48, 0x8a, 0xef, 0x41, 0x6a, 0x41, 0xd7, 0x68, 0x0d, 0x1c, 0xf0, 0x1d, 0x70,
        0xf5, 0x9b, 0x60, 0xd7, 0xf5, 0xf7, 0x7e, 0x30, 0xe7, 0x8b, 0x8b, 0xf9, 0xd2, 0xd8,
        0x82, 0xf1, 0x56, 0xa6,
    ])
    .unwrap()
}

fn test_scalar_k() -> Scalar {
    Scalar::from_repr(arr![u8;
        0x67, 0xe2, 0xf6, 0x80, 0x71, 0xed, 0x82, 0x81, 0xe8, 0xae, 0xd6, 0xbc, 0xf1, 0xc5,
        0x20, 0x7c, 0x5e, 0x63, 0x37, 0x22, 0xd9, 0x20, 0xaf, 0xd6, 0xae, 0x22, 0xd0, 0x6e,
        0xeb, 0x80, 0x35, 0xe3,
    ])
    .unwrap()
}

fn test_scalar_z() -> Scalar {
    Scalar::from_repr(arr![u8;
        0xe3, 0x35, 0x80, 0xeb, 0x6e, 0xd0, 0x22, 0xae, 0xd6, 0xaf, 0x20, 0xd9, 0x22, 0x37,
        0x63, 0x5e, 0x7c, 0x20, 0xc5, 0xf1, 0xbc, 0xd6, 0xae, 0xe8, 0x81, 0x82, 0xed, 0x71,
        0x80, 0xf6, 0xe2, 0x67
    ])
    .unwrap()
}

fn bench_ecdsa(c: &mut Criterion) {
    let mut group = c.benchmark_group("ecdsa");

    let d = test_scalar_d();
    let k = test_scalar_k();
    let z = test_scalar_z();

    group.bench_function("try_sign_prehashed", |b| {
        b.iter(|| d.try_sign_prehashed(k, z).unwrap())
    });

    let q = (AffinePoint::generator() * d).to_affine();
    let s = d.try_sign_prehashed(k, z).unwrap().0;

    group.bench_function("verify_prehashed", |b| {
        b.iter(|| q.verify_prehashed(z, &s).unwrap())
    });

    group.finish();
}

criterion_group!(benches, bench_ecdsa);
criterion_main!(benches);
