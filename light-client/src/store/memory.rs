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

#[extern_spec]
impl<T> std::option::Option<T> {
    #[pure]
    #[ensures(matches!(*self, Some(_)) == result)]
    pub fn is_some(&self) -> bool;

    #[pure]
    #[ensures(self.is_some() == !result)]
    pub fn is_none(&self) -> bool;

    #[requires(self.is_some())]
    pub fn unwrap(self) -> T;

    // ...
}


/// Internal entry for the memory store
#[derive(Clone, PartialEq, Eq)]
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

#[pure]
#[trusted]
fn height0() -> Height {
    panic!("Nvm");
}

#[pure]
fn ok(olb: Option<LightBlock>) -> bool {
    return true;
}


// #[ensures(
//     match result {
//         None => true, 
//         Some(x) => x.status == status
//     })]
// // #[ensures(forall(|i: isize| result.is_some() ==> result.unwrap().status == status))]
// fn lowest(ls: &MemoryStore, status: Status) -> Option<StoreEntry> {
//     let mut vec: Vec<StoreEntry> = values(ls);
//     let mut i = 0;
//     while i < veclen(&vec) {
//         body_invariant!(i < veclen(&vec));
//         let e = get_and_unwrap(&vec, i);
//         if (e.status == status) {
//             return Some(e)
//         }
//         i += 1;
//     }
//     return None;
// }

#[pure]
#[trusted]
#[ensures(result >= 0)]
fn num_values(ls: &MemoryStore) -> usize {
    return veclen(&values(ls));
}

#[pure]
#[trusted]
#[requires(idx < num_values(ls))]
fn get_at(ls: &MemoryStore, idx: usize) -> StoreEntry {
    let vec: Vec<StoreEntry> = values(ls);
    return get_and_unwrap(&vec, idx);
}

#[ensures(result < num_values(ls) as i64)]
#[ensures(result > 0 ==> get_at(&ls, result as usize).status == status)]
fn lowest_idx(ls: &MemoryStore, status: Status) -> i64 {
    let mut i = 0;
    while i < num_values(&ls) {
        body_invariant!(i < num_values(&ls));
        let e = get_at(ls, i);
        if (e.status == status) {
            return i as i64;
        }
        i += 1;
    }
    return -1;
}

// #[pure]
// fn contains(ms: &MemoryStore, lb: LightBlock) -> bool {
//     let vec: Vec<StoreEntry> = values(ms);
//     return contains_helper(vec, lb, 0);
// }

// #[pure]
// fn contains_helper(haystack: Vec<StoreEntry>, needle: LightBlock, at: usize) -> bool {
//     if at < veclen(&haystack) {
//         let e = get_and_unwrap(&haystack, at);
//         if (e.light_block == needle) {
//             return true;
//         } else {
//             return contains_helper(haystack, needle, at + 1);
//         }
//     } else {
//         return false;
//     }
// }

fn light_blocks(ls: &MemoryStore) -> Vec<LightBlock> {
    return ls.store.values().cloned().map(|e| e.light_block).collect();
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
