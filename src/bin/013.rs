// Large sum - Problem 13

// Work out the first ten digits of the sum of the following one-hundred
// 50-digit numbers.
// (...)
//
// Solution:
//
// Read the 11-digit prefix from each number, sum them up as 64bit integers.
// Finally, take only the first 10 digits.
//
// The program assumes input from stdin. To run with the original input:
//
// cargo run --bin 012 < src/resources/013.txt

use std::io;
use std::char;
use std::io::prelude::*;

fn main() {
    let mut total = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let mut curr: i64 = 0;
        for (i, c) in line.unwrap().chars().enumerate() {
            curr = 10 * curr + char::to_digit(c, 10).unwrap() as i64;
            if i==10 {
                break;
            }
        }
        total += curr;
    }
    println!("{}", &total.to_string()[..10])
}
