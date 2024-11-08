use super::constants;
use super::utils::{allocate_buffer, CodecError, Encode};
use lutils::bites::Byte;

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

    fn serialized_length(&self) -> usize {
        constants::ENCODED_SIZE_U8
            + match self {
                Some(v) => v.serialized_length(),
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
