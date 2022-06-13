pub fn main() {
    let t = read!(usize);
    for i in 0..t {
        let _n = read!(usize);
        let a = read_vec!(i64);
        let b = read_vec!(i64);

        let (max_diff_a, max_diff_b) = a.iter().zip(b.iter()).max_by_key(|(&ai, &bi)| ai - bi).unwrap();
        let max_diff = max_diff_a - max_diff_b;
        if max_diff < 0 {
            println!("NO");
            // Next test!
            continue;
        }
        let updated_a: Vec<i64> = a.iter().map(|&ai| (ai - max_diff).max(0)).collect();

        let mut valid = true;

        for (&ai, &bi) in updated_a.iter().zip(b.iter()) {
            if ai != bi {
                valid = false;
                break;
            }
        }

        if valid {
            println!("YES");
        } else {
            println!("NO");
        }
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

