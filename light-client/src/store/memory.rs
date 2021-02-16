//! Transient in-memory store

extern crate prusti_contracts;
use prusti_contracts::*;
use std::fmt;

use crate::{
    store::{LightStore, Status},
    types::{Height, LightBlock},
};

use std::collections::btree_map::Entry::*;
use std::collections::BTreeMap;

/// Internal entry for the memory store
#[derive(Clone, PartialEq)]
struct StoreEntry {
    light_block: LightBlock,
    status: Status,
}

impl fmt::Debug for StoreEntry {
    #[trusted]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StoreEntry")
            .finish()
    }
}

impl StoreEntry {
    fn new(light_block: LightBlock, status: Status) -> Self {
        Self {
            light_block,
            status,
        }
    }
}

/// Transient in-memory store.
#[derive(Clone, Default)]
pub struct MemoryStore {
    store: BTreeMap<Height, StoreEntry>,
}

impl MemoryStore {
    /// Create a new, empty, in-memory store
    pub fn new() -> Self {
        Self {
            store: BTreeMap::new(),
        }
    }
}

impl fmt::Debug for MemoryStore {
    #[trusted]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MemoryStore")
            .finish()
    }
}

impl LightStore for MemoryStore {
    #[trusted]
    // fn get(&self, height: Height, status: Status) -> Option<LightBlock> {
    //     self.store
    //         .get(&height)
    //         .filter(|e| e.status == status)
    //         .cloned()
    //         .map(|e| e.light_block)
    // }

    fn insert(&mut self, light_block: LightBlock, status: Status) {
        self.store
            .insert(light_block.height(), StoreEntry::new(light_block, status));
    }

    #[trusted]
    fn remove(&mut self, height: Height, status: Status) {
        if let Occupied(e) = self.store.entry(height) {
            if e.get().status == status {
                e.remove_entry();
            }
        }
    }

    fn update(&mut self, light_block: &LightBlock, status: Status) {
        self.insert(light_block.clone(), status);
    }

    #[trusted]
    fn highest(&self, status: Status) -> Option<LightBlock> {
        None
        // self.store
        //     .iter()
        //     .filter(|(_, e)| e.status == status)
        //     .max_by_key(|(&height, _)| height)
        //     .map(|(_, e)| e.light_block.clone())
    }

    fn lowest(&self, status: Status) -> Option<LightBlock> {
        // Note: keys are in sorted order
        match self.store.keys().next() {
            Some(key) => match self.store.get(key) {
                Some(e) => Some(e.light_block.clone()),
                None => None
            }
          None => None
        }
        // let it = self.store.iter();
        // let lowest_height = Height(0);
        // self.store
        //     .iter()
        //     .filter(|(_, e)| e.status == status)
        //     .min_by_key(|(&height, _)| height)
        //     .map(|(_, e)| e.light_block.clone())
    }

    #[trusted]
    fn all(&self, status: Status) -> Box<dyn Iterator<Item = LightBlock>> {
        // let light_blocks: Vec<_> = self
        //     .store
        //     .iter()
        //     .filter(|(_, e)| e.status == status)
        //     .map(|(_, e)| e.light_block.clone())
        //     .collect();

        // Box::new(light_blocks.into_iter())
        //
        Box::new(std::iter::empty())
    }
}
