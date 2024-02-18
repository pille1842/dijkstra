/**
 * Dijkstra prime finding algorithm implementation (for fun)
 * Author: Eric Haberstroh <gpg@erixpage.de>
 * Date: 2024-02-18
 *
 * Inspired by b001: https://www.youtube.com/watch?v=fwxjMKBMR7s
 */

use std::env;

#[derive(Debug)]
struct Element {
    number: u64,
    square: u64,
}

impl Element {
    fn new(number: u64) -> Self {
        Element {
            number,
            square: match number.checked_pow(2) {
                Some(val) => val,
                None => panic!("Cannot calculate {}^2, number is too large!", number),
            },
        }
    }
}

pub fn primes(limit: u64) -> Vec<u64> {
    let mut pool: Vec<Element> = vec![Element::new(2)];
    let mut primes: Vec<u64> = vec![2];

    for i in 3..=limit {
        let min = pool.iter().min_by_key(|el| el.square).unwrap();
        if i < min.square {
            primes.push(i);
            pool.push(Element::new(i));
        } else if i == min.square {
            for el in pool.iter_mut() {
                if el.square == i {
                    el.square += el.number;
                }
            }
        }
    }

    primes
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Expecting one argument: upper limit");
    }

    let limit: u64 = match args[1].parse() {
        Ok(num) => num,
        Err(error) => panic!("Invalid argument: {:?}", error),
    };

    println!("{:?}", primes(limit));
}
