use std::cmp::Ordering;
use std::collections::BinaryHeap;
use crate::alg::WdGraph;

mod alg;

#[derive(Debug, PartialOrd, PartialEq)]
struct Edge {
    idx: usize,
    weight: f32,
}

#[derive(Debug, PartialOrd, PartialEq)]
struct Node<T> {
    value: T,
    edges: Vec<Edge>,
}

impl<T> Node<T> {
    fn new(i_value: T) -> Self {
        Node {
            value: i_value,
            edges: Vec::new()
        }
    }
}

#[derive(Debug)]
struct Graph<T>(Vec<Node<T>>);

// Graph that is generic over the value in its nodes.
// T: Value of a node
impl<T: PartialEq> Graph<T> {
    fn new() -> Self {
        Graph(Vec::new())
    }

    // Check for duplicates?
    fn add_node(&mut self, node: Node<T>) {
        self.0.push(node);
    }

    fn add_nodes<I>(&mut self, nodes: I) where
        I: IntoIterator<Item = Node<T>> {

        for node in nodes {
            self.add_node(node);
        }
    }

    // Check for duplicates?
    fn add_edge(&mut self, from: T, to: T, weight: f32) {
        // Check if the 'from' node exists
        if let Some(f_idx) = self.0.iter().position(|n| n.value == from) {
            // Check if the 'to' node exists
            match self.0.iter().position(|n| n.value == to) {
                // If it doesn't exist, create it and then add the edge
                // Vec#push() appends to the vector, so the new node is always
                // going to be at the end.
                None => {
                    self.add_node(Node { value: to, edges: Vec::new() });
                    let t_idx = self.0.len() - 1;
                    self.0.get_mut(f_idx)
                        .unwrap()
                        .edges
                        .push(Edge { idx: t_idx, weight })
                }
                // If it does exist, add the edge
                Some(t_idx) => {
                    self.0.get_mut(f_idx)
                        .unwrap()
                        .edges
                        .push(Edge { idx: t_idx, weight })
                }
            }
        }
    }

    fn add_edges<I>(&mut self, edges: I) where
        I: IntoIterator<Item = (T, T, f32)> {
        for (from, to, weight) in edges {
            self.add_edge(from, to, weight);
        }
    }

    fn i_get_neighbors(&self, node: &T) -> Vec<&Node<T>> {
        let mut neighbors: Vec<&Node<T>> = Vec::new();
        if let Some(idx) = self.0.iter().position(|n| n.value == *node) {
            let target_node = self.0.get(idx).unwrap();
            for edge in &target_node.edges {
                neighbors.push(self.0.get(edge.idx).unwrap());
            }
        }
        neighbors
    }

    fn i_get_weight_between(&self, from: &T, to: &T) -> Option<f32> {
        // Check if the from node exists
        if let Some(f_idx) = self.0.iter().position(|n| n.value == *from) {
            let from_node = self.0.get(f_idx).unwrap();
            // Check the edges of the from node, if any of them lead to a node with a value
            // that matches 'to', return that edges weight.
            for edge in &from_node.edges {
                if self.0.get(edge.idx).unwrap().value == *to {
                    return Some(edge.weight);
                }
            }
            None
        } else {
            None
        }
    }
}

// T: Type for values of nodes in the graph
impl<T: PartialEq> WdGraph<T> for Graph<T> {
    fn get_neighbors(&self, node: &T) -> Vec<&Node<T>> {
        self.i_get_neighbors(node)
    }

    fn get_weight_between(&self, from: &T, to: &T) -> Option<f32> {
        self.i_get_weight_between(from, to)
    }
}

fn main() {
    let mut G: Graph<u32> = Graph::new();
    let nodes = vec![
        Node::new(1),
        Node::new(2),
        Node::new(3),
        Node::new(4)
    ];

    G.add_nodes(nodes);

    let edges = vec![
        (1,2,4.0),
        (2,3,3.0),
        (3,4,2.0),
        (4,1,1.0)
    ];

    G.add_edges(edges);
    println!("{:#?}", G);
    let neighbor_test = G.i_get_neighbors(&1);
    println!("{:#?}", neighbor_test);
    let weight_test = G.i_get_weight_between(&1,&2);
    println!("Weight between nodes 1 and 2 is {:#?}", weight_test);
}

