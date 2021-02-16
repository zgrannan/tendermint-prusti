use crate::{block};
#[derive(Clone, PartialEq)]
pub struct Header {
    /// Current block height
    pub height: block::Height,

}
