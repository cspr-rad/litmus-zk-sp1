use super::super::constants;
use super::super::utils::{deconstruct_bytes, CodecError, Decode, Encode};

// ------------------------------------------------------------------------
// Type: i32.
// ------------------------------------------------------------------------

impl Decode for i32 {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (bytes, remainder) = deconstruct_bytes::<4>(&bytes).unwrap();

        Ok((<i32>::from_le_bytes(bytes), remainder))
    }
}

impl Encode for i32 {
    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_I32
    }

    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        Ok(self.to_le_bytes().to_vec())
    }
}

// ------------------------------------------------------------------------
// Type: i64.
// ------------------------------------------------------------------------

impl Decode for i64 {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (bytes, remainder) = deconstruct_bytes::<8>(&bytes).unwrap();

        Ok((<i64>::from_le_bytes(bytes), remainder))
    }
}

impl Encode for i64 {
    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_I64
    }

    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        Ok(self.to_le_bytes().to_vec())
    }
}

// ------------------------------------------------------------------------
// Type: u8.
// ------------------------------------------------------------------------

impl Decode for u8 {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        match bytes.split_first() {
            None => Err(CodecError::EarlyEndOfStream),
            Some((byte, rem)) => Ok((*byte, rem)),
        }
    }
}

impl Encode for u8 {
    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_u8
    }

    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        Ok(vec![*self])
    }
}

// ------------------------------------------------------------------------
// Type: u16.
// ------------------------------------------------------------------------

impl Decode for u16 {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (bytes, remainder) = deconstruct_bytes::<2>(&bytes).unwrap();

        Ok((<u16>::from_le_bytes(bytes), remainder))
    }
}

impl Encode for u16 {
    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_U16
    }

    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        Ok(self.to_le_bytes().to_vec())
    }
}

// ------------------------------------------------------------------------
// Type: u32.
// ------------------------------------------------------------------------

impl Decode for u32 {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (bytes, remainder) = deconstruct_bytes::<4>(&bytes).unwrap();

        Ok((<u32>::from_le_bytes(bytes), remainder))
    }
}

impl Encode for u32 {
    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_U32
    }

    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        Ok(self.to_le_bytes().to_vec())
    }
}

// ------------------------------------------------------------------------
// Type: u64.
// ------------------------------------------------------------------------

impl Decode for u64 {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (bytes, remainder) = deconstruct_bytes::<8>(&bytes).unwrap();

        Ok((<u64>::from_le_bytes(bytes), remainder))
    }
}

impl Encode for u64 {
    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_U64
    }

    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        Ok(self.to_le_bytes().to_vec())
    }
}

// ------------------------------------------------------------------------
// Type: u128.
// ------------------------------------------------------------------------

impl Decode for u128 {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (bytes, remainder) = deconstruct_bytes::<16>(&bytes).unwrap();

        Ok((<u128>::from_le_bytes(bytes), remainder))
    }
}

impl Encode for u128 {
    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_U128
    }

    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        Ok(self.to_le_bytes().to_vec())
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
