use std::fmt;
use std::fmt::Debug;

extern crate prusti_contracts;
use prusti_contracts::*;


#[derive(Copy, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Height(u64);

impl Debug for Height {
    #[trusted]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "block::Height({})", self.0)
    }
}
