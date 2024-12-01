use crate::binary::utils::{safe_split_at, CodecError, Decode, Encode};
use ltypes::primitives::bites::{Bytes32, Bytes33, Bytes64, Bytes65};

// ------------------------------------------------------------------------
// Type: [Byte; N].
// ------------------------------------------------------------------------

impl<const N: usize> Decode for [u8; N] {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (bytes, rem) = safe_split_at(bytes, N)?;
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
        Bytes32::len()
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
        Bytes33::len()
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
        Bytes64::len()
    }
}

// ------------------------------------------------------------------------
// Type: Bytes65.
// ------------------------------------------------------------------------

impl Decode for Bytes65 {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        Decode::from_bytes(bytes).map(|(arr, rem)| (Bytes65::new(arr), rem))
    }
}

impl Encode for Bytes65 {
    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        Ok(self.to_vec())
    }

    fn get_encoded_size(&self) -> usize {
        Bytes65::len()
    }
}
