use std::io::{prelude::*, self};

use rand_api::evaluate;

fn main() {
    for line in io::stdin().lines() {
        match line {
            Ok(line) => println!("{}", evaluate(&line)),
            Err(_) => println!("Error while reading line"),
        }
    }
}