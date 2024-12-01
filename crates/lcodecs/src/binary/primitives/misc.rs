use super::super::constants;
use crate::binary::utils::{allocate_buffer, CodecError, Decode, Encode};

// ------------------------------------------------------------------------
// Type: bool.
// ------------------------------------------------------------------------

impl Decode for bool {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        match bytes.split_first() {
            None => Err(CodecError::EarlyEndOfStream),
            Some((byte, rem)) => match byte {
                1 => Ok((true, rem)),
                0 => Ok((false, rem)),
                _ => Err(CodecError::Formatting),
            },
        }
    }
}

impl Encode for bool {
    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_BOOL
    }

    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        u8::from(*self).encode()
    }
}

// ------------------------------------------------------------------------
// Type: Option<T>.
// ------------------------------------------------------------------------

impl<T: Decode> Decode for Option<T> {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (tag, rem) = u8::decode(bytes)?;
        match tag {
            constants::TAG_OPTION_NONE => Ok((None, rem)),
            constants::TAG_OPTION_SOME => {
                let (t, rem) = T::decode(rem)?;
                Ok((Some(t), rem))
            }
            _ => Err(CodecError::Formatting),
        }
    }
}

impl<T: Encode> Encode for Option<T> {
    fn get_encoded_size(&self) -> usize {
        match self {
            Some(v) => constants::ENCODED_SIZE_u8 + v.get_encoded_size(),
            None => constants::ENCODED_SIZE_u8,
        }
    }

    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        match self {
            None => Ok(vec![constants::TAG_OPTION_NONE]),
            Some(v) => {
                let mut result = allocate_buffer(self).unwrap();
                result.push(constants::TAG_OPTION_SOME);

                let mut value = v.encode().unwrap();
                result.append(&mut value);

                Ok(result)
            }
        }
    }
}

// ------------------------------------------------------------------------
// Type: Result<T, E>.
// ------------------------------------------------------------------------

impl<T: Decode, E: Decode> Decode for Result<T, E> {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (variant, rem) = u8::decode(bytes)?;
        match variant {
            constants::TAG_RESULT_ERR => {
                let (value, rem) = E::decode(rem)?;
                Ok((Err(value), rem))
            }
            constants::TAG_RESULT_OK => {
                let (value, rem) = T::decode(rem)?;
                Ok((Ok(value), rem))
            }
            _ => Err(CodecError::Formatting),
        }
    }
}

impl<T: Encode, E: Encode> Encode for Result<T, E> {
    fn get_encoded_size(&self) -> usize {
        match self {
            Err(error) => constants::ENCODED_SIZE_u8 + error.get_encoded_size(),
            Ok(value) => constants::ENCODED_SIZE_u8 + value.get_encoded_size(),
        }
    }

    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        let mut result = allocate_buffer(self)?;
        match self {
            Err(error) => {
                result.push(constants::TAG_RESULT_ERR);
                result.extend(error.encode().unwrap());
            }
            Ok(value) => {
                result.push(constants::TAG_RESULT_OK);
                result.extend(value.encode().unwrap());
            }
        };
        Ok(result)
    }
}

// ------------------------------------------------------------------------
// Type: unit.
// ------------------------------------------------------------------------

impl Decode for () {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        Ok(((), bytes))
    }
}

impl Encode for () {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        Ok(Vec::new())
    }

    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_UNIT
    }
}
