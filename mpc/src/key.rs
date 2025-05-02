use ark_bls12_381::{Fr, G1Affine};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Read, Write};

use crate::errors::MPCError;

/// Represents a participant's share in the MPC wallet
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyShare {
    pub index: u32,
    pub secret_share: Fr,
    pub public_key_share: G1Affine,
}

impl KeyShare {
    pub fn serialize<W: Write>(&self, mut writer: W) -> Result<(), MPCError> {
        self.index
            .serialize_uncompressed(&mut writer)
            .map_err(|e| MPCError::SerializationError(e.to_string()))?;
        self.secret_share
            .serialize_uncompressed(&mut writer)
            .map_err(|e| MPCError::SerializationError(e.to_string()))?;
        self.public_key_share
            .serialize_uncompressed(&mut writer)
            .map_err(|e| MPCError::SerializationError(e.to_string()))?;
        Ok(())
    }

    pub fn deserialize<R: Read>(mut reader: R) -> Result<Self, MPCError> {
        let index = u32::deserialize_uncompressed(&mut reader)
            .map_err(|e| MPCError::SerializationError(e.to_string()))?;
        let secret_share = Fr::deserialize_uncompressed(&mut reader)
            .map_err(|e| MPCError::SerializationError(e.to_string()))?;
        let public_key_share = G1Affine::deserialize_uncompressed(&mut reader)
            .map_err(|e| MPCError::SerializationError(e.to_string()))?;

        Ok(Self {
            index,
            secret_share,
            public_key_share,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ec::AffineRepr;
    #[test]
    fn test_key_share_serialization() {
        let key_share = KeyShare {
            index: 1,
            secret_share: Fr::from(1),
            public_key_share: G1Affine::generator(),
        };
        println!("{:?}", key_share.public_key_share);
        let mut writer = Vec::new();
        key_share.serialize(&mut writer).unwrap();
        let mut reader = writer.as_slice();
        let key_share2 = KeyShare::deserialize(&mut reader).unwrap();
        assert_eq!(key_share, key_share2);
        println!("writer: {:?}", writer);
        println!("reader: {:?}", reader);
    }
}
