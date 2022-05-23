pub fn main() {
    let t: u64 = read!(u64);
    for _i in 0..t {
        let n: usize = read!(usize);
        let forward_candies: Vec<u64> = read_vec!(u64);
        let mut backward_candies: Vec<u64> = forward_candies.to_owned();
        backward_candies.reverse();

        let forward_sums = partial_sums(&forward_candies);
        let backward_sums = partial_sums(&backward_candies);

        let mut best_found_so_far: Option<usize> =  None;

        forward_sums.iter().enumerate().for_each(|(f_idx, w)| {
            match backward_sums.binary_search(w) {
                Ok(b_idx) => {
                    if pair_valid(f_idx, b_idx, n) && f_idx + b_idx >= best_found_so_far.unwrap_or(0) {
                        best_found_so_far = Some(f_idx + b_idx);
                    }
                },
                Err(_) => {}
            }
        });

        match best_found_so_far {
            // The arrays are 0 indexed, but a valid answer of 0,0 is actually 2 candies eaten
            Some(ans) => println!("{}", ans+2),
            None => println!("{}", 0)
        }
    }
}

fn pair_valid(fidx: usize, bidx: usize, n: usize) -> bool {
    // Pairs are not allowed to cross
    fidx < (n - bidx - 1)
}

fn partial_sums(candy: &Vec<u64>) -> Vec<u64> {
    let mut partial_sums: Vec<u64> = Vec::with_capacity(candy.len());
    for (idx, &c) in candy.iter().enumerate() {
        let total_so_far = match idx {
            0 => 0,
            anything_else => *partial_sums.get(anything_else-1).unwrap()
        };

        partial_sums.push(total_so_far + c);
    }

    return partial_sums;
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
Reads a full line of input and returns it as a `&str`.
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

pub(crate) use read;
pub(crate) use read_str;
pub(crate) use read_vec;
