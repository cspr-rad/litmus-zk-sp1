use super::super::utils::{CodecError, Decode, Encode};
use ltypes::{
    chain::{TransactionV1Hash, TransactionV2Hash},
    crypto::Digest,
};

// ------------------------------------------------------------------------
// Codec: TransactionV1Hash.
// ------------------------------------------------------------------------

impl Decode for TransactionV1Hash {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (inner, remainder) = Digest::decode(bytes).unwrap();

        Ok((TransactionV1Hash::new(inner), remainder))
    }
}

impl Encode for TransactionV1Hash {
    fn get_encoded_size(&self) -> usize {
        self.inner().get_encoded_size()
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.inner().write_bytes(writer).unwrap();
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: TransactionV2Hash.
// ------------------------------------------------------------------------

impl Decode for TransactionV2Hash {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (inner, remainder) = Digest::decode(bytes).unwrap();

        Ok((TransactionV2Hash::new(inner), remainder))
    }
}

impl Encode for TransactionV2Hash {
    fn get_encoded_size(&self) -> usize {
        self.inner().get_encoded_size()
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.inner().write_bytes(writer).unwrap();
        Ok(())
    }
}
