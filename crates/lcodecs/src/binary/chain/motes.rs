use crate::binary::utils::{CodecError, Decode, Encode};
use ltypes::chain::Motes;

// ------------------------------------------------------------------------
// Codec: Motes.
// ------------------------------------------------------------------------

impl Decode for Motes {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (inner, bytes) = u64::decode(&bytes).unwrap();

        Ok((Self::new(inner), &bytes))
    }
}

impl Encode for Motes {
    fn get_encoded_size(&self) -> usize {
        self.inner().get_encoded_size()
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.inner().write_bytes(writer).unwrap();
        Ok(())
    }
}
