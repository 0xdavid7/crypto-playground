use std::collections::HashMap;

use super::KeyShare;
use super::MPCError;
use super::SignatureShare;
use ark_bls12_381::{Bls12_381, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{pairing::Pairing, AffineRepr, CurveGroup, PrimeGroup};
use ark_ff::{Field, One, PrimeField, Zero};
use ark_std::{rand::RngCore, UniformRand};
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct MPCWallet {
    pub threshold: usize,
    pub total_participants: usize,
    pub public_key: G2Affine,
}

impl MPCWallet {
    pub fn keygen<R: RngCore>(
        threshold: usize,
        total_participants: usize,
        rng: &mut R,
    ) -> Result<(Self, Vec<KeyShare>), MPCError> {
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

            // the scope below is equivalent to:
            // secret_share = f(x) = Σ(ai * x^i) for i = [0, t) // just for random polynomial simulation
            for item in coefficients.iter().skip(1) {
                let term = *item * x_pow;
                secret_share += term;
                x_pow *= x;
            }

            // Calculate the public key share g^{secret_share}
            let generator = G2Projective::generator();
            // Aggreted_Public_Key = g^{secret_key}
            let public_key_share = (generator * secret_share).into_affine();

            shares.push(KeyShare {
                index,
                secret_share,
                public_key_share,
            });
        }

        // Calculate the master public key g^{secret_key}
        let generator = G2Projective::generator();
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

        // Calculate the signature share: sig_i = H(msg) * secret_share with msg, secret_share, and g are all points on the curve G1 -> sig is a point on the curve G1
        let sig_share = (message_point * key_share.secret_share).into_affine();

        SignatureShare {
            index: key_share.index,
            sig: sig_share,
        }
    }

    pub fn verify(
        public_key: &G2Affine,
        message: &[u8],
        signature: &G1Affine,
    ) -> Result<(), MPCError> {
        let message_point = Self::hash_to_curve(message).into_affine();

        let g2_generator = G2Affine::generator();

        // Verify the signature using the public key and the message
        // e(sig ∈ G1, g2 ∈ G2) == e(H(msg) ∈ G1, pk ∈ G2), with e is the pairing function

        let lhs = Bls12_381::pairing(*signature, g2_generator);
        let rhs = Bls12_381::pairing(message_point, *public_key);

        if lhs == rhs {
            Ok(())
        } else {
            Err(MPCError::VerificationFailed)
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

            // li(x) = Π((x - xj) / (xi - xj)) for j != i
            for j in 0..self.threshold {
                // this stuff is equivalent to:
                // (-1) * li(x) = (-1) * Π((-xj) / (xi - xj)) for j!= i
                // -li(x) = Π((xj) / (xi - xj)) for j!= i
                if i != j {
                    let idx_j = Fr::from(shares_to_use[j].index);
                    let mut temp = idx_j;
                    temp -= &idx_i;
                    temp = temp.inverse().unwrap();
                    temp *= &idx_j;
                    lambda_i *= &temp;
                }
            }

            // Store the Langrange coefficient for the party i
            // party_1 -> l1
            // party_2 -> l2
            // party_3 -> l3
            // ...
            lagrange_coefficients.insert(shares_to_use[i].index, lambda_i);
        }

        // Combine the shares using the Lagrange coefficients
        let mut combined_sig = G1Projective::zero();

        // combined_sig = Σ(sig_i * li(x)) for i = [1, t]
        // -> combined_sig is representing the point that is the sum of all the shares multiplied by the Lagrange coefficients
        // imagine that:
        // f_secret(x) = Σ(share_i * li(x)) for i = [0, t], share_i is the secret if i = 0
        // correlated to:
        // f_signature(x) = Σ(sig_i * li(x)) for i = [0, t]
        for share in shares_to_use {
            let lambda = lagrange_coefficients.get(&share.index).unwrap();
            combined_sig += &(G1Projective::from(share.sig) * lambda);
        }

        Ok(combined_sig.into_affine())
    }

    fn hash_to_curve(message: &[u8]) -> G1Projective {
        let hash = Sha256::digest(message);
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&hash[..32]);
        let scalar = Fr::from_le_bytes_mod_order(&bytes);
        G1Projective::generator() * scalar
    }
}

#[cfg(test)]
mod tests {
    use ark_bls12_381::G2Projective;
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
    fn test_sign_and_verify() {
        let mut rng = test_rng();

        // Generate a random key
        let sk = Fr::rand(&mut rng);
        let pk = (G2Projective::generator() * sk).into_affine();

        let key_share = KeyShare {
            index: 0,
            secret_share: sk,
            public_key_share: pk,
        };

        let message = b"hello MPC BLS!";
        let sig = MPCWallet::sign_share(message, &key_share);

        let result = MPCWallet::verify(&pk, message, &sig.sig);
        assert!(result.is_ok(), "Signature should verify");
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

        // Verify the signature
        let result = MPCWallet::verify(&wallet.public_key, message, &signature)
            .map_err(|e| println!("Error verifying signature: {:?}", e));
        assert!(result.is_ok(), "Signature should verify");
    }
}
