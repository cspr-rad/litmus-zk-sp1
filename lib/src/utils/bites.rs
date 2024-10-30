// Sizes of fixed byte digest array.
pub const SIZE_32: usize = 32;
pub const SIZE_64: usize = 64;

// Type alias: Raw byte.
pub type Byte = u8;

// Type alias: Unbounded byte array.
pub type VecOfBytes = Vec<Byte>;

// Type alias: Context specific raw 32 bytes. Typically but not exclusively a digest.
pub type Bytes32 = [Byte; SIZE_32];

// Type alias: Context specific raw 64 bytes. Typically but not exclusively a signature.
pub type Bytes64 = [Byte; SIZE_64];
