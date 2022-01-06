use std::io::{self};
use std::collections::VecDeque;

/**
 * This solution is not correct. There is some edge case causing a run-time error
 * on Kattis but I can't think of what it is yet. Otherwise this handles all of the test
 * cases present as .txt files included with the source code.
 * 
 * badroot.txt is a breaking case!
 * The issue is either:
 * - How I select the root
 * - How I'm implcitely directing the edges with add_edge
 */


/**
 * Data Structure Stuff
 */

#[derive(Debug, Clone)]
struct Node {
    idx: usize,
    colors: [Option<i32>; 2],
    parent: Option<usize>,
    children: Vec<usize>,
}

impl Node {
    fn new(idx: usize) -> Node {
        Node {
            idx: idx,
            colors: [None, None],
            parent: None,
            children: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Tree {
    nodes: Vec<Option<Node>>,
    root: Option<usize>,
}

impl Tree {
    fn new() -> Tree {
        // This is kind of like making an array of 10,000 elements in Java
        // Except Rust really doesn't like you allocating raw arrays, so we
        // make a giant Vec and fill it with a sentinel (None)
        let mut node_vec = Vec::with_capacity(10_000);
        node_vec.resize_with(10_000, || None);

        Tree {
            nodes: node_vec,
            root: None,
        }
    }

    fn add_edge(&mut self, start: usize, end: usize) {
        match self.nodes.get_mut(start).unwrap() {
            Some(v) => {
                v.children.push(end);
            }
            None => {
                let mut new_parent_node = Node::new(start);
                new_parent_node.children.push(end);

                self.nodes[start] = Some(new_parent_node);
            }
        }

        match self.nodes.get_mut(end).unwrap() {
            Some(v) => {
                v.parent = Some(start);
            }
            None => {
                let mut new_child_node = Node::new(end);
                new_child_node.parent = Some(start);

                self.nodes[end] = Some(new_child_node);
            }
        }
    }

    fn compute_root(&mut self) {
        let root = self
            .nodes
            .iter()
            .find_map(|n| n.as_ref())
            .map_or(None, |n| Some(n.idx));

        self.root = root;
    }

    fn get(&self, idx: usize) -> Option<&Node> {
        self.nodes.get(idx).unwrap().as_ref()
    }

    fn get_mut(&mut self, idx: usize) -> Option<&mut Node> {
        self.nodes.get_mut(idx).unwrap().as_mut()
    }

    fn has_unsightly_root(&self) -> bool {
        let root = self.root.expect("Rootless tree!");
        if let Some(node) = self.get(root) {
            return node.children.len() == 1;
        } else {
            // We know this is unreachable because we're doing that silly
            // Vector where we've pre-filled 10,000 elements.
            // Plus we know that if self.root() doesn't return None
            // the node MUST exist.
            unreachable!();
        }
    }

    fn reroot(&mut self) -> Option<usize> {
        if let Some(root) = self.root {
            /*
             * Bear with me:
             * We need to mutably borrow two different elements out of the collection
             * We cannot do that in the same scope because that would introduce two
             * mutable references int the same scope. So we COMPLETELY update the root
             * node first in its own scope so that the references drop
             * 
             * Then we update the other node entirely in its own scope.
             * 
             * This is *probably* an anti-pattern that I've walked into because the whole
             * Vec::resize_with(10,000, || None) business is... unsavory
             * 
             * We compute the child_idx first because usize implements Copy and
             * so it doesn't interfere with the memory semantics.
             */
            let child_idx = self.nodes[root].as_ref().unwrap().children[0];
            {
                let root_node = self.nodes[root].as_mut().unwrap();
                root_node.parent = Some(child_idx);

                if root_node.children.len() > 1 {
                    panic!("Clearing root children when there is more than one!");
                }
                root_node.children.clear();
            }
            let child_node = self.nodes[child_idx].as_mut().unwrap();

            child_node.parent = None;
            child_node.children.push(root);

            self.root = Some(child_idx);
            return Some(child_idx);
        } else {
            return None;
        }
    }

    fn color_edges(&mut self) {
        let mut to_process: VecDeque<usize> = VecDeque::new();
        // usize is Copy so mutating curr_node does not mutate self.root
        to_process.push_back(self.root.expect("Compute the root before coloring!"));

        let mut next_color = 1;

        while to_process.len() > 0 {
            let curr_node = to_process.pop_front().unwrap();
            let mut parent_colors = self.get(curr_node).unwrap().colors;

            // COPY all of the children because we need the immutable borrow
            // from self.get to go away, so that we can mutate each child.
            // A better API from Tree would avoid this. But I don't have time
            // for that.
            let children = self.get(curr_node).unwrap().children.as_slice().to_owned();

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
                    parent_colors[color_slot] = Some(next_color);

                    next_color += 1;
                } else {
                    let child_node = self.get_mut(child).unwrap();
                    child_node.colors[0] = Some(parent_colors[0].unwrap());
                }

                to_process.push_back(child)
            }
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
    let actual_nodes = tree.nodes.iter()
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
    // println!("Pre-Reroot Tree is: {:#?}", tree.nodes.iter().filter(|n| n.is_some()).collect::<Vec<&Option<Node>>>());
    if n == 2 {
        println!("1 2");
        println!("2 1");
    } else {
        tree.compute_root();
    
        if tree.has_unsightly_root() {
            tree.reroot();
        }
    
        // println!("Post-Reroot Tree is: {:#?}", tree.nodes.iter().filter(|n| n.is_some()).collect::<Vec<&Option<Node>>>());
        tree.color_edges();
        
        println!("Colored Tree is: {:#?}", tree.nodes.iter().filter(|n| n.is_some()).collect::<Vec<&Option<Node>>>());


        print_colors(&tree);
    }
}
