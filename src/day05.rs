use aoc_2025::util;
use std::ops::RangeInclusive;

fn count_fresh(ranges: &[RangeInclusive<i64>], nums: &[i64]) -> usize {
    let mut count = 0;
    for n in nums {
        if ranges.iter().any(|r| r.contains(n)) {
            count += 1;
        }
    }
    count
}

fn total_fresh(ranges: &mut [RangeInclusive<i64>]) -> i64 {
    ranges.sort_by_key(|r| *r.start());
    let mut merged = Vec::new();
    let mut last = ranges[0].clone();
    for r in ranges.iter().skip(1) {
        if r.start() <= last.end() {
            last = *last.start()..=*r.end().max(last.end());
        } else {
            merged.push(last);
            last = r.clone();
        }
    }
    merged.push(last);
    merged
        .iter()
        .fold(0, |acc, r| acc + r.end() - r.start() + 1)
}

fn main() {
    let mut ranges = Vec::new();
    let mut nums = Vec::new();
    let mut done_ranges = false;
    for line in util::get_lines().map_while(Result::ok) {
        if !done_ranges {
            if line.is_empty() {
                done_ranges = true;
                continue;
            }
            let (l, r) = line.split_once('-').unwrap();
            ranges.push(l.parse::<i64>().unwrap()..=r.parse::<i64>().unwrap());
        } else {
            nums.push(line.parse::<i64>().unwrap());
        }
    }
    println!("{}", count_fresh(&ranges, &nums));
    println!("{}", total_fresh(&mut ranges));
}
