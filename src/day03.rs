use aoc_2025::util;

fn max_in_bank(bank: &[u32], digits: usize) -> u64 {
    let mut out = 0;

    let mut start = 0;

    for i in 0..digits {
        let digits_to_leave = digits - i - 1;
        let digits_to_take = bank.len() - start - digits_to_leave;
        let (new_start, digit) = bank
            .iter()
            .enumerate()
            .skip(start)
            .take(digits_to_take)
            .max_by(|(i1, val1), (i2, val2)| {
                if val1 == val2 {
                    // prefer the earlier
                    i2.cmp(i1)
                } else {
                    val1.cmp(val2)
                }
            })
            .unwrap();
        start = new_start + 1;
        out *= 10;
        out += *digit as u64
    }

    out
}

fn max_in_banks(banks: &[Vec<u32>], digits: usize) -> u64 {
    banks.iter().fold(0, |acc, b| acc + max_in_bank(b, digits))
}

fn main() {
    let mut banks = Vec::new();
    for line in util::get_lines().map_while(Result::ok) {
        let mut bank = Vec::new();
        for c in line.chars() {
            bank.push(c.to_digit(10).unwrap());
        }
        banks.push(bank);
    }
    println!("{}", max_in_banks(&banks, 2));
    println!("{}", max_in_banks(&banks, 12));
}
