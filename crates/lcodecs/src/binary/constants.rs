/// Encoded size: `bool`.
pub(super) const ENCODED_SIZE_BOOL: usize = 1;

/// The number of bytes in a serialized `i32`.
pub(super) const ENCODED_SIZE_I32: usize = core::mem::size_of::<i32>();

/// The number of bytes in a serialized `i64`.
pub(super) const ENCODED_SIZE_I64: usize = core::mem::size_of::<i64>();

/// Encoded size: `unit`.
pub(super) const ENCODED_SIZE_UNIT: usize = 0;

/// Encoded size: `u8`.
pub(super) const ENCODED_SIZE_U8: usize = core::mem::size_of::<u8>();

/// Encoded size: `u16`.
pub(super) const ENCODED_SIZE_U16: usize = core::mem::size_of::<u16>();

/// Encoded size: `u32`.
pub(super) const ENCODED_SIZE_U32: usize = core::mem::size_of::<u32>();

/// Encoded size: `u64`.
pub(super) const ENCODED_SIZE_U64: usize = core::mem::size_of::<u64>();
