//
// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the
// bottom right corner.
//
// How many such routes are there through a 20×20 grid?
//
// Solution: the problem is equivalent to counting permutations of 40 elements, made of two sets of 20 identical elements in each set.
// Using the formula for permutations with repetitions, it's $$40! / 20! * 20!$. Dividing by 20!, we get:
// $(21*...*40) / 20!$
// This representation turns out to fit in i128 without an overflow.

fn main() {
    println!("{}", (21..41).product::<i128>() / (1..21).product::<i128>());
}
