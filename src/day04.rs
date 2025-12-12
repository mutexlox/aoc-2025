use aoc_2025::util;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum GridSpot {
    Empty,
    Paper,
}

fn count_surrounding_paper(grid: &[Vec<GridSpot>], point: (usize, usize)) -> usize {
    let mut count = 0;
    for d in util::Direction::directions() {
        if let Some((nx, ny)) = d.neighbor(point, grid.len(), grid[0].len()) {
            if grid[nx][ny] == GridSpot::Paper {
                count += 1;
            }
        }
    }
    count
}

fn count_accessible_paper(grid: &[Vec<GridSpot>], threshold: usize) -> usize {
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != GridSpot::Paper {
                continue;
            }
            let temp = count_surrounding_paper(grid, (i, j));
            if temp < threshold {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let mut grid = Vec::new();
    for line in util::get_lines().map_while(Result::ok) {
        let mut vec = Vec::new();
        for c in line.chars() {
            vec.push(match c {
                '.' => GridSpot::Empty,
                '@' => GridSpot::Paper,
                _ => panic!("bad char {}", c),
            });
        }
        grid.push(vec);
    }

    println!("{}", count_accessible_paper(&grid, 4));
}
