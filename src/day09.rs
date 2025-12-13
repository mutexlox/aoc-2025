use aoc_2025::util;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

fn largest_hypothetical_rect_area(coords: &[(i64, i64)]) -> i64 {
    let mut max = i64::MIN;
    for (i, (x1, y1)) in coords.iter().enumerate() {
        for (x2, y2) in coords.iter().skip(i + 1) {
            max = max.max(((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1));
        }
    }
    max
}

fn largest_hypothetical_area_with_green(coords: &[(i64, i64)]) -> i64 {
    let (mut x_values, mut y_values): (Vec<_>, Vec<_>) = coords.iter().cloned().unzip();
    // An index i in |x_values| corresponds to x_values[i] in |coords|. similar for y_value
    x_values = x_values.iter().unique().sorted().cloned().collect();
    y_values = y_values.iter().unique().sorted().cloned().collect();

    // Reverse the maps
    let mut reversed_x = HashMap::new();
    for (i, orig_x) in x_values.iter().enumerate() {
        reversed_x.insert(orig_x, i);
    }
    let mut reversed_y = HashMap::new();
    for (i, orig_y) in y_values.iter().enumerate() {
        reversed_y.insert(orig_y, i);
    }

    let mut inside = HashMap::new();

    // Build the "walls"
    for (i, c1) in coords.iter().enumerate() {
        let c1x = reversed_x[&c1.0];
        let c1y = reversed_y[&c1.1];

        let c2 = coords[(i + 1) % coords.len()];
        let c2x = reversed_x[&c2.0];
        let c2y = reversed_y[&c2.1];

        if c1x == c2x {
            let start = c1y.min(c2y);
            let stop = c1y.max(c2y);
            for y in start..=stop {
                inside.insert((y, c1x), true);
            }
        } else {
            let start = c1x.min(c2x);
            let stop = c1x.max(c2x);
            for x in start..=stop {
                inside.insert((c1y, x), true);
            }
        }
    }

    for x in 0..x_values.len() {
        for y in 0..y_values.len() {
            if inside.contains_key(&(y, x)) {
                continue;
            }
            let mut visited_this_trip = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back((x, y));
            let mut enclosed_section = true;
            while let Some((xi, yi)) = queue.pop_front() {
                if inside.contains_key(&(yi, xi)) || visited_this_trip.contains(&(xi, yi)) {
                    continue;
                }
                visited_this_trip.insert((xi, yi));
                for d in util::Direction::directions() {
                    if let Some((xj, yj)) = d.neighbor((xi, yi), x_values.len(), y_values.len()) {
                        queue.push_back((xj, yj));
                    } else {
                        enclosed_section = false;
                    }
                }
            }
            for &(xi, yi) in visited_this_trip.iter() {
                inside.insert((yi, xi), enclosed_section);
            }
        }
    }

    let mut max = i64::MIN;
    for (i, (x1, y1)) in coords.iter().enumerate() {
        for (x2, y2) in coords.iter().skip(i + 1) {
            let x1_shrunk = reversed_x[&x1];
            let x2_shrunk = reversed_x[&x2];
            let y1_shrunk = reversed_y[&y1];
            let y2_shrunk = reversed_y[&y2];
            let x_min = x1_shrunk.min(x2_shrunk);
            let x_max = x1_shrunk.max(x2_shrunk);
            let y_min = y1_shrunk.min(y2_shrunk);
            let y_max = y1_shrunk.max(y2_shrunk);
            let mut works = true;
            'outer: for x in x_min..=x_max {
                for y in y_min..=y_max {
                    if inside.get(&(y, x)) == Some(&false) {
                        works = false;
                        break 'outer;
                    }
                }
            }

            if works {
                max = max.max(((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1));
            }
        }
    }
    max
}

fn main() {
    let mut coords = Vec::new();
    for line in util::get_lines().map_while(Result::ok) {
        let (x, y) = line.split_once(',').unwrap();
        coords.push((x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()));
    }
    println!("{:?}", largest_hypothetical_rect_area(&coords));
    println!("{:?}", largest_hypothetical_area_with_green(&coords));
}
