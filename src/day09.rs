use aoc_2025::util;

fn largest_hypothetical_rect_area(coords: &[(i64, i64)]) -> i64 {
    let mut max = i64::MIN;
    for (i, (x1, y1)) in coords.iter().enumerate() {
        for (x2, y2) in coords.iter().skip(i + 1) {
            max = max.max(((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1));
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
}
