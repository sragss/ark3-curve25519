use crate::{Fq, FqParameters, Fr, FrParameters};
use ark_curve25519_v4::{Fq as Fq4, FqConfig as FqConfig4, Fr as Fr4, FrConfig as FrConfig4};
use ark_ff::{FpParameters, PrimeField};
use ark_ff_v4::{MontConfig, PrimeField as PrimeField4};

#[test]
fn fr_configs() {
    assert_eq!(
        <Fr as PrimeField>::modulus_minus_one_div_two().0,
        <Fr4 as PrimeField4>::MODULUS_MINUS_ONE_DIV_TWO.0
    );
    assert_eq!(FrParameters::GENERATOR.0, FrConfig4::GENERATOR.0 .0);
}

#[test]
fn fr_maths() {
    let three: Fr = Fr::from(10) / Fr::from(500);
    let four: Fr4 = Fr4::from(10) / Fr4::from(500);
    assert_eq!(three.0 .0, four.0 .0);
}

#[test]
fn fq_configs() {
    assert_eq!(
        <Fq as PrimeField>::modulus_minus_one_div_two().0,
        <Fq4 as PrimeField4>::MODULUS_MINUS_ONE_DIV_TWO.0
    );
    assert_eq!(FqParameters::GENERATOR.0, FqConfig4::GENERATOR.0 .0);
}

#[test]
fn fq_maths() {
    let three: Fq = Fq::from(10) / Fq::from(500);
    let four: Fq4 = Fq4::from(10) / Fq4::from(500);
    assert_eq!(three.0 .0, four.0 .0);
}
