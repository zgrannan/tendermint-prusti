use crate::{block};
#[derive(Clone, PartialEq, Eq)]
pub struct Header {
    /// Current block height
    pub height: block::Height,

}
