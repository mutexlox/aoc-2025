use aoc_2025::util;
use std::collections::HashMap;

fn paths(g: &HashMap<String, Vec<String>>, start: &str, end: &str) -> usize {
    let mut cache = HashMap::new();

    fn paths_helper(
        g: &HashMap<String, Vec<String>>,
        start: &str,
        end: &str,
        cache: &mut HashMap<String, usize>,
    ) -> usize {
        if start == end {
            return 1;
        }
        if let Some(&n) = cache.get(start) {
            return n;
        }
        let sum = g[start]
            .iter()
            .map(|node| paths_helper(g, node, end, cache))
            .sum();
        cache.insert(start.to_string(), sum);
        cache[start]
    }

    paths_helper(g, start, end, &mut cache)
}

fn main() {
    let mut g = HashMap::new();
    for line in util::get_lines().map_while(Result::ok) {
        let (node, rest) = line.split_once(':').unwrap();
        let dests = rest
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        g.insert(node.to_string(), dests);
    }
    println!("{}", paths(&g, "you", "out"));
}
