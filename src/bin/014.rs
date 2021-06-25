//  Longest Collatz sequence - Problem 14
//
// The following iterative sequence is defined for the set of positive integers:
//
// n → n/2 (n is even)
// n → 3n + 1 (n is odd)
//
// Using the rule above and starting with 13, we generate the following sequence:
// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
//
// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet
// (Collatz Problem), it is thought that all starting numbers finish at 1.
//
// Which starting number, under one million, produces the longest chain?
//
// NOTE: Once the chain starts the terms are allowed to go above one million.
//
// Solution: simple iteration + memoization using a hash map

use std::collections::HashMap;

fn main() {
    let mut m = HashMap::new();
    m.insert(1, 1);
    let mut max = 1;
    let mut arg_max = 1;
    for n in 2..1000000 {
        if !m.contains_key(&n) {
            let curr_max = collatz_len(n, &mut m);
            if curr_max > max {
                max = curr_max;
                arg_max = n;
            }
        }
    }
    println!("{}", arg_max);
}

fn collatz_len(n: i64, m: &mut HashMap<i64, i64>) -> i64 {
    let res = match m.get(&n) {
        Some(res) => *res,
        None => {
            1 + if n % 2 == 0 {
                collatz_len(n / 2, m)
            } else {
                collatz_len(n * 3 + 1, m)
            }
        }
    };
    m.insert(n, res);
    res
}
