use rand::RngCore;

use crate::ff_ops::{PRIME, mod_add, mod_mul};

/// ```md
/// Shamir’s secret sharing scheme utilises the fact that for any for t+1 points on the two dimensional
/// plane (x1, y1), . . . , (xt+1, yt+1) with unique xi, there exists a unique polynomial q(x) of degree at most t such that q(xi) = yi for every i. Furthermore, it is possible to eﬃciently reconstruct
/// the polynomial q(x), or any specific point on it. One way to do this is with the Lagrange basis
/// polynomials ℓ1(x), . . . , ℓt(x), where reconstruction is carried out by computing q(x) = ∑i=1 t+1 yi ℓi(x).
/// From here on, we will assume that all computations are in the finite field Zp, for a prime p > n.
///
/// Denote the secret and the threshold as s and t, respectively.
///
/// f(x) = s*x^0 + a1*x^1 + a2*x^2 + ... + at*x^t
///
///
///
///
/// ```

pub fn generate_polynomial(secret: u64, threshold: u32) -> Vec<u64> {
    let mut rng = rand::rng();
    let mut coefficients = vec![secret]; // y0

    // Generate random coefficients
    for _ in 0..(threshold - 1) {
        let mut coeff = [0u8; 8];
        rng.fill_bytes(&mut coeff);
        coefficients.push(u64::from_le_bytes(coeff) % PRIME);
    }

    coefficients
}

// Evaluates polynomial at point x
pub fn evaluate_polynomial(coefficients: &[u64], x: u64) -> u64 {
    let mut result = 0u64;
    let mut power = 1u64;

    for &coeff in coefficients {
        result = mod_add(result, mod_mul(coeff, power));
        power = mod_mul(power, x);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_polynomial_generation() {
        let secret = 12345;
        let threshold = 3;
        let polynomial = generate_polynomial(secret, threshold);
        assert_eq!(polynomial.len(), threshold as usize);
    }

    #[test]
    fn test_polynomial_evaluation() {
        let coefficients = vec![1, 2, 3]; // polynomial: 3x² + 2x + 1
        let x = 2;
        let result = evaluate_polynomial(&coefficients, x);
        assert_eq!(result, 17);
    }
}
