#[macro_use] extern crate lazy_static;

extern crate itertools;

use itertools::Itertools;
use std::fmt;
use std::cmp;
use std::fs::File;
use std::io;
use std::io::{
    BufRead,
    BufReader,
    Stdin,
};

lazy_static! {
    static ref IO_HANDLE: Stdin = io::stdin();
}

enum OrderState {
    Ascending,
    Descending,
    Unordered
}

impl OrderState {
    fn from_word(s: &str) -> OrderState {
        match s {
            _ if s == sort_string(s, |&a, &b| a.cmp(&b)) => OrderState::Ascending,
            _ if s == sort_string(s, |&a, &b| b.cmp(&a)) => OrderState::Descending,
            _ => OrderState::Unordered
        }
    }
}

#[inline]
fn sort_string<F: Fn(&char, &char) -> cmp::Ordering>(s: &str, f: F) -> String {
    s.chars().sorted_by(f).iter().cloned().collect()
}

struct OrderResult {
    word: String,
    state: OrderState,
}

impl OrderResult {
    fn from_word<S: Into<String>>(s: S) -> OrderResult {
        let word = s.into();

        OrderResult {
            state: OrderState::from_word(&word),
            word: word,
        }
    }
}

impl fmt::Display for OrderResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.word, match self.state {
            OrderState::Ascending => "IN ORDER",
            OrderState::Descending => "REVERSE ORDER",
            OrderState::Unordered => "NOT IN ORDER",
        })
    }
}

pub fn main() {
    let input: Box<BufRead> = match load_input() {
        Some(input) => input,
        None => {
            if std::env::args().any(|s| s == "-p" || s == "--pipe") {
                Box::new(IO_HANDLE.lock())
            } else {
                println!("No input provided");
                std::process::exit(1);
            }
        },
    };

    for result in parse_input(input.lines().filter_map(|line| line.ok())) {
        println!("{}", result);
    }
}

// Here I'm using a boxed iterator because I'm just kind of assuming that, otherwise, the return
// type is inexpressible. I could do this with just vectors and slices or whatever instead, but
// I'm kind of keeping up a habit of treating memory and allocations as more important than
// computation cycles--something I picked up writing C#.
fn parse_input<'a, I: Iterator<Item=String> + 'a>(input: I) -> Box<Iterator<Item=OrderResult> + 'a> {
    Box::new(input.map(|s| OrderResult::from_word(s)))
}

fn load_input() -> Option<Box<BufRead>> {
    std::env::args().nth(1)
        .and_then(|path| File::open(&path).ok())
        .map(|file| Box::new(BufReader::new(file)) as Box<BufRead>)
}
