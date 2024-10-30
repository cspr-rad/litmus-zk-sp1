// Length of fixed byte digest array.
pub const SIZE_32: usize = 32;
pub const SIZE_64: usize = 64;

// Context specific raw 32 bytes. Typically but not exclusively a digest.
pub type Bytes32 = [u8; SIZE_32];

// Context specific raw 64 bytes. Typically but not exclusively a signature.
pub type Bytes64 = [u8; SIZE_64];
