use aoc_2025::util;
use std::collections::{HashMap, HashSet};

fn count_splits(start: usize, max: usize, splitters: &HashSet<(usize, usize)>) -> (usize, usize) {
    let mut count = 0;

    let mut beam_locs = HashMap::new();
    beam_locs.insert(start, 1);
    for i in 1..max {
        let mut new_locs = HashMap::new();
        for (b, prev_count) in beam_locs {
            if splitters.contains(&(i, b)) {
                *new_locs.entry(b - 1).or_insert(0) += prev_count;
                *new_locs.entry(b + 1).or_insert(0) += prev_count;
                count += 1;
            } else {
                *new_locs.entry(b).or_insert(0) += prev_count;
            }
        }
        beam_locs = new_locs;
    }

    (count, beam_locs.values().sum())
}

fn main() {
    let mut start = usize::MAX;
    let mut splitters = HashSet::new();
    let mut max_i = 0;
    for (i, line) in util::get_lines().map_while(Result::ok).enumerate() {
        if start == usize::MAX {
            start = line.find('S').unwrap();
        } else {
            let indices = line.match_indices('^').map(|(j, _)| j);
            for idx in indices {
                splitters.insert((i, idx));
            }
        }
        max_i = i;
    }
    max_i += 1;
    println!("{:?}", count_splits(start, max_i, &splitters));
}
