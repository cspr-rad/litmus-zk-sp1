// Sizes of fixed byte digest array.
pub const SIZE_32: usize = 32;
pub const SIZE_64: usize = 64;

// Type alias: Raw byte.
pub type Byte = u8;

// Type alias: Unbounded byte array.
pub type Bytes = Vec<Byte>;

// Type alias: Context specific raw 32 bytes. Typically (but not exclusively) a digest.
pub type Bytes32 = [Byte; SIZE_32];

// Type alias: Context specific raw 64 bytes. Typically (but not exclusively) a signature.
pub type Bytes64 = [Byte; SIZE_64];


pub struct digest1(pub Bytes32) {};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Digest(pub Bytes32);

impl From<Byte> for bool {
    pub fn from()
}

impl From<Byte> for bool {
    fn from(x: Byte) -> Self {
        match x {
            0 => False,
            1 => True,
        }
    }
}


// impl XX {
//     pub fn to_vec(&self) -> Bytes {
//         self.0.to_vec()
//     }
// }

// // Convertors.
// impl Bytes32 {
//     pub fn to_vec(&self) -> Bytes {
//         self.0.to_vec()
//     }
// }

// impl Bytes32 {
//     pub fn to_vec(&self) -> Bytes {
//         self.0.to_vec()
//     }
// }
