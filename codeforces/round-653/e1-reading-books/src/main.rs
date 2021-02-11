use std::io;
use std::io::{BufRead, StdinLock};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Book {
    size: usize,
    alice_likes: bool,
    bob_likes: bool
}

impl Ord for Book {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare by size and flip the ordering
        // This forces Rust's binaryHeap to function as a min-heap
        other.size.cmp(&self.size)
    }
}

impl PartialOrd for Book {
    // This trait impl is required for mathy/formal reasons
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_problem_info(handle: &mut StdinLock) -> (usize, usize) {
    let mut buffer: String = String::with_capacity(10);
    handle.read_line(&mut buffer).unwrap();
    let mut numbers: Vec<usize> = buffer
        .split_ascii_whitespace()
        .map(|s| {s.parse::<usize>().unwrap()})
        .collect();
    let n = numbers.get(0).unwrap();
    let k = numbers.get(1).unwrap();
    (*n,*k)
}

fn read_books(handle: &mut StdinLock, n: usize) -> Vec<Book> {
    let mut buffer: String = String::with_capacity(10);
    let mut books: Vec<Book> = Vec::with_capacity(n);
    for _i in 0..n {
        handle.read_line(&mut buffer).unwrap();
        let mut components = buffer
            .split_ascii_whitespace()
            .enumerate();

        let mut new_book: Book = Book { size: 0, alice_likes: false, bob_likes: false };
        for (i, val) in components {
            match i {
                0 => new_book.size = val.parse::<usize>().unwrap(),
                1 => {
                    // Boolean parsing expects "true" or "false", not "1" or "0"
                    // So in this case I need to parse it to a number and compare
                    let numeric_value = val.parse::<usize>().unwrap();
                    if numeric_value == 0 {
                        new_book.alice_likes = false;
                    } else {
                        new_book.alice_likes = true;
                    }
                },
                2 => {
                    let numeric_value = val.parse::<usize>().unwrap();
                    if numeric_value == 0 {
                        new_book.bob_likes = false;
                    } else {
                        new_book.bob_likes = true;
                    }
                }
                _ => panic!("Index should be one of 0, 1, or 2")
            }
        }
        books.push(new_book);
        buffer.clear();
    }
    books
}

// Shared, Alice, and Bob are the ordering of the heaps
fn construct_heaps(books: Vec<Book>) -> (BinaryHeap<Book>, BinaryHeap<Book>, BinaryHeap<Book>) {
    let mut s: BinaryHeap<Book> = BinaryHeap::new();
    let mut a: BinaryHeap<Book> = BinaryHeap::new();
    let mut b: BinaryHeap<Book> = BinaryHeap::new();

    for book in books {
        // By scaling the Alice/Bob books by 2, it's as if
        // the weight of the shared books was cut in half
        // But by multiplying all numbers stay as integers
        if book.alice_likes && book.bob_likes {
            s.push(book);
        } else if book.alice_likes && !book.bob_likes {
            let actual_book = Book {
                size: book.size * 2,
                alice_likes: true,
                bob_likes: false
            };
            a.push(actual_book);
        } else if !book.alice_likes && book.bob_likes {
            let actual_book = Book {
                size: book.size * 2,
                alice_likes: false,
                bob_likes: true
            };
            b.push(actual_book);
        } else {
            // Do nothing, no one likes the book so we ignore it
        }
    }
    (s, a, b)
}

// S is the size of the shared set of books
// A is the size of the set of books which only Alice enjoys
// B is the size of the set of books which only Bob enjoys
// K is the number of books A and B must read together
fn problem_is_possible(len_s: usize, len_a: usize, len_b: usize, k: usize) -> bool {
    // If the shared set if at least K, it's possible
    if len_s >= k {
        return true;
    }
    // If both individual sets are at least K, it's possible
    if len_a >= k && len_b >= k {
        return true;
    }
    // If either of them are less than K
    if len_a < k || len_b < k {
        // s_x is the amount of books required to be in s
        // in order to supplement x. If x has 2 books but k
        // is 5, s must contain 3 books.
        let s_a = (k - len_a);
        let s_b = (k - len_b);

        // Shared books are shared, so s_a and s_b have no interaction
        // That is, just because Alice "needs" a book from the shared set doesn't
        // mean Bob cannot also read that book.
        return s_a <= len_s && s_b <= len_s
    }

    // Make the compiler happy -- if I have problems, make sure my boolean logic
    // actually covers all cases!
    return false;
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let (n, k) = get_problem_info(&mut handle);
    let books = read_books(&mut handle, n);
    let (mut s, mut a, mut b) = construct_heaps(books);
    let is_possible = problem_is_possible(s.len(), a.len(), b.len(), k);

    if is_possible {
        let mut alice_count = k;
        let mut bob_count = k;

        // MAX HEAPS for the final set of book sizes
        let mut shared_books: BinaryHeap<usize> = BinaryHeap::new();
        let mut alice_books: BinaryHeap<usize> = BinaryHeap::new();
        let mut bob_books: BinaryHeap<usize> = BinaryHeap::new();

        while alice_count > 0 {
            // Get cheapest books out of S and A
            // Only one of these should be None at a time if the problem is possible.
            let s_top;
            match s.peek() {
                None => s_top = usize::max_value(),
                Some(book) => s_top = book.size,
            }

            let a_top;
            match a.peek() {
                None => a_top = usize::max_value(),
                Some(book) => a_top = book.size,
            }

            // If S is cheaper or it's a tie, select the book from S
            if s_top <= a_top {
                // Remove it and place it on its final stack
                s.pop();
                shared_books.push(s_top);

                // Shared books count for bob too
                // bob_count = bob_count - 1;
            } else {
                a.pop();
                // Non shared books have their weight scaled by 2
                // Divide the weight when adding it to the final stack
                alice_books.push(a_top / 2);
            }
            alice_count = alice_count - 1;
        }

        // Checked shared books - create duplicate count
        // to avoid mutating the range while iterating.
        let shared_books_count = shared_books.len();
        for i in 0 .. shared_books_count {
            // We are concerned about LITERAL VALUES
            // SO DESCALE A AND B
            let a_top;
            match a.peek() {
                None => a_top = usize::max_value(),
                Some(book) => a_top = book.size / 2,
            }

            let s_top;
            match s.peek() {
                None => s_top = usize::max_value(),
                Some(book) => s_top = book.size,
            }

            let b_top;
            match b.peek() {
                None => b_top = usize::max_value(),
                Some(book) => b_top = book.size / 2,
            }
            // If a has no books, we're DONE!
            // decrement bob_count by the remaining shared books we can't process
            // We cannot process them because A is out of books.
            if a_top == usize::max_value() {
                bob_count = bob_count - (shared_books.len() - i);
                break;
            }
            let discard_shared_cost = a_top + b_top;
            let keep_shared_cost = shared_books.peek().unwrap().clone();

            if discard_shared_cost < keep_shared_cost {
                alice_books.push(a_top);
                bob_books.push(b_top);

                shared_books.pop();
            }
            bob_count = bob_count - 1;
        }

        while bob_count > 0 {
            let a_books_top;
            // This should never be None because this is alice's final stack
            match alice_books.peek() {
                None => a_books_top = panic!(),
                Some(book_size) => a_books_top = *book_size
            }

            let s_top;
            match s.peek() {
                None => s_top = usize::max_value(),
                Some(book) => s_top = book.size,
            }

            let b_top;
            match b.peek() {
                None => b_top = usize::max_value(),
                Some(book) => b_top = book.size,
            }
            // If we add a shared book...
            let choose_s_cost = s_top - a_books_top;

            // If replacing alice's most expensive book is at least as cheap
            // as the -literal cost- of Bob's cheapest book, do it
            if choose_s_cost <= b_top / 2 {
                alice_books.pop();
                shared_books.push(s_top);
            } else {
                // Otherwise just pick a book from bob's stack
                b.pop();
                bob_books.push(b_top / 2);
            }
            bob_count = bob_count - 1;
        }

        let mut sum: usize = 0;
        let mut count = 0;
        for book in shared_books.iter() {
            sum = sum + *book;
            count = count + 1;
        }
        for book in alice_books.iter() {
            sum = sum + *book;
            count = count + 1;
        }
        for book in bob_books.iter() {
            sum = sum + *book;
            count = count + 1;
        }

        println!("{}", sum);
    } else {
        println!("-1");
    }
}
