use super::super::constants;
use super::super::utils::{safe_split_at, CodecError, Decode, Encode};

// ------------------------------------------------------------------------
// Type: i32.
// ------------------------------------------------------------------------

impl Decode for i32 {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let mut result = [0u8; constants::ENCODED_SIZE_I32];
        let (bytes, remainder) = safe_split_at(bytes, constants::ENCODED_SIZE_I32)?;
        result.copy_from_slice(bytes);
        Ok((<i32>::from_le_bytes(result), remainder))
    }
}

impl Encode for i32 {
    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        Ok(self.to_le_bytes().to_vec())
    }

    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_I32
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Type: i64.
// ------------------------------------------------------------------------

impl Decode for i64 {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let mut result = [0u8; constants::ENCODED_SIZE_I64];
        let (bytes, remainder) = safe_split_at(bytes, constants::ENCODED_SIZE_I64)?;
        result.copy_from_slice(bytes);
        Ok((<i64>::from_le_bytes(result), remainder))
    }
}

impl Encode for i64 {
    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        Ok(self.to_le_bytes().to_vec())
    }

    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_I64
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Type: u8.
// ------------------------------------------------------------------------

impl Decode for u8 {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        match bytes.split_first() {
            None => Err(CodecError::EarlyEndOfStream),
            Some((byte, rem)) => Ok((*byte, rem)),
        }
    }
}

impl Encode for u8 {
    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        Ok(vec![*self])
    }

    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_u8
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.push(*self);
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Type: u16.
// ------------------------------------------------------------------------

impl Decode for u16 {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let mut result = [0u8; constants::ENCODED_SIZE_U16];
        let (bytes, remainder) = safe_split_at(bytes, constants::ENCODED_SIZE_U16)?;
        result.copy_from_slice(bytes);
        Ok((<u16>::from_le_bytes(result), remainder))
    }
}

impl Encode for u16 {
    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        Ok(self.to_le_bytes().to_vec())
    }

    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_U16
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Type: u32.
// ------------------------------------------------------------------------

impl Decode for u32 {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let mut result = [0u8; constants::ENCODED_SIZE_U32];
        let (bytes, remainder) = safe_split_at(bytes, constants::ENCODED_SIZE_U32)?;
        result.copy_from_slice(bytes);
        Ok((<u32>::from_le_bytes(result), remainder))
    }
}

impl Encode for u32 {
    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        Ok(self.to_le_bytes().to_vec())
    }

    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_U32
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Type: u64.
// ------------------------------------------------------------------------

impl Decode for u64 {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let mut result = [0u8; constants::ENCODED_SIZE_U64];
        let (bytes, remainder) = safe_split_at(bytes, constants::ENCODED_SIZE_U64)?;
        result.copy_from_slice(bytes);
        Ok((<u64>::from_le_bytes(result), remainder))
    }
}

impl Encode for u64 {
    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        Ok(self.to_le_bytes().to_vec())
    }

    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_U64
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Type: u128.
// ------------------------------------------------------------------------

impl Decode for u128 {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let mut result = [0u8; constants::ENCODED_SIZE_U128];
        let (bytes, remainder) = safe_split_at(bytes, constants::ENCODED_SIZE_U128)?;
        result.copy_from_slice(bytes);
        Ok((<u128>::from_le_bytes(result), remainder))
    }
}

impl Encode for u128 {
    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        Ok(self.to_le_bytes().to_vec())
    }

    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_U128
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Tests.
// ------------------------------------------------------------------------

#[cfg(test)]
mod proptests {
    use crate::binary::utils::assert_codec;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_i32(u in any::<i32>()) {
            assert_codec(&u);
        }

        #[test]
        fn test_i64(u in any::<i64>()) {
            assert_codec(&u);
        }

        #[test]
        fn test_u8(u in any::<u8>()) {
            assert_codec(&u);
        }

        #[test]
        fn test_u16(u in any::<u16>()) {
            assert_codec(&u);
        }

        #[test]
        fn test_u32(u in any::<u32>()) {
            assert_codec(&u);
        }

        #[test]
        fn test_u64(u in any::<u64>()) {
            assert_codec(&u);
        }

        #[test]
        fn test_u128(u in any::<u128>()) {
            assert_codec(&u);
        }
    }
}
