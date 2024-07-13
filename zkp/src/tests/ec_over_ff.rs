use ark_bn254::{Fq, G1Projective};
use ark_ec::Group;

// print(G1)
// # (1, 2)

// print(add(G1, G1))

#[test]
fn test_hello_world() {
    let g1 = G1Projective::new(Fq::from(1), Fq::from(2), Fq::from(1));

    println!("g1: {:?}", g1);

    let g1_plus_g1 = g1 + g1;

    println!("g1: {:?}", g1);
    println!("g1 + g1: {:?}", g1_plus_g1);

    // g1 * 2
    let g1_times_2 = g1.mul_bigint([2]);

    println!("g1 * 2: {:?}", g1_times_2);

    assert_eq!(g1_times_2, g1_plus_g1);

    // field moduls:
}
