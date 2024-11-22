mod chain;
mod constants;
mod crypto;
mod primitives;
mod utils;

pub use utils::{CodecError, Decode, Encode};

pub fn encode<T>(entity: &T) -> Vec<u8>
where
    T: Encode,
{
    entity.to_bytes().unwrap()
}

pub fn decode<T>(encoded: &Vec<u8>) -> Result<(T, &[u8]), CodecError>
where
    T: Decode,
{
    T::from_bytes(encoded.as_slice())
}
