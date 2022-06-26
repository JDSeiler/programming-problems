use std::collections::HashSet;

pub fn main() {
    let t = read!(usize);
    for _ in 0..t {
        let n = read!(usize);
        let a = read_vec!(u64);

        /*
         * Each a_i could be at most 10^9. Because every operation
         * applied to a number divides it by 2, any a_i could potentially
         * be AT MOST ~30 different numbers.
         *
         * For each a_i \in a, we compute a set p_i, which is the set of some_numbers
         * a_i COULD become.
         *
         * Let p be a collection of all p_i.
         *
         * Here is where this problem is modeled as a bipartite graph:
         * Let B = 1..n 
         * Let A = 1..n 
         * We add an edge (b,a) if b \in p_a, that is, the number `b` could 
         * be produced by the value a_i through some amount of the operation.
         *
         * Check that every vertex has degree at least 1.
         * Then check that every vertex in B with degree 1 is already paired
         * with a unique element in A.
         * */

        let mut p: Vec<HashSet<u64>> = vec![HashSet::new(); n];
        for (i, &a_i) in a.iter().enumerate() {
            build_set_of_possibles(a_i, &mut p[i]);
        }

        // vertices in B are first set of n, then vertices in A are second set of n
        let mut degree = vec![0; n*2];
        let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n*2];

        // For each p_a (value we're given)
        for (a, p_a) in p.iter().enumerate() {
            // For each value we need in [1,n]
            for b in 0..n {
                // Could this p_a produce this b?
                if p_a.contains(&((b+1) as u64)) {
                    degree[a+n] += 1;
                    degree[b] += 1;

                    graph[a+n].push(b);
                    graph[b].push(a+n);
                }
            }
        }

        let mut exclusive_vertices = Vec::new();
        // Only consider vertices in B 
        for (idx, &d) in degree.iter().enumerate().take(n) {
            if d == 1 {
                exclusive_vertices.push(graph[idx][0]);
            }
        }

        if !is_unique(&mut exclusive_vertices) {
            // Two numbers we need:
            // 1. Can only be produced by a single value in the input
            // 2. Both numbers share the same value.
            println!("NO");
            continue;
        }

        if degree.contains(&0) {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn is_unique(v: &mut Vec<usize>) -> bool {
    // This is a funky hack but it's a tiny amount of code.
    let start_len = v.len();

    v.sort_unstable();
    // dedup only removes neighboring duplicates, which is why we sort first.
    v.dedup();

    start_len == v.len()
}

fn build_set_of_possibles(start_val: u64, set: &mut HashSet<u64>) {
    set.insert(start_val);
    let mut current = start_val;
    while current > 1 {
        // Integer division takes care of the flooring for us.
        current /= 2;
        set.insert(current);
    }
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
    ($type:ty) => {
        {
            let mut inner = String::new();
            std::io::stdin().read_line(&mut inner).expect("read! was unable to read from stdin");
            inner.trim().parse::<$type>().expect("read! was unable to parse into the desired type")
        }
    };
}

/**
 Reads a full line of input and returns it as a `String`.
 */
#[allow(unused_macros)]
macro_rules! read_str {
    () => {
        {
            let mut inner = String::new();
            std::io::stdin().read_line(&mut inner).expect("read_str! was unable to read from stdin");
            inner.trim().to_owned()
        }
    };
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
    ($type:ty) => {
        {
            let mut inner = String::new();
            std::io::stdin().read_line(&mut inner).unwrap();
            inner
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<$type>().unwrap())
                .collect::<Vec<$type>>()
        }
    };
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
    ($type:ty) => {
        {
            let mut inner = String::new();
            std::io::stdin().read_line(&mut inner).unwrap();
            let v = inner
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<$type>().unwrap())
                .collect::<Vec<$type>>();
            let (a, b) = match &v[..] {
                &[f, s, ..] => (f, s),
                _ => unreachable!()
            };
            (a,b)
        }
    };
}

pub(crate) use read;
pub(crate) use read_str;
pub(crate) use read_vec;
pub(crate) use read_pair;
