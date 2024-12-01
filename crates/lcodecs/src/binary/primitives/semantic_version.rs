use super::super::constants;
use super::super::utils::{CodecError, Decode, Encode};
use ltypes::primitives::semantic_version::SemanticVersion;

// ------------------------------------------------------------------------
// Codec: SemanticVersion.
// ------------------------------------------------------------------------

const ENCODED_SIZE: usize = constants::ENCODED_SIZE_U32 * 3;

impl Decode for SemanticVersion {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (major, bytes) = u32::decode(&bytes).unwrap();
        let (minor, bytes) = u32::decode(&bytes).unwrap();
        let (patch, bytes) = u32::decode(&bytes).unwrap();

        Ok((SemanticVersion::new(major, minor, patch), bytes))
    }
}

impl Encode for SemanticVersion {
    fn get_encoded_size(&self) -> usize {
        ENCODED_SIZE
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.major.write_bytes(writer).unwrap();
        self.minor.write_bytes(writer).unwrap();
        self.patch.write_bytes(writer).unwrap();
        Ok(())
    }
}
