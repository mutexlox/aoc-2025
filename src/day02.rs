use aoc_2025::util;

fn is_silly_number(i: usize) -> bool {
    let digits = i.ilog10() + 1;
    if !digits.is_multiple_of(2) {
        return false;
    }
    let mul = 10usize.pow(digits / 2);
    i % mul == i / mul
}

fn is_silly_number_ext(i: usize) -> bool {
    let digits = i.ilog10() + 1;
    for pat_length in 1..=digits / 2 {
        if !digits.is_multiple_of(pat_length) {
            continue;
        }
        let mul = 10usize.pow(pat_length);
        let mut works = true;
        let mut temp = i;
        let candidate = temp % mul;
        while temp > 0 {
            if temp % mul != candidate {
                works = false;
                break;
            }
            temp /= mul;
        }
        if works {
            return true;
        }
    }
    false
}

fn count_silly_numbers_in_range(start: usize, end: usize) -> usize {
    let mut count = 0;
    for i in start..=end {
        if is_silly_number(i) {
            count += i;
        }
    }
    count
}

fn count_ext_silly_numbers_in_range(start: usize, end: usize) -> usize {
    let mut count = 0;
    for i in start..=end {
        if is_silly_number_ext(i) {
            count += i;
        }
    }
    count
}

fn main() {
    let input = util::get_all_input();
    let ranges = input
        .trim()
        .split(",")
        .map(|s| {
            let (a, b) = s.split_once("-").unwrap();
            (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();
    println!(
        "{}",
        ranges
            .iter()
            .fold(0, |acc, (a, b)| acc + count_silly_numbers_in_range(*a, *b))
    );

    println!(
        "{}",
        ranges.iter().fold(0, |acc, (a, b)| acc
            + count_ext_silly_numbers_in_range(*a, *b))
    );
}
