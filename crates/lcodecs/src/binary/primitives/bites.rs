use crate::binary::utils::{safe_split_at, CodecError, Decode, Encode};
use ltypes::primitives::bites::{Bytes32, Bytes33, Bytes64, SIZE_32, SIZE_33, SIZE_64};

// ------------------------------------------------------------------------
// Type: [Byte; N].
// ------------------------------------------------------------------------

impl<const N: usize> Decode for [u8; N] {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (bytes, rem) = safe_split_at(bytes, N)?;
        // SAFETY: safe_split_at makes sure `bytes` is exactly `COUNT` bytes.
        let ptr = bytes.as_ptr() as *const [u8; N];
        let result = unsafe { *ptr };
        Ok((result, rem))
    }
}

impl<const N: usize> Encode for [u8; N] {
    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        Ok(self.to_vec())
    }

    fn get_encoded_size(&self) -> usize {
        N
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(&self.as_slice());
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Type: Bytes32.
// ------------------------------------------------------------------------

impl Decode for Bytes32 {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        Decode::from_bytes(bytes).map(|(arr, rem)| (Bytes32::new(arr), rem))
    }
}

impl Encode for Bytes32 {
    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        Ok(self.to_vec())
    }

    fn get_encoded_size(&self) -> usize {
        SIZE_32
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(&self.as_slice());
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Type: Bytes33.
// ------------------------------------------------------------------------

impl Decode for Bytes33 {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        Decode::from_bytes(bytes).map(|(arr, rem)| (Bytes33::new(arr), rem))
    }
}

impl Encode for Bytes33 {
    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        Ok(self.to_vec())
    }

    fn get_encoded_size(&self) -> usize {
        SIZE_33
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(&self.as_slice());
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Type: Bytes64.
// ------------------------------------------------------------------------

impl Decode for Bytes64 {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        Decode::from_bytes(bytes).map(|(arr, rem)| (Bytes64::new(arr), rem))
    }
}

impl Encode for Bytes64 {
    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        Ok(self.to_vec())
    }

    fn get_encoded_size(&self) -> usize {
        SIZE_64
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(&self.as_slice());
        Ok(())
    }
}
