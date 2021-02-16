//! Defines or just re-exports the main datatypes used by the light client.

pub use tendermint::{block::Height, hash::Hash};

use tendermint::{
    block::{
        signed_header::SignedHeader as TMSignedHeader,
        // Commit as TMCommit,
    },
};

pub struct LightBlock {
    /// Header and commit of this block
    pub signed_header: SignedHeader,
}

impl LightBlock {
    /// Constructs a new light block
    pub fn new(
        signed_header: SignedHeader
    ) -> LightBlock {
        Self {
            signed_header,
        }
    }

    /// Returns the height of this block.
    ///
    /// ## Note
    /// This is a shorthand for `block.signed_header.header.height`.
    pub fn height(&self) -> Height {
        self.signed_header.header.height
    }
}

pub enum Status {
    /// The light block has failed verification.
    Failed,
    /// The light has not been verified yet.
    Unverified,
    /// The light block has been successfully verified.
    Verified,
    /// The light block has been successfully verified and has passed fork detection.
    Trusted,
}

/// A signed header contains both a `Header` and its corresponding `Commit`.
pub type SignedHeader = TMSignedHeader;
