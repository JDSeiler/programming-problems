use std::iter::FromIterator;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Parity {
    Odd,
    Even,
}

// This is vestigial from when I was trying a solution with
// a bunch of operator overloading. Maybe it's still needed for
// the FromIterator impl? IDK.
#[derive(Debug)]
struct ParityVector(Vec<Parity>);

impl FromIterator<Parity> for ParityVector {
    fn from_iter<I: IntoIterator<Item = Parity>>(iter: I) -> Self {
        let mut c = Vec::new();

        for i in iter {
            c.push(i);
        }

        ParityVector(c)
    }
}

pub fn main() {
    let t = read!(u64);
    for i in 0..t {
        let n = read!(usize);
        let nums: Vec<u64> = read_vec!(u64);

        let parity: ParityVector = nums
            .iter()
            .map(|&n| match n % 2 {
                0 => Parity::Even,
                1 => Parity::Odd,
                _ => panic!("Impossible result mod 2"),
            })
            .collect();

        if is_all_same(&parity) {
            println!("YES")
        } else {
            if is_alternating(&parity) {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}

fn is_alternating(ParityVector(p): &ParityVector) -> bool {
    let mut last_parity = p.get(0).unwrap();
    for i in 1..p.len() {
        if last_parity == p.get(i).unwrap() {
            return false;
        } else {
            last_parity = p.get(i).unwrap();
        }
    }
    return true;
}

fn is_all_same(ParityVector(p): &ParityVector) -> bool {
    let mut is_all_same = true;
    return if let Some(&required_parity) = p.get(0) {
        for &parity in p.iter() {
            if parity != required_parity {
                return false;
            }
        }
        is_all_same
    } else {
        // If the array is empty, trivially everything is the same.
        is_all_same
    };
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
