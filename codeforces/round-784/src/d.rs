
pub fn main() {
    let t = read!(u64);
    for _i in 0..t {
        let _n = read!(usize);
        let picture = read_str!();
        let compressed = compress(&picture);
        let subpictures: Vec<&str> = compressed.split('W').collect();

        let mut valid = true;
        for &pic in subpictures.iter() {
            if pic.len() == 1 {
                println!("NO");
                valid = false;
                break;
            }
        }

        if valid {
            println!("YES");
        }
    }
}

fn compress(picture: &str) -> String {
    // Strings are not indexable in Rust for good reasons, but this is Codeforces
    // and I know the input is ASCII, so this is fine.
    let chars: Vec<char> = picture.chars().collect();
    let mut last_char = chars[0];

    let mut result = String::new();

    for i in 0..chars.len() {
        let this_char = chars[i];

        if i == chars.len()-1 {
            // NORMALLY, when this_char != last_char, we add the last_char
            // So we have to tack on this extra condition so that we don't miss it.
            if last_char != this_char {
                result.push(last_char);
            }

            // If we're at the end, we're always going to have to append the last character..
            // It's as if this_char != last_char, except last_char is "nothing"
            result.push(this_char);
        } else if this_char != last_char {
            // We must have finished a contiguous string of the same character.
            // So, we append ONE of that character to the output
            result.push(last_char);
            last_char = this_char
        } else {
            // So long as the character is not changing, do nothing.
            // There charater hasn't changed so don't bother updating last_char either
            continue;
        }
    }
    return result;
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
