// How bilinear pairings work
// When a scalar  is multiplied by a point, the result is a point
// That is P = p * G, G is a generator point
// Assumme pg = r
// pG = P
// qG = Q
// rG = R
// then we want a function such that
// f(P, Q) = f(R, G)
//
// Bilinear means:
// If f(x,y) is a bilinear function and c is constant, then z = f(x, c) varies linearly with x,
// and z = f(c, y) varies linearly with y.
// So we can infer from this that an EC bilinear pairing has the following properties:
// f(aG, bG) = f(abG, G) = f(G, abG)

// What is e(P, Q) returning?
// The output of a bilinear pairing is a group element, specifically an element of a finite cyclic group.
// It’s best to treat GT as a black box similar to how most programmers treat hash functions like black boxes.
// However, despite it being a black box, we still know a lot about the properties of the output, which we call
// GT:
// GT is a cyclic group, so it has a closed binary operator.
// The binary operator of
// GT is associative.
// GT has an identity element.
// Each element in GT has an inverse.
// Because the group is cyclic, it has a generator.
// Because the group is cyclic and finite, then finite cyclic groups are homomorphic to GT
// That is, we have some way to homomorphically map elements in a finite field to elements in GT
// GT is a 12-dimensional vector space over the field Fp

// e(aG1, bG2) = e(abG1, G2) = e(G1, abG2)

#[cfg(test)]
mod tests {
    use ark_bls12_381::{Bls12_381, Fr, G1Projective, G2Projective};
    use ark_ec::{CurveGroup, PrimeGroup, pairing::Pairing};

    #[test]
    fn test_mul() {
        // e(aG1, bG2) = e(cG1, dG2) with a*b = c*d

        let a = G1Projective::generator() * Fr::from(3);
        let b = G2Projective::generator() * Fr::from(4);

        let c = G1Projective::generator() * Fr::from(6);
        let d = G2Projective::generator() * Fr::from(2);

        let ab = Bls12_381::pairing(a.into_affine(), b.into_affine());
        let cd = Bls12_381::pairing(c.into_affine(), d.into_affine());
        assert_eq!(ab, cd);
    }

    #[test]
    fn test_pairing() {
        let a = G1Projective::generator() * Fr::from(3);
        let b = G2Projective::generator() * Fr::from(4);
        let ab = Bls12_381::pairing(a.into_affine(), b.into_affine());

        let c = G1Projective::generator() * Fr::from(5);
        let d = G2Projective::generator() * Fr::from(2);
        let cd = Bls12_381::pairing(c.into_affine(), d.into_affine());
        let abcd = ab.0 * cd.0;

        // 3*4 + 5*2 = 22

        let e = G1Projective::generator() * Fr::from(2);
        let f = G2Projective::generator() * Fr::from(11);
        let ef = Bls12_381::pairing(e.into_affine(), f.into_affine());
        assert_eq!(abcd, ef.0);

        println!("ABCD: {}", abcd);
        println!("=========");
        println!("EF: {}", ef.0);
    }
}
