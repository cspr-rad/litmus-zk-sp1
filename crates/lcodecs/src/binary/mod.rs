mod bites;
mod chain_1;
mod chain_2;
mod constants;
mod crypto;
mod primitives_1;
mod primitives_2;
mod primitives_3;
mod utils;

pub use utils::Encode;

pub fn encode<T>(entity: &T) -> Vec<u8>
where
    T: Encode,
{
    entity.to_bytes().unwrap()
}
