use super::super::utils::{CodecError, Decode, Encode};
use std::{collections::BTreeMap, vec::Vec};

// ------------------------------------------------------------------------
// Codec: Vec<T>.
// ------------------------------------------------------------------------

impl<T: Decode> Decode for Vec<T> {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!()
    }
}

impl<T: Encode> Encode for Vec<T> {
    fn get_encoded_size(&self) -> usize {
        unimplemented!()
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        unimplemented!()
    }
}

// impl<T: ToBytes> ToBytes for Vec<T> {
//     fn to_bytes(&self) -> Result<Vec<u8>, Error> {
//         ensure_efficient_serialization::<T>();

//         let mut result = Vec::with_capacity(self.serialized_length());
//         let length_32: u32 = self.len().try_into().map_err(|_| Error::NotRepresentable)?;
//         result.append(&mut length_32.to_bytes()?);

//         for item in self.iter() {
//             result.append(&mut item.to_bytes()?);
//         }

//         Ok(result)
//     }

//     fn into_bytes(self) -> Result<Vec<u8>, Error> {
//         ensure_efficient_serialization::<T>();

//         let mut result = allocate_buffer(&self)?;
//         let length_32: u32 = self.len().try_into().map_err(|_| Error::NotRepresentable)?;
//         result.append(&mut length_32.to_bytes()?);

//         for item in self {
//             result.append(&mut item.into_bytes()?);
//         }

//         Ok(result)
//     }

//     fn serialized_length(&self) -> usize {
//         iterator_serialized_length(self.iter())
//     }

//     fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), Error> {
//         let length_32: u32 = self.len().try_into().map_err(|_| Error::NotRepresentable)?;
//         writer.extend_from_slice(&length_32.to_le_bytes());
//         for item in self.iter() {
//             item.write_bytes(writer)?;
//         }
//         Ok(())
//     }
// }

// ------------------------------------------------------------------------
// Codec: BTreeMap<K, V>.
// ------------------------------------------------------------------------

impl<K, V> Decode for BTreeMap<K, V>
where
    K: Decode + Ord,
    V: Decode,
{
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!()
    }
}

impl<K, V> Encode for BTreeMap<K, V>
where
    K: Encode,
    V: Encode,
{
    fn get_encoded_size(&self) -> usize {
        unimplemented!()
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        unimplemented!()
    }
}

// impl<K, V> ToBytes for BTreeMap<K, V>
// where
//     K: ToBytes,
//     V: ToBytes,
// {
//     fn to_bytes(&self) -> Result<Vec<u8>, Error> {
//         let mut result = allocate_buffer(self)?;

//         let num_keys: u32 = self.len().try_into().map_err(|_| Error::NotRepresentable)?;
//         result.append(&mut num_keys.to_bytes()?);

//         for (key, value) in self.iter() {
//             result.append(&mut key.to_bytes()?);
//             result.append(&mut value.to_bytes()?);
//         }

//         Ok(result)
//     }

//     fn serialized_length(&self) -> usize {
//         U32_SERIALIZED_LENGTH
//             + self
//                 .iter()
//                 .map(|(key, value)| key.serialized_length() + value.serialized_length())
//                 .sum::<usize>()
//     }

//     fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), Error> {
//         let length_32: u32 = self.len().try_into().map_err(|_| Error::NotRepresentable)?;
//         writer.extend_from_slice(&length_32.to_le_bytes());
//         for (key, value) in self.iter() {
//             key.write_bytes(writer)?;
//             value.write_bytes(writer)?;
//         }
//         Ok(())
//     }
// }

// impl<K, V> FromBytes for BTreeMap<K, V>
// where
//     K: FromBytes + Ord,
//     V: FromBytes,
// {
//     fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
//         let (num_keys, mut stream) = u32::from_bytes(bytes)?;
//         let mut result = BTreeMap::new();
//         for _ in 0..num_keys {
//             let (k, rem) = K::from_bytes(stream)?;
//             let (v, rem) = V::from_bytes(rem)?;
//             result.insert(k, v);
//             stream = rem;
//         }
//         Ok((result, stream))
//     }
// }
