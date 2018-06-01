struct FibGen {
    a: u64,
    b: u64,
}

impl Iterator for FibGen {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let temp = self.b;
        self.b += self.a;
        self.a = temp;

        Some(self.a)
    }
}

fn main() {
    let n: u64 = FibGen { a: 1, b: 1 }
        .take_while(|x| *x <= 4000000)
        .filter(|x| *x % 2 == 0)
        .sum();

    println!("{}", n)
}
