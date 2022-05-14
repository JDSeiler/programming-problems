pub fn main() {
    let t = read!(u64);
    'outer:
    for i in 0..t {
        let n = read!(usize);
        let nums: Vec<usize> = read_vec!(usize);
        let mut counts: Vec<u64> = Vec::with_capacity(n);
        counts.resize(n+1, 0);

        for &v in nums.iter() {
            let count = counts.get_mut(v).expect("Invalid index in counts array!?!?");
            *count += 1;

            if *count >= 3 {
                println!("{}", v);
                continue 'outer;
            }
        }
        println!("-1");
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
Reads a full line of input and returns it as a `&str`.
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

pub(crate) use read;
pub(crate) use read_str;
pub(crate) use read_vec;
