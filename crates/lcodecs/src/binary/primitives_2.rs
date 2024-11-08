use super::constants;
use super::utils::{allocate_buffer, CodecError, Encode};
use lutils::bites::Byte;

// Encoder: `Option<T>` where T: Encode.
impl<T: Encode> Encode for Option<T> {
    fn to_bytes(&self) -> Result<Vec<Byte>, CodecError> {
        match self {
            None => Ok(vec![constants::TAG_OPTION_NONE]),
            Some(v) => {
                let mut result = allocate_buffer(self)?;
                result.push(constants::TAG_OPTION_SOME);

                let mut value = v.to_bytes()?;
                result.append(&mut value);

                Ok(result)
            }
        }
    }

    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_U8
            + match self {
                Some(v) => v.get_encoded_size(),
                None => 0,
            }
    }

    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), CodecError> {
        match self {
            None => writer.push(constants::TAG_OPTION_NONE),
            Some(v) => {
                writer.push(constants::TAG_OPTION_SOME);
                v.write_bytes(writer)?;
            }
        };
        Ok(())
    }
}

// Encoder: `Result<T, E>` where T + E: Encode.
impl<T: Encode, E: Encode> Encode for Result<T, E> {
    fn to_bytes(&self) -> Result<Vec<Byte>, CodecError> {
        let mut result = allocate_buffer(self)?;
        let (variant, mut value) = match self {
            Err(error) => (constants::TAG_RESULT_ERR, error.to_bytes()?),
            Ok(result) => (constants::TAG_RESULT_OK, result.to_bytes()?),
        };
        result.push(variant);
        result.append(&mut value);
        Ok(result)
    }

    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_U8
            + match self {
                Ok(ok) => ok.get_encoded_size(),
                Err(error) => error.get_encoded_size(),
            }
    }

    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), CodecError> {
        match self {
            Err(error) => {
                writer.push(constants::TAG_RESULT_ERR);
                error.write_bytes(writer)?;
            }
            Ok(result) => {
                writer.push(constants::TAG_RESULT_OK);
                result.write_bytes(writer)?;
            }
        };
        Ok(())
    }
}
