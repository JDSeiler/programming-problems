pub fn main() {
    let t = read!(usize);
    
    for _ in 0..t {
        let v = read_vec!(i64);
        let (a, b, c) = match &v[..] {
            &[f, s, t, ..] => (f, s, t),
            _ => unreachable!()
        };
        if is_arithmetic_seq(a, b, c) { println!("YES");
            continue;
        }

        if can_change_a(a, b, c) || can_change_b(a, b, c) || can_change_c(a, b, c) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}

/*
 * We don't know if the sequence increase or decreases from left to right.
 * But that's ok, because by the definition of the sequence we're just adding
 * some integer "d" each time. So as long as we're consistent in our arithmetic 
 * everything will work our. For out purposes, we assume that we're always 
 * ADDING `d` when going left to right.
 *
 * */

fn can_change_a(a: i64, b: i64, c: i64) -> bool {
    /*
     * Given: a, b, c 
     * c - b = d (by definition of sequence) 
     * a' + d = b :: a' == some value that would give a valid sequence
     * a' / a must be a whole, positive number
     * */
    let assumed_difference = c - b;
    let necessary_a = b - assumed_difference;

    necessary_a % a == 0 && necessary_a / a > 0
}

fn can_change_b(a: i64, b: i64, c: i64) -> bool {
    /*
     * Given: a, b, c 
     * (c - a) / 2 = d (by def.) 
     * b' = c - d 
     * b' / b must be a whole, positive number
     * */

    // If the difference between the first and last element of the sequence 
    // isn't divisible by 2, then there is no value b' that could equally
    // divide the space between them, and thus an arithmetic sequence cannot
    // be constructed by changing b.
    if (c-a) % 2 != 0 {
        return false;
    }
    let assumed_difference = (c - a) / 2;
    let necessary_b = c - assumed_difference;

    necessary_b % b == 0 && necessary_b / b > 0
}

fn can_change_c(a: i64, b: i64, c: i64) -> bool {
    /*
     * Given: a, b, c 
     * b - a = d (by def.)
     * c' = b + d (by def.)
     * c' / c must be a whole, positive number
     * */
    let assumed_difference = b - a;
    let necessary_c = b + assumed_difference;

    necessary_c % c == 0 && necessary_c / c > 0
}



fn is_arithmetic_seq(a: i64, b: i64, c: i64) -> bool {
    a-b == b-c
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

