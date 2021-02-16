//! Commits to a Tendermint blockchain

use crate::block::commit_sig::CommitSig;
use crate::block::{Height, Id, Round};
use crate::{Error, Kind};
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
use tendermint_proto::types::Commit as RawCommit;

/// Commit contains the justification (ie. a set of signatures) that a block was committed by a set
/// of validators.
/// TODO: Update links below!
/// <https://github.com/tendermint/tendermint/blob/51dc810d041eaac78320adc6d53ad8b160b06601/types/block.go#L486-L502>
/// <https://github.com/tendermint/spec/blob/d46cd7f573a2c6a2399fcab2cde981330aa63f37/spec/core/data_structures.md#lastcommit>
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(try_from = "RawCommit", into = "RawCommit")] // Used by testgen Generator trait
pub struct Commit {
    /// Block height
    pub height: Height,

    /// Round
    pub round: Round,

    /// Block ID
    pub block_id: Id,

    /// Signatures
    pub signatures: Vec<CommitSig>,
}

impl TryFrom<RawCommit> for Commit {
    type Error = Error;

    fn try_from(value: RawCommit) -> Result<Self, Self::Error> {
        let signatures: Result<Vec<CommitSig>, Error> = value
            .signatures
            .into_iter()
            .map(TryFrom::try_from)
            .collect();
        Ok(Self {
            height: value.height.try_into()?,
            round: value.round.try_into()?,
            block_id: value.block_id.ok_or(Kind::InvalidBlock)?.try_into()?, /* gogoproto.nullable = false */
            signatures: signatures?,
        })
    }
}

impl From<Commit> for RawCommit {
    fn from(value: Commit) -> Self {
        RawCommit {
            height: value.height.into(),
            round: value.round.into(),
            block_id: Some(value.block_id.into()),
            signatures: value.signatures.into_iter().map(Into::into).collect(),
        }
    }
}

impl Default for Commit {
    fn default() -> Self {
        Commit {
            // The default Height is 1, but the default commit is an empty commit with height = 0.
            height: Height::from(0_u32),
            round: Default::default(),
            block_id: Default::default(),
            signatures: vec![],
        }
    }
}
