use lazy_static::lazy_static;
use num_bigint::{BigUint, ToBigUint};

lazy_static! {
    pub static ref ZERO: BigUint = ToBigUint::to_biguint(&0).unwrap();
    pub static ref ONE: BigUint = ToBigUint::to_biguint(&1).unwrap();
    pub static ref TWO: BigUint = ToBigUint::to_biguint(&2).unwrap();
    pub static ref THREE: BigUint = ToBigUint::to_biguint(&3).unwrap();
    pub static ref FOUR: BigUint = ToBigUint::to_biguint(&4).unwrap();
}
