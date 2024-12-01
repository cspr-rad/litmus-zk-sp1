use crate::binary::utils::{safe_split_at, CodecError, Decode, Encode};
use ltypes::primitives::bites::{Bytes32, Bytes33, Bytes64, Bytes65};

// ------------------------------------------------------------------------
// Type: [Byte; N].
// ------------------------------------------------------------------------

impl<const N: usize> Decode for [u8; N] {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (bytes, rem) = safe_split_at(bytes, N)?;
        let ptr = bytes.as_ptr() as *const [u8; N];
        let result = unsafe { *ptr };
        Ok((result, rem))
    }
}

impl<const N: usize> Encode for [u8; N] {
    fn get_encoded_size(&self) -> usize {
        N
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(self.as_slice());
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Type: Bytes32.
// ------------------------------------------------------------------------

impl Decode for Bytes32 {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        Decode::decode(bytes).map(|(arr, rem)| (Bytes32::new(arr), rem))
    }
}

impl Encode for Bytes32 {
    fn get_encoded_size(&self) -> usize {
        Bytes32::len()
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(self.as_slice());
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Type: Bytes33.
// ------------------------------------------------------------------------

impl Decode for Bytes33 {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        Decode::decode(bytes).map(|(arr, rem)| (Bytes33::new(arr), rem))
    }
}

impl Encode for Bytes33 {
    fn get_encoded_size(&self) -> usize {
        Bytes33::len()
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(self.as_slice());
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Type: Bytes64.
// ------------------------------------------------------------------------

impl Decode for Bytes64 {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        Decode::decode(bytes).map(|(arr, rem)| (Bytes64::new(arr), rem))
    }
}

impl Encode for Bytes64 {
    fn get_encoded_size(&self) -> usize {
        Bytes64::len()
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(self.as_slice());
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Type: Bytes65.
// ------------------------------------------------------------------------

impl Decode for Bytes65 {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        Decode::decode(bytes).map(|(arr, rem)| (Bytes65::new(arr), rem))
    }
}

impl Encode for Bytes65 {
    fn get_encoded_size(&self) -> usize {
        Bytes65::len()
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(self.as_slice());
        Ok(())
    }
}
