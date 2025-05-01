use std::collections::HashMap;

mod ff_ops;
use ff_ops::*;
mod polynomial;
use polynomial::*;

// Generates shares for all parties
pub fn generate_shares(secret: u64, threshold: u32, total_shares: u32) -> HashMap<u32, u64> {
    let polynomial = generate_polynomial(secret, threshold);
    let mut shares = HashMap::new();

    for id in 1..=total_shares {
        let share = evaluate_polynomial(&polynomial, id as u64);
        shares.insert(id, share);
    }

    shares
}

// Lagrange interpolation for secret reconstruction
///
/// ### Lagrange formula
///
/// f(x) = Σ(yi * li(x))
///
/// where li(x) is the lagrange basis polynomial:
///
/// li(x) = Π((x - xj) / (xi - xj)) for j != i
///
/// so:
///
/// secret = f(0) = Σ(yi * li(x)) for i = 1..t+1 with x=0
///
/// where t is the threshold and yi is the share for party i
///
/// if dont know how the code works, write the formula in paper form and solve for x=0
///
///
pub fn reconstruct_secret(shares: &HashMap<u32, u64>) -> u64 {
    let mut secret = 0u64;

    for (&i, &share_i) in shares {
        let mut lagrange = 1u64;

        for (&j, _) in shares {
            if i != j {
                // Calculate lagrange basis polynomial
                // li(x) = Π((x - xj) / (xi - xj)) for j!= i
                // li(0) = Π((0 - xj) / (xi - xj)) for j!= i
                // li(0) = Π((-xj) / (xi - xj)) for j!= i

                // We can calculate the formula above in two ways:
                // 1
                let num = PRIME - j as u64;
                let den = mod_sub(i as u64, j as u64);
                let inverse = mod_inv(den);
                lagrange = mod_mul(lagrange, mod_mul(num, inverse));

                // 2
                // let num = j as u64;
                // let den = mod_sub(j as u64, i as u64);
                // let inverse = mod_inv(den);
                // lagrange = mod_mul(lagrange, mod_mul(num, inverse));
            }
        }

        secret = mod_add(secret, mod_mul(share_i, lagrange));
    }

    secret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_share_generation_and_reconstruction() {
        let threshold = 3;
        let total_shares = 5;

        for secret in 1..PRIME - 1 {
            let shares = generate_shares(secret, threshold, total_shares);
            println!("Shares: {:?}", shares);
            assert_eq!(shares.len(), total_shares as usize);

            // Test reconstruction with exactly threshold shares
            let mut threshold_shares: HashMap<u32, u64> = HashMap::new();
            for i in 1..=threshold {
                threshold_shares.insert(i, *shares.get(&i).unwrap());
            }

            let reconstructed = reconstruct_secret(&threshold_shares);
            assert_eq!(reconstructed, secret);
        }
    }
}
