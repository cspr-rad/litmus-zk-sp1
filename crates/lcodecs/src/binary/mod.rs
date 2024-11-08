mod bites;
mod chain;
mod constants;
mod crypto;
mod primitives_1;
mod primitives_2;
mod primitives_3;
mod utils;

pub use utils::Encode;

pub fn ddd<T: Encode>(entity: &T) -> Vec<u8> {
    entity.to_bytes().unwrap()
}
