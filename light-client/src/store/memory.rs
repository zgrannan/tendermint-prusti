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
        f.debug_struct("StoreEntry").finish()
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
        f.debug_struct("MemoryStore").finish()
    }
}

#[trusted]
#[ensures(get_ms(ms, light_block.height(), status) == Some(light_block))]
fn insert_ms(ms: &mut MemoryStore, light_block: LightBlock, status: Status) {
    ms.store
        .insert(light_block.height(), StoreEntry::new(light_block, status));
}

#[pure]
#[trusted]
fn get_ms(ms: &MemoryStore, height: Height, status: Status) -> Option<LightBlock> {
    let value = ms.store.get(&height);
    match value {
        Some(e) => {
            if (e.status == status) {
                return Some(e.light_block.clone());
            } else {
                return None;
            }
        }
        None => return None,
    }
}

#[trusted]
#[requires(index < veclen(vec))]
fn get_and_unwrap(vec: &Vec<StoreEntry>, index: usize) -> StoreEntry {
    return vec.get(index).unwrap().clone();
}

#[ensures(
    match result {
        None => true, 
        Some(x) => true
    })]
fn highest(ls: &MemoryStore, status: Status) -> Option<LightBlock> {
    let mut vec: Vec<StoreEntry> = values(ls);
    vec.reverse();
    let mut i = 0;
    while i < veclen(&vec) {
        body_invariant!(i < veclen(&vec));
        let e = get_and_unwrap(&vec, i);
        if (e.status == status) {
            return Some(e.light_block)
        }
        i += 1;
    }
    return None;
}

fn values(ls: &MemoryStore) -> Vec<StoreEntry> {
    return ls.store.values().cloned().collect()
}

#[pure]
#[trusted]
fn veclen(vec: &Vec<StoreEntry>) -> usize {
    return vec.len();
}


impl LightStore for MemoryStore {
    #[pure]
    #[trusted]
    fn get(&self, height: Height, status: Status) -> Option<LightBlock> {
        let value = self.store.get(&height);
        match value {
            Some(e) => {
                if (e.status == status) {
                    return Some(e.light_block.clone());
                } else {
                    return None;
                }
            }
            None => return None,
        }
    }

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

    // Note: this relies on the fact that iter() returns a list
    //       of elements sorted by key
    fn highest(&self, status: Status) -> Option<LightBlock> {
        let mut vec: Vec<StoreEntry> = values(self);
        vec.reverse();
        let mut i = 0;
        while i < veclen(&vec) {
            body_invariant!(i < veclen(&vec));
            let e = get_and_unwrap(&vec, i);
            if (e.status == status) {
                return Some(e.light_block)
            }
            i += 1;
        }
        return None;
    }

    // Note: this relies on the fact that iter() returns a list
    //       of elements sorted by key
    fn lowest(&self, status: Status) -> Option<LightBlock> {
        let vec: Vec<StoreEntry> = values(self);
        let mut i = 0;
        while i < veclen(&vec) {
            body_invariant!(i < veclen(&vec));
            let e = get_and_unwrap(&vec, i);
            if (e.status == status) {
                return Some(e.light_block)
            }
            i += 1;
        }
        return None;
    }

    #[trusted]
    fn all(&self, status: Status) -> Box<dyn Iterator<Item = LightBlock>> {
        let mut vec = Vec::new();
        let mut it = self.store.values();
        loop {
            match it.next() {
                Some(e) => {
                    if (e.status == status) {
                        vec.push(e.light_block.clone())
                    }
                }
                None => return Box::new(vec.into_iter()),
            }
        }
    }
}
