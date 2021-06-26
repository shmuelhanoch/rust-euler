// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

// What is the sum of the digits of the number 2^1000?

use num::{BigInt, FromPrimitive};

fn main() {
    println!(
        "{}",
        <BigInt as FromPrimitive>::from_i32(2)
            .unwrap()
            .pow(1000)
            .to_str_radix(10)
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .sum::<u32>()
    );
}
