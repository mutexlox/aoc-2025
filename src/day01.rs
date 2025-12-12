use aoc_2025::util;

#[derive(Debug, Copy, Clone)]
enum Rotations {
    Left(i32),
    Right(i32),
}

fn count_zeroes(start: i32, modulus: i32, rotations: &[Rotations]) -> (usize, i32) {
    let mut zeroes = 0;
    let mut pass_zeroes = 0;
    let mut cur = start;
    for &r in rotations {
        cur = match r {
            Rotations::Left(x) => {
                if x >= cur {
                    // Pass zero once, unless started there...
                    if cur > 0 {
                        pass_zeroes += 1;
                    }
                    // and now that we've gotten there, any more times?
                    pass_zeroes += (x - cur) / modulus;
                }
                (cur - x).rem_euclid(modulus)
            }
            Rotations::Right(x) => {
                pass_zeroes += (x + cur) / modulus;
                (x + cur) % modulus
            }
        };
        if cur == 0 {
            zeroes += 1;
        }
    }
    (zeroes, pass_zeroes)
}

fn main() {
    let mut rotations = vec![];
    for line in util::get_lines().map_while(Result::ok) {
        let (c, rest) = line.split_at(1);
        let amount = rest.parse::<i32>().unwrap();
        rotations.push(match c {
            "R" => Rotations::Right(amount),
            "L" => Rotations::Left(amount),
            _ => panic!("invalid char {}", c),
        });
    }
    println!("{:?}", count_zeroes(50, 100, &rotations));
}
