/*
This code is from users coder3101 and ouflak on Stack Overflow:
https://stackoverflow.com/a/57200055

I have modified the macros in the following ways:
1. The macros now return a value instead of magically declaring a variable.
2. The error messages in the `expect` calls are more helpful now.
3. `read_str!` now has a `to_owned` call to avoid ownership/lifetime issues, since the
   macros are returning values, not adding them to the same scope.

HOW TO USE THESE:
1. Copy ALL the code including the three `pub(crate) ...` lines at the bottom
2. Paste the code into the file you're writing your solution in. This is necessary
   because Codeforces/Kattis requires your solution be all in one file.
3. You can now invoke the macros using their names: `read!`, `read_str!` etc.

This method also makes it pretty easy to convert this file into a module later if you
so wish. More or less all you need to do is add `mod <name of this file>;` in `main.rs`
or your `mod.rs` file.
*/

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

pub(crate) use read;
pub(crate) use read_str;
pub(crate) use read_vec;

