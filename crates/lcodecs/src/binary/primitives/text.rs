use crate::binary::utils::{
    encode_byte_slice, get_encoded_size_of_byte_slice, safe_split_at, CodecError, Decode, Encode,
};

// ------------------------------------------------------------------------
// Type: str.
// ------------------------------------------------------------------------

impl Encode for str {
    #[inline(always)]
    fn get_encoded_size(&self) -> usize {
        get_encoded_size_of_byte_slice(self.as_bytes())
    }

    #[inline(always)]
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_byte_slice(self.as_bytes())
    }
}

// ------------------------------------------------------------------------
// Type: &str.
// ------------------------------------------------------------------------

impl Decode for &str {
    #[inline(always)]
    fn decode(_: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!();
    }
}

impl Encode for &str {
    #[inline(always)]
    fn get_encoded_size(&self) -> usize {
        (*self).get_encoded_size()
    }

    #[inline(always)]
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        (*self).encode()
    }
}

// ------------------------------------------------------------------------
// Type: String.
// ------------------------------------------------------------------------

impl Decode for String {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (size, remainder) = u32::decode(bytes)?;
        let (str_bytes, remainder) = safe_split_at(remainder, size as usize)?;
        let result = String::from_utf8(str_bytes.to_vec()).map_err(|_| CodecError::Formatting)?;
        Ok((result, remainder))
    }
}

impl Encode for String {
    fn get_encoded_size(&self) -> usize {
        get_encoded_size_of_byte_slice(self.as_bytes())
    }

    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_byte_slice(self.as_bytes())
    }
}
