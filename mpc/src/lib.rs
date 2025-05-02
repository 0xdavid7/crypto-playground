use ark_bls12_381::{Fr, G1Affine, G1Projective};
use ark_ec::CurveGroup;
use ark_ec::PrimeGroup;
use ark_std::rand::RngCore;
use ark_std::UniformRand;
use errors::MPCError;
use key::KeyShare;

mod errors;
mod key;
mod sig;

#[derive(Debug)]
pub struct MPCWallet {
    pub threshold: usize,
    pub total_participants: usize,
    pub public_key: G1Affine,
}

impl MPCWallet {
    pub fn keygen<R: RngCore>(
        threshold: usize,
        total_participants: usize,
        rng: &mut R,
    ) -> Result<(Self, Vec<KeyShare>), MPCError> {
        // Validate the parameters
        if total_participants < 2 {
            return Err(MPCError::InvalidParticipants(total_participants));
        }

        if threshold > total_participants || threshold < 1 {
            return Err(MPCError::InvalidThreshold(threshold));
        }

        // Generate a random polynomial of degree threshold - 1
        // f(x) = a_0 + a_1*x + a_2*x^2 + ... + a_{t-1}*x^{t-1}
        let mut coefficients = Vec::with_capacity(threshold);
        for _ in 0..threshold {
            coefficients.push(Fr::rand(rng));
        }

        // The constant term a_0 is the secret key
        let secret_key = coefficients[0];

        // Generate shares for each participant
        let mut shares = Vec::with_capacity(total_participants);

        for i in 1..=total_participants {
            let index = i as u32;
            let x = Fr::from(index);

            // Evaluate the polynomial at point x
            let mut secret_share = coefficients[0];
            let mut x_pow = x;

            // secret_share = f(x) = Σ(ai * x^i) for i = [0, t)

            for j in 1..threshold {
                let term = coefficients[j] * x_pow;
                secret_share += term;
                x_pow *= x;
            }

            // Calculate the public key share g^{secret_share}
            let generator = G1Projective::generator();
            let public_key_share = (generator * secret_share).into_affine();

            shares.push(KeyShare {
                index,
                secret_share,
                public_key_share,
            });
        }

        // Calculate the master public key g^{secret_key}
        let generator = G1Projective::generator();
        let public_key = (generator * secret_key).into_affine();

        Ok((
            Self {
                threshold,
                total_participants,
                public_key,
            },
            shares,
        ))
    }
}

#[cfg(test)]
mod tests {
    use ark_std::test_rng;

    use super::*;
    #[test]
    fn test_keygen() {
        let threshold = 3;
        let total_participants = 5;
        let mut rng = test_rng();
        let (wallet, shares) = MPCWallet::keygen(threshold, total_participants, &mut rng)
            .expect("Keygen should succeed in test");
        assert_eq!(wallet.threshold, threshold);
        assert_eq!(wallet.total_participants, total_participants);
        assert_eq!(shares.len(), total_participants);
        println!("wallet: {:?}", wallet);
        println!("shares: {:?}", shares);
    }
}
