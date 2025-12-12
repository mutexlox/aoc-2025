use aoc_2025::util;

#[derive(Debug, Copy, Clone)]
enum Rotations {
    Left(u32),
    Right(u32),
}

fn count_zeroes(start: u32, modulus: u32, rotations: &[Rotations]) -> usize {
    let mut zeroes = 0;
    let mut cur = start;
    for &r in rotations {
        cur = match r {
            Rotations::Left(mut x) => {
                // Reduce to avoid going negative
                x %= modulus;
                if x > cur {
                    modulus - (x - cur)
                } else {
                    cur - x
                }
            }
            Rotations::Right(x) => (x + cur) % modulus,
        };
        if cur == 0 {
            zeroes += 1;
        }
    }
    zeroes
}

fn main() {
    let mut rotations = vec![];
    for line in util::get_lines().map_while(Result::ok) {
        let (c, rest) = line.split_at(1);
        let amount = rest.parse::<u32>().unwrap();
        rotations.push(match c {
            "R" => Rotations::Right(amount),
            "L" => Rotations::Left(amount),
            _ => panic!("invalid char {}", c),
        });
    }
    println!("{}", count_zeroes(50, 100, &rotations));
}
