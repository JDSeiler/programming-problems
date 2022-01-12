use std::collections::{HashSet, VecDeque};
use std::io::{self};

/**
Originally this solution was not correct but I've since corrected it and
now it passes the tests on Kattis. There were two bugs:
1. My initial implementation of the tree implicitely directed the edges
   which was a huge oversight.
2. My backing array was 1 element too short because I'm 1-indexing the nodes,
   which is what the problem description does.
*/

/**
 * Data Structure Stuff
 */

#[derive(Debug, Clone)]
struct Node {
    idx: usize,
    colors: [Option<i32>; 2],
    neighbors: Vec<usize>,
}

impl Node {
    fn new(idx: usize) -> Node {
        Node {
            idx: idx,
            colors: [None, None],
            neighbors: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Tree {
    nodes: Vec<Option<Node>>,
}

impl Tree {
    fn new() -> Tree {
        // This is kind of like making an array of 10,000 elements in Java
        // Except Rust really doesn't like you allocating raw arrays, so we
        // make a giant Vec and fill it with a sentinel (None)
        let mut node_vec = Vec::with_capacity(10_500);
        node_vec.resize_with(10_500, || None);

        Tree { nodes: node_vec }
    }

    fn add_directional_edge(&mut self, u_idx: usize, v_idx: usize) {
        match self.nodes.get_mut(u_idx).unwrap() {
            Some(u) => {
                u.neighbors.push(v_idx);
            }
            None => {
                let mut new_node = Node::new(u_idx);
                new_node.neighbors.push(v_idx);

                self.nodes[u_idx] = Some(new_node);
            }
        }
    }

    fn add_edge(&mut self, u_idx: usize, v_idx: usize) {
        self.add_directional_edge(u_idx, v_idx);
        self.add_directional_edge(v_idx, u_idx);
    }

    fn compute_root(&mut self) -> Option<usize> {
        self.nodes
            .iter()
            .find_map(|n| n.as_ref())
            .map_or(None, |n| Some(n.idx))
    }

    /**
     * An "unsightly root" is a root with only one neighbor.
     */
    fn has_unsightly_root(&self, candidate_root: usize) -> bool {
        if let Some(node) = self.get(candidate_root) {
            return node.neighbors.len() == 1;
        } else {
            // As long as we check that the compute_root returns Some
            // then this is true.
            unreachable!();
        }
    }

    /**
     * We don't want to deal with processing roots with only one child.
     * So we make the new root (start the traversal from) the only child
     * of the unsightly root. This ensures that so long as the tree has 3 or more
     * nodes in it, the root will have 2 children.
     */
    fn get_new_root(&self, unsightly_root: usize) -> usize {
        if let Some(node) = self.get(unsightly_root) {
            return *node.neighbors.get(0).unwrap();
        } else {
            // This is also fine as long as we... yknow.. use the function right...
            unreachable!();
        }
    }

    fn get(&self, idx: usize) -> Option<&Node> {
        self.nodes.get(idx).unwrap().as_ref()
    }

    fn get_mut(&mut self, idx: usize) -> Option<&mut Node> {
        self.nodes.get_mut(idx).unwrap().as_mut()
    }

    fn color_edges(&mut self, root: usize) {
        let mut to_process: VecDeque<usize> = VecDeque::new();
        // usize is Copy so mutating curr_node does not mutate self.root
        to_process.push_back(root);

        let mut visited: HashSet<usize> = HashSet::new();

        let mut next_color = 1;

        while to_process.len() > 0 {
            let curr_node = to_process.pop_front().unwrap();
            let mut parent_colors = self.get(curr_node).unwrap().colors;

            // COPY all of the children because we need the immutable borrow
            // from self.get to go away, so that we can mutate each child.
            // A better API from Tree would avoid this. But I don't have time
            // for that.
            let neighbors = self.get(curr_node).unwrap().neighbors.as_slice().to_owned();
            let children: Vec<usize> = neighbors
                .iter()
                .filter(|n| !visited.contains(*n))
                .copied() // The only purpose of this is to covert from Vec<&usize> to Vec<usize>
                .collect();

            if children.len() == 0 {
                let this_leaf_node = self.get_mut(curr_node).unwrap();
                this_leaf_node.colors[1] = Some(next_color);
                next_color += 1
            }

            for child in children {
                if let Some(color_slot) = open_slot(parent_colors) {
                    let child_node = self.get_mut(child).unwrap();
                    // Because this is a child node, it always has both slots free.
                    child_node.colors[0] = Some(next_color);

                    let parent_node = self.get_mut(curr_node).unwrap();
                    parent_node.colors[color_slot] = Some(next_color);

                    // Ugly hack, pretend that parent_colors is by reference :)
                    // Really, the NODE has a copy and we've got a copy floating
                    // around for the algorithm. Gotta update both.
                    parent_colors[color_slot] = Some(next_color);

                    next_color += 1;
                } else {
                    let child_node = self.get_mut(child).unwrap();
                    child_node.colors[0] = Some(parent_colors[0].unwrap());
                }

                to_process.push_back(child)
            }

            visited.insert(curr_node);
        }
    }
}

/**
 * Helpers for Colors
 */

// Returns Some(idx) if this node has open colors. Where idx is the index of the
// unused color. Returns None if there are no open slots.
fn open_slot(colors: [Option<i32>; 2]) -> Option<usize> {
    match colors {
        [f, s] => {
            if f.is_none() {
                Some(0)
            } else if s.is_none() {
                Some(1)
            } else {
                None
            }
        }
    }
}

fn print_colors(tree: &Tree) {
    let actual_nodes = tree
        .nodes
        .iter()
        .filter(|n| n.is_some())
        .map(|n| n.as_ref().unwrap())
        .collect::<Vec<&Node>>();

    for node in actual_nodes {
        println!("{} {}", node.colors[0].unwrap(), node.colors[1].unwrap());
    }
}

/**
 * Boring I/O stuff
 */

fn read_int() -> u32 {
    let mut buf = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut buf).expect("IO Error");
    buf.trim()
        .parse::<u32>()
        .expect("Expected a u32, got an error!")
}

fn read_edge() -> Vec<usize> {
    let mut buf = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut buf).expect("IO Error");
    buf.trim()
        .split_ascii_whitespace()
        .map(|i| i.parse::<usize>().expect("Expected a usize, got an error!"))
        .collect()
}

fn main() {
    let mut tree = Tree::new();

    let n = read_int();
    for _ in 0..(n - 1) {
        match read_edge().as_slice() {
            [start, end] => tree.add_edge(*start, *end),
            _ => unreachable!(),
        }
    }
    if n == 2 {
        println!("1 2");
        println!("2 1");
    } else {
        let mut candidate_root = tree.compute_root().expect("Expected a root, got nothing!");

        if tree.has_unsightly_root(candidate_root) {
            candidate_root = tree.get_new_root(candidate_root);
        }

        tree.color_edges(candidate_root);

        // println!(
        //     "Colored Tree is: {:#?}",
        //     tree.nodes
        //         .iter()
        //         .filter(|n| n.is_some())
        //         .collect::<Vec<&Option<Node>>>()
        // );

        print_colors(&tree);
    }
}
