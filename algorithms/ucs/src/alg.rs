use std::cmp::Ordering;
use std::collections::BinaryHeap;
use crate::Node;
use std::ops::Add;

// This design pattern is taken directly from the wonderful
// petgraph crate: https://docs.rs/crate/petgraph/0.5.1/source/src/scored.rs

// N: Type of Nodes
// S: Type of "score"

struct TaggedNode<N, S>(N, S);
impl<N, S: PartialOrd> PartialEq for TaggedNode<N, S> {
    fn eq(&self, other: &TaggedNode<N, S>) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<N, S: PartialOrd> Eq for TaggedNode<N, S> {}

impl<N, S: PartialOrd> PartialOrd for TaggedNode<N, S> {
    fn partial_cmp(&self, other: &TaggedNode<N, S>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Wrapper implementation that handles the strange behavior with NaN
// Allowing for floating point weights
impl<N, S: PartialOrd> Ord for TaggedNode<N, S> {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = &self.1;
        let b = &other.1;
        if a == b {
            Ordering::Equal
        } else if a < b {
            Ordering::Greater
        } else if a > b {
            Ordering::Less
        } else if a.ne(a) && b.ne(b) {
            // these are the NaN cases
            Ordering::Equal
        } else if a.ne(a) {
            // Order NaN less, so that it is last in the MinScore order
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

/*
T = Value Type of Nodes
*/
pub trait WdGraph<T> {
    fn get_neighbors(&self, node: &T) -> Vec<&Node<T>>;
    fn get_weight_between(&self, from: &T, to: &T) -> Option<f32>;
}

// Abstract over the graph implementation (G)
// Abstract over the type of node's values (T)
// NOT abstract over the Node itself. This will use
// the Node type, but it will be Node<T>
pub fn ucs<G, T>(graph: &mut G, start: T) where G: WdGraph<T> {
    // TODO - BinaryHeap in Rust does not have a reduce_key operation
}
