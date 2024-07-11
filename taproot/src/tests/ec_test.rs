use ethereum_types::U256;
use secp256k1::constants::CURVE_ORDER;
use secp256k1::rand::rngs::OsRng;
use secp256k1::SecretKey;

#[test]
fn test_scalar_addition() {
    let a = SecretKey::new(&mut OsRng);

    let b = SecretKey::new(&mut OsRng);

    let a_big_int = U256::from(&a[..]);
    let b_big_int = U256::from(&b[..]);

    // let mut sum = U256::zero();
    // for (i, part) in a_big_int.0.iter().enumerate() {
    //     let part_big_int = U256::from(*part);
    //     let big_exp = U256::from(u64::MAX) + 1;
    //     let part_big_int = part_big_int.mul(big_exp.pow(U256::from(i)));
    //     sum = sum + part_big_int;
    // }
    // assert!(sum == a_big_int);

    println!("a: {:?}", a_big_int);
    println!("b: {:?}", b_big_int);
    let (ab, is_overflowed) = a_big_int.overflowing_add(b_big_int);
    println!("overflow: {:?}", is_overflowed);
    println!("CURVE_ORDER= {:?}", U256::from(CURVE_ORDER));

    match is_overflowed {
        true => {
            return;
        }
        false => {}
    };

    println!("ab: {:?}", ab);

    let mut ab_bytes = [0u8; 32];
    ab.to_big_endian(&mut ab_bytes);

    let a_key = SecretKey::from_slice(&a[..]).unwrap();
    let b_key = SecretKey::from_slice(&b[..]).unwrap();

    let ab_key = SecretKey::from_slice(&ab_bytes).unwrap();

    println!("a: {:?}", U256::from(&a_key[..]));
    println!("b: {:?}", U256::from(&b_key[..]));
    println!("ab: {:?}", U256::from(&ab_key[..]));
}
