use std::{fmt};
use crate::{block};

extern crate prusti_contracts;
use prusti_contracts::*;

#[non_exhaustive]
#[derive(Clone, PartialEq)]
pub struct SignedHeader {
    pub header: block::Header,
}

impl fmt::Debug for SignedHeader {
    #[trusted]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SignedHeader")
            .finish()
    }
}
