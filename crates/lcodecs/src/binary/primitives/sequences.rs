use super::super::{
    constants,
    utils::{CodecError, Decode, Encode},
};
use std::collections::BTreeMap;

// ------------------------------------------------------------------------
// Codec: Vec<T>.
// ------------------------------------------------------------------------

impl<T: Decode> Decode for Vec<T> {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        // Vec size.
        let (size, mut bytes_1) = u32::decode(bytes).unwrap();
        if size == 0 {
            return Ok((Vec::new(), bytes_1));
        }

        // Vec data.
        let mut result = Vec::<T>::with_capacity(size as usize);
        for _ in 0..size {
            let (entity, bytes_2) = T::decode(bytes_1).unwrap();
            result.push(entity);
            bytes_1 = bytes_2;
        }

        Ok((result, bytes_1))
    }
}

impl<T: Encode> Encode for Vec<T> {
    fn get_encoded_size(&self) -> usize {
        let mut result = constants::ENCODED_SIZE_U32;
        for entity in self.iter() {
            result += entity.get_encoded_size();
        }
        result
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        // Vec size.
        let size: u32 = self
            .len()
            .try_into()
            .map_err(|_| CodecError::NotRepresentable)
            .unwrap();
        size.write_bytes(writer).unwrap();

        // Vec data.
        for entity in self.iter() {
            entity.write_bytes(writer).unwrap();
        }

        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: BTreeMap<K, V>.
// ------------------------------------------------------------------------

impl<K, V> Decode for BTreeMap<K, V>
where
    K: Decode + Ord,
    V: Decode,
{
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        // BTreeMap size.
        let (size, mut bytes_1) = u32::decode(bytes).unwrap();

        // BTreeMap data.
        let mut result = BTreeMap::new();
        for _ in 0..size {
            let (k, bytes_2) = K::decode(bytes_1).unwrap();
            let (v, bytes_2) = V::decode(bytes_2).unwrap();
            result.insert(k, v);
            bytes_1 = bytes_2;
        }

        Ok((result, bytes_1))
    }
}

impl<K, V> Encode for BTreeMap<K, V>
where
    K: Encode,
    V: Encode,
{
    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_U32
            + self
                .iter()
                .map(|(key, value)| key.get_encoded_size() + value.get_encoded_size())
                .sum::<usize>()
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        // BTreeMap size.
        let size: u32 = self
            .len()
            .try_into()
            .map_err(|_| CodecError::NotRepresentable)
            .unwrap();
        size.write_bytes(writer).unwrap();

        // BTreeMap data.
        for (key, value) in self.iter() {
            key.write_bytes(writer).unwrap();
            value.write_bytes(writer).unwrap();
        }

        Ok(())
    }
}
