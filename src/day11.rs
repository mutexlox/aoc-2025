use aoc_2025::util;
use std::collections::HashMap;

fn paths(g: &HashMap<String, Vec<String>>, start: &str, end: &str) -> usize {
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

    paths_helper(g, start, end, &mut HashMap::new())
}

fn paths_through(
    g: &HashMap<String, Vec<String>>,
    start: &str,
    end: &str,
    must_visit: (&str, &str),
) -> usize {
    fn paths_helper(
        g: &HashMap<String, Vec<String>>,
        start: &str,
        end: &str,
        must_visit: (&str, &str),
        saw_must_visit: (bool, bool),
        cache: &mut HashMap<(String, (bool, bool)), usize>,
    ) -> usize {
        if start == end {
            if saw_must_visit.0 && saw_must_visit.1 {
                return 1;
            }
            return 0;
        }
        if let Some(&n) = cache.get(&(start.to_string(), saw_must_visit)) {
            return n;
        }

        let sum = g[start]
            .iter()
            .map(|node| {
                paths_helper(
                    g,
                    node,
                    end,
                    must_visit,
                    (
                        saw_must_visit.0 || node == must_visit.0,
                        saw_must_visit.1 || node == must_visit.1,
                    ),
                    cache,
                )
            })
            .sum();
        cache.insert((start.to_string(), saw_must_visit), sum);
        sum
    }

    paths_helper(
        g,
        start,
        end,
        must_visit,
        (false, false),
        &mut HashMap::new(),
    )
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
    println!("{}", paths_through(&g, "svr", "out", ("dac", "fft")));
}
