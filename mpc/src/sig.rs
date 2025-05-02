use ark_bls12_381::G1Affine;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Read, Write};

use crate::errors::MPCError;

#[derive(Clone, Debug, PartialEq, Eq, CanonicalDeserialize, CanonicalSerialize)]
pub struct SignatureShare {
    pub index: u32,
    pub share: G1Affine,
}

impl SignatureShare {
    pub fn serialize<W: Write>(&self, mut writer: W) -> Result<(), MPCError> {
        self.index
            .serialize_uncompressed(&mut writer)
            .map_err(|e| MPCError::SerializationError(e.to_string()))?;
        self.share
            .serialize_uncompressed(&mut writer)
            .map_err(|e| MPCError::SerializationError(e.to_string()))?;
        Ok(())
    }

    pub fn deserialize<R: Read>(mut reader: R) -> Result<Self, MPCError> {
        let index = u32::deserialize_uncompressed(&mut reader)
            .map_err(|e| MPCError::SerializationError(e.to_string()))?;
        let share = G1Affine::deserialize_uncompressed(&mut reader)
            .map_err(|e| MPCError::SerializationError(e.to_string()))?;

        Ok(Self { index, share })
    }
}
