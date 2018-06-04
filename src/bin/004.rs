// Problem 4 - Largest Palindrome Product
//
// "A palindromic number reads the same both ways. The largest palindrome made from
// the product of two 2-digit numbers is 9009 = 91 × 99.
//
// Find the largest palindrome made from the product of two 3-digit numbers."
//
// Actually, it's not so hard to solve it without a computer. But my current
// motivation is to learn Rust :)

#[macro_use]
extern crate itertools;

fn is_palindrome(n: &i32) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

fn main() {
    let n_ = iproduct!(100..1000, 100..1000)
        .map(|(x, y)| x * y)
        .filter(is_palindrome)
        .max();

    match n_ {
        Some(n) => println!("{}", n),
        _ => unreachable!(),
    }
}
