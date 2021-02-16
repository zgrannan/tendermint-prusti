/// Hash digests
pub const SHA256_HASH_SIZE: usize = 32;
// #[derive(Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum Hash {
    /// SHA-256 hashes
    Sha256([u8; SHA256_HASH_SIZE]),
    /// Empty hash
    None,
}
