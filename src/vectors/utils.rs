#![allow(unused)]

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

use super::{Vector2Int, ORTHO_DIRECTIONS};

/// helper struct for the path finder
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Node {
    pub v: Vector2Int,
    pub cost: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.v.cmp(&other.v))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
