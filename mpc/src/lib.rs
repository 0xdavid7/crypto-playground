use std::collections::HashMap;

use ark_bls12_381::{Fr, G1Affine, G1Projective};
use ark_ec::{CurveGroup, PrimeGroup};
use ark_ff::{Field, One, Zero};
use ark_std::{
    rand::{rngs, RngCore, SeedableRng},
    UniformRand,
};
use errors::MPCError;
use key::KeyShare;
use sha2::{Digest, Sha256};
use sig::SignatureShare;

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
        // TODO: Validate the parameters
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

        // The constant term f(0) is the secret key
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

    pub fn sign_share(message: &[u8], key_share: &KeyShare) -> SignatureShare {
        let message_point = Self::hash_to_curve(message);

        let sig_share = (message_point * key_share.secret_share).into_affine();

        SignatureShare {
            index: key_share.index,
            share: sig_share,
        }
    }

    pub fn combine_signature_shares(
        &self,
        shares: &[SignatureShare],
    ) -> Result<G1Affine, MPCError> {
        if shares.len() < self.threshold {
            return Err(MPCError::InsufficientShares);
        }

        let shares_to_use = &shares[0..self.threshold];

        let mut lagrange_coefficients = HashMap::new();

        for i in 0..self.threshold {
            let idx_i = Fr::from(shares_to_use[i].index);
            let mut lambda_i = Fr::one();

            for j in 0..self.threshold {
                if i != j {
                    let idx_j = Fr::from(shares_to_use[j].index);
                    let mut temp = idx_j;
                    temp -= &idx_i;
                    temp = temp.inverse().unwrap();
                    temp *= &idx_j;
                    lambda_i *= &temp;
                }
            }

            lagrange_coefficients.insert(shares_to_use[i].index, lambda_i);
        }

        // Combine the shares using the Lagrange coefficients
        let mut combined_sig = G1Projective::zero();

        for share in shares_to_use {
            let lambda = lagrange_coefficients.get(&share.index).unwrap();
            combined_sig += &(G1Projective::from(share.share) * lambda);
        }

        Ok(combined_sig.into_affine())
    }

    pub fn verify(
        public_key: &G1Affine,
        message: &[u8],
        signature: &G1Affine,
    ) -> Result<(), MPCError> {
        let message_point = Self::hash_to_curve(message);

        let message_proj = message_point;
        let signature_proj = G1Projective::from(*signature);

        // In a true BLS signature scheme, we would use pairing checks
        if !signature.is_on_curve() || !signature.is_in_correct_subgroup_assuming_on_curve() {
            return Err(MPCError::VerificationFailed);
        }

        // A simplified check: verify signature is not the identity
        if signature_proj.is_zero() {
            return Err(MPCError::VerificationFailed);
        }

        // TODO: Implement the actual BLS signature verification

        Ok(())
    }

    fn hash_to_curve(message: &[u8]) -> G1Projective {
        let mut hasher = Sha256::new();
        hasher.update(message);
        let hash_result = hasher.finalize();

        // WARN: Just generate a random point - this is not secure, just for illustration
        let mut seed = [0u8; 32];
        seed.copy_from_slice(&hash_result);

        let mut rng = rngs::StdRng::from_seed(seed);

        G1Projective::generator() * Fr::rand(&mut rng)
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

    #[test]
    fn test_hash_to_curve() {
        let message = b"hello world";
        let point = MPCWallet::hash_to_curve(message);
        println!("point: {:?}", point);
    }

    #[test]
    fn test_sign_share() {
        let threshold = 3;
        let total_participants = 5;
        let mut rng = test_rng();
        let (wallet, shares) = MPCWallet::keygen(threshold, total_participants, &mut rng)
            .expect("Keygen should succeed in test");

        // Message to sign
        let message = b"Send 1 BTC to Alice";

        // Two participants sign the message
        let sig_share1 = MPCWallet::sign_share(message, &shares[0]);
        let sig_share2 = MPCWallet::sign_share(message, &shares[1]);
        let sig_share3 = MPCWallet::sign_share(message, &shares[2]);

        // Combine the signature shares
        let signature = wallet
            .combine_signature_shares(&[sig_share1, sig_share2, sig_share3])
            .map_err(|e| {
                println!("Error combining signature shares: {:?}", e);
                e
            })
            .unwrap();

        println!("Generated signature: {:?}", signature);
    }
}
