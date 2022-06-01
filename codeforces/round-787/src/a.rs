pub fn main() {
    let t = read!(u64);

    for _ in 0..t {
        let input = read_vec!(i64);
        let dog_food = input[0];
        let cat_food = input[1];
        let universal_food = input[2];
        let dogs = input[3];
        let cats = input[4];

        let unfed_dogs = i64::max(dogs-dog_food, 0);
        let unfed_cats = i64::max(cats-cat_food, 0);

        if (unfed_dogs + unfed_cats) > universal_food {
            println!("NO");
        } else {
            println!("YES");
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

