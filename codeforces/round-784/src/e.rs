
pub fn main() {
    let t = read!(u64);
    for i in 0..t {
        let n = read!(usize);
        let mut strs: Vec<String> = Vec::with_capacity(n);
        for i in 0..n {
            strs.push(read_str!());
        }
        let mut string_table: Vec<Vec<u64>> = populate_table(&strs);

        let mut total_count: u64 = 0;
        for s in strs.iter() {
            total_count += count_pairs(s, &mut string_table);
        }

        println!("{}", total_count);
    }
}

fn count_pairs(s: &str, table: &mut Vec<Vec<u64>>) -> u64 {
    let (start, end)= str_to_indices(s);
    let mut row_sum = 0;
    for i in 0..11 {
        row_sum += table[start][i];
    }

    let mut col_sum = 0;
    for i in 0..11 {
        col_sum += table[i][end];
    }

    let count = row_sum + col_sum - (2*table[start][end]);
    table[start][end] -= 1;
    return count;
}

fn populate_table(strs: &Vec<String>) -> Vec<Vec<u64>> {
    let mut string_table: Vec<Vec<u64>> = Vec::with_capacity(11);
    string_table.resize_with(11, || { Vec::with_capacity(11) });

    for i in 0..11 {
        string_table[i].resize(11, 0)
    }

    for s in strs.iter() {
        let (start, end) = str_to_indices(s);

        string_table[start][end] += 1;
    }

    return string_table;
}

fn str_to_indices(s: &str) -> (usize, usize) {
    let start_index = char_to_index(s.chars().nth(0).unwrap());
    let end_index = char_to_index(s.chars().nth(1).unwrap());
    (start_index, end_index)
}

fn char_to_index(c: char) -> usize {
    match c {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        'i' => 8,
        'j' => 9,
        'k' => 10,
        _ => panic!("Codeforces lied! Got a char that isn't in the range a..k")
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
