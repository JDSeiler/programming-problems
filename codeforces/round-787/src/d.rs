
/*
 * Solution is correct (produces the right answer) but times out on the largest test (200,000
 * vertices). It's something with `get_paths` where I'm processing each node more than once.
 * Ideally I'd process each one only once as I find the path, but it's not set up like that.
 *
 * I'll have to do some restructuring to get this fast enough.
 * */

pub fn main() {
    let t = read!(usize);
    for _ in 0..t {
        let n = read!(usize);
        let nodes = read_vec!(usize);
        let (mut adj_list, root_node) = make_adjacency_list(nodes, n);
 
        // BFS the tree to get layer numbers for each node.
        let mut layers = get_layer_numbers(&adj_list, root_node);
        // Start at the nodes furthest away from the root and follow 
        // parent pointers back to the root, making a path as you go.
        let paths = get_paths(&mut adj_list, &mut layers);
 
        // First, print the number of paths
        println!("{}", paths.len());
        for path in paths {
            // Then, print the length of each path, followed by the nodes in the path
            println!("{}", path.len());
            for (idx, node) in path.iter().rev().enumerate() {
                if idx == path.len()-1 {
                    println!("{}", node);
                } else {
                    print!("{} ", node);
                }
            }
        }
        // After each case, print an extra newline
        println!();
    }
}
 
#[derive(Default, Debug)]
struct Node {
    visited: bool,
    children: Vec<usize>,
    // default for Option<T> is None
    parent: Option<usize>,
}
 
fn make_adjacency_list(parents: Vec<usize>, num_nodes: usize) -> (Vec<Node>, usize) {
    let mut adj_list: Vec<Node> = Vec::new();
    // Pre-allocate so that all indices are already valid.
    // Allocate an extra slot in the array so that we can 1-index, like the problem desc.
    adj_list.resize_with(num_nodes + 1, Default::default);
 
    let mut root = 0;

    for (idx, &parent) in parents.iter().enumerate() {
        // We just leave the root node alone, the default Struct values are acceptable.
        // We use parent: None to find the root later.
        if idx + 1 != parent {
            // Add a child to this parent.
            // Create a scope so that the `parent_node` reference
            // goes away, so that we can mutably borrow something else.
            {
                let parent_node = adj_list.get_mut(parent).unwrap();
                parent_node.children.push(idx + 1);
            }
 
            let child_node = adj_list.get_mut(idx + 1).unwrap();
            child_node.parent = Some(parent);
        } else {
           root = idx + 1; 
        }
    }
 
    // The first element is a dud, give it a dud parent so that only
    // one node in the adj_list has parent: None, the root
    adj_list.get_mut(0).unwrap().parent = Some(0);
 
    (adj_list, root)
}
 
/// Returns an association list between node IDs and their layer number in the graph.
fn get_layer_numbers(adj_list: &[Node], root_node: usize) -> Vec<(usize, usize)> {
    // (node_id, layer_num)
    // layer of ID is NUM
    let mut layers = Vec::new();
 
    let mut frontier = VecDeque::new();
    frontier.push_back((root_node, 1));
    while !frontier.is_empty() {
        let (current, layer) = frontier.pop_front().unwrap();
        layers.push((current, layer));
 
        for &c in adj_list.get(current).unwrap().children.iter() {
            // Add the children for exploration
            frontier.push_back((c, layer + 1));
        }
    }
 
    // Conveniently, this adds the layers in ascending order!
    // so that the leafs are all at the end of the list.
    layers
}
 
fn get_paths(adj_list: &mut [Node], layers: &mut Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut paths = Vec::new();
 
    // I'm a bit concerned about performance, because this loop is going
    // to process EVERY node. So the performance is going to be O(n * something)?
    //
    // I could try replacing the association list with a BTreeSet. That'd let me have
    // constant time "find max" and worst-case O(log n) removal. That'd make this O(n log n)
    // instead of O(n^2) worst case? I can't prove it's n^2 but it feels like it.
    // Except, BTreeSet#pop_last is nightly only...
    while !layers.is_empty() {
        // Due to the sort order of `layers` this will always be
        // the lowest non-processed leaf
        let (start_node, _layer) = layers.pop().unwrap();
 
        if !adj_list.get(start_node).unwrap().visited {
            let mut path = vec![start_node];
            let mut current = start_node;
            adj_list.get_mut(current).unwrap().visited = true;

            // I need to process each node once... but right now I'm 
            // processing each node multiple times, which is causing me to time out.
            while let Some(parent) = adj_list.get(current).unwrap().parent {
                if adj_list.get(parent).unwrap().visited {
                    break;
                }
 
                path.push(parent);
                adj_list.get_mut(parent).unwrap().visited = true;
                current = parent;
            }
            paths.push(path);
        }
    }
 
    paths
}
 
/**
Reads a full line of input and parses it into the value of the specified type.
Any type that implements FromStr can be used: https://doc.rust-lang.org/stable/std/str/trait.FromStr.html
```rs
let my_number = read!(u32);
let my_float = read!(f64);
```
*/
#[allow(unused_macros)]
macro_rules! read {
    ($type:ty) => {{
        let mut inner = String::new();
        std::io::stdin()
            .read_line(&mut inner)
            .expect("read! was unable to read from stdin");
        inner
            .trim()
            .parse::<$type>()
            .expect("read! was unable to parse into the desired type")
    }};
}
 
/**
Reads a full line of input and returns it as a `String`.
*/
#[allow(unused_macros)]
macro_rules! read_str {
    () => {{
        let mut inner = String::new();
        std::io::stdin()
            .read_line(&mut inner)
            .expect("read_str! was unable to read from stdin");
        inner.trim().to_owned()
    }};
}
 
/**
Reads a line of input, splits it on whitespace, and parses each value into the specified type.
The values are returned in a Vec.
```rs
let mut some_numbers: Vec<i64> = read_vec!(i64);
```
*/
#[allow(unused_macros)]
macro_rules! read_vec {
    ($type:ty) => {{
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).unwrap();
        inner
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<$type>().unwrap())
            .collect::<Vec<$type>>()
    }};
}
 
/**
Reads a line of input, splits it on whitespace, and parses each value into the specified type.
The values are then matched into a 2-element tuple using slice matching.
```rs
let (a,b) = read_pair!(usize);
```
*/
#[allow(unused_macros)]
macro_rules! read_pair {
    ($type:ty) => {{
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).unwrap();
        let v = inner
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<$type>().unwrap())
            .collect::<Vec<$type>>();
        let (a, b) = match &v[..] {
            &[f, s, ..] => (f, s),
            _ => unreachable!(),
        };
        (a, b)
    }};
}
 
use std::collections::VecDeque;
 
pub(crate) use read;
pub(crate) use read_pair;
pub(crate) use read_str;
pub(crate) use read_vec;
