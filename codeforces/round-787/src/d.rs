pub fn main() {
    let t = read!(usize);
    for _ in 0..t {
        let n = read!(usize);
        let nodes = read_vec!(usize);

        let mut adj_list = make_adjacency_list(nodes, n);
        println!("{:#?}", adj_list);
        let mut layers = get_layer_numbers(&adj_list);
        println!("{:?}", layers);
        let paths = get_paths(&mut adj_list, &mut layers);
        println!("{:?}", paths);
        // TODO: Print out paths how the problem wants
    }
}

#[derive(Default, Debug)]
struct Node {
    visited: bool,
    children: Vec<usize>,
    // default for Option<T> is None
    parent: Option<usize>,
}

fn make_adjacency_list(parents: Vec<usize>, num_nodes: usize) -> Vec<Node> {
    let mut adj_list: Vec<Node> = Vec::new();
    // Pre-allocate so that all indices are already valid.
    // Allocate an extra slot in the array so that we can 1-index, like the problem desc.
    adj_list.resize_with(num_nodes + 1, Default::default);

    for (idx, &parent) in parents.iter().enumerate() {
        // We just leave the root node alone, the default Struct values are acceptable.
        // We use parent: None to find the root later.
        if idx + 1 != parent {
            // Otherwise, add a child to this parent.
            // Create a scope so that the `parent_node` reference
            // goes away, so that we can mutably borrow something else.
            {
                let parent_node = adj_list.get_mut(parent).unwrap();
                parent_node.children.push(idx + 1);
            }

            let child_node = adj_list.get_mut(idx + 1).unwrap();
            child_node.parent = Some(parent);
        }
    }

    // The first element is a dud, give it a dud parent so that only
    // one node in the adj_list has parent: None, the root
    adj_list.get_mut(0).unwrap().parent = Some(0);

    adj_list
}

fn get_layer_numbers(adj_list: &[Node]) -> Vec<(usize, usize)> {
    // (node_id, layer_num)
    // layer of ID is NUM
    let mut layers = Vec::new();

    let (root_node, _) = adj_list
        .iter()
        .enumerate()
        .find(|(idx, node)| node.parent.is_none())
        .unwrap();

    let mut frontier = VecDeque::new();
    frontier.push_back((root_node, 1));
    while !frontier.is_empty() {
        let (current, layer) = frontier.pop_front().unwrap();
        layers.push((current, layer));

        for &c in adj_list.get(current).unwrap().children.iter() {
            frontier.push_back((c, layer + 1))
        }
    }

    // conveniently, this adds the layers in ascending order!
    // so that the leafs are all at the end of the list.
    layers
}

fn get_paths(adj_list: &mut [Node], layers: &mut Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut paths = Vec::new();

    // I'm a bit concerned about performance, because this loop is going
    // to process EVERY node. So the performance is going to be O(n * something)?
    while !layers.is_empty() {
        // Due to the sort order of `layers` this will always be
        // the lowest non-processed node
        let (start_node, layer) = layers.pop().unwrap();

        if !adj_list.get(start_node).unwrap().visited {
            let mut path = vec![start_node];
            let mut current = start_node;

            while let Some(parent) = adj_list.get(current).unwrap().parent {
                if adj_list.get(parent).unwrap().visited {
                    break;
                }

                path.push(parent);
                adj_list.get_mut(current).unwrap().visited = true;
                adj_list.get_mut(parent).unwrap().visited = true;
                current = parent;
            }
            path.reverse();
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
