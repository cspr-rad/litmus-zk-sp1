use super::utils::{safe_split_at, CodecError, Decode, Encode};
use lutils::bites::{Byte, Bytes32, Bytes33, Bytes64, SIZE_32, SIZE_33, SIZE_64};

// ------------------------------------------------------------------------
// Type: [Byte; N].
// ------------------------------------------------------------------------

impl<const N: usize> Encode for [Byte; N] {
    fn to_bytes(&self) -> Result<Vec<Byte>, super::utils::CodecError> {
        Ok(self.to_vec())
    }

    fn get_encoded_size(&self) -> usize {
        N
    }

    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), super::utils::CodecError> {
        writer.extend_from_slice(&self.as_slice());
        Ok(())
    }
}

impl<const N: usize> Decode for [Byte; N] {
    fn from_bytes(bytes: &[Byte]) -> Result<(Self, &[Byte]), CodecError> {
        let (bytes, rem) = safe_split_at(bytes, N)?;
        // SAFETY: safe_split_at makes sure `bytes` is exactly `COUNT` bytes.
        let ptr = bytes.as_ptr() as *const [Byte; N];
        let result = unsafe { *ptr };
        Ok((result, rem))
    }
}

// ------------------------------------------------------------------------
// Type: Bytes32.
// ------------------------------------------------------------------------

impl Encode for Bytes32 {
    fn to_bytes(&self) -> Result<Vec<Byte>, super::utils::CodecError> {
        Ok(self.to_vec())
    }

    fn get_encoded_size(&self) -> usize {
        SIZE_32
    }

    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), super::utils::CodecError> {
        writer.extend_from_slice(&self.as_slice());
        Ok(())
    }
}

impl Decode for Bytes32 {
    fn from_bytes(bytes: &[Byte]) -> Result<(Self, &[Byte]), CodecError> {
        Decode::from_bytes(bytes).map(|(arr, rem)| (Bytes32::new(arr), rem))
    }
}

// ------------------------------------------------------------------------
// Type: Bytes33.
// ------------------------------------------------------------------------

impl Encode for Bytes33 {
    fn to_bytes(&self) -> Result<Vec<Byte>, super::utils::CodecError> {
        Ok(self.to_vec())
    }

    fn get_encoded_size(&self) -> usize {
        SIZE_33
    }

    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), super::utils::CodecError> {
        writer.extend_from_slice(&self.as_slice());
        Ok(())
    }
}

impl Decode for Bytes33 {
    fn from_bytes(bytes: &[Byte]) -> Result<(Self, &[Byte]), CodecError> {
        Decode::from_bytes(bytes).map(|(arr, rem)| (Bytes33::new(arr), rem))
    }
}

// ------------------------------------------------------------------------
// Type: Bytes64.
// ------------------------------------------------------------------------

impl Encode for Bytes64 {
    fn to_bytes(&self) -> Result<Vec<Byte>, super::utils::CodecError> {
        Ok(self.to_vec())
    }

    fn get_encoded_size(&self) -> usize {
        SIZE_64
    }

    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), super::utils::CodecError> {
        writer.extend_from_slice(&self.as_slice());
        Ok(())
    }
}

impl Decode for Bytes64 {
    fn from_bytes(bytes: &[Byte]) -> Result<(Self, &[Byte]), CodecError> {
        Decode::from_bytes(bytes).map(|(arr, rem)| (Bytes64::new(arr), rem))
    }
}
