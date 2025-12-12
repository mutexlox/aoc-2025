use aoc_2025::util;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum GridSpot {
    Empty,
    Paper,
    EmptyingPaper,
}

fn count_surrounding_paper(grid: &[Vec<GridSpot>], point: (usize, usize)) -> usize {
    let mut count = 0;
    for d in util::Direction::directions() {
        if let Some((nx, ny)) = d.neighbor(point, grid.len(), grid[0].len())
            && grid[nx][ny] != GridSpot::Empty
        {
            count += 1;
        }
    }
    count
}

fn count_accessible_paper(grid: &mut [Vec<GridSpot>], threshold: usize) -> usize {
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == GridSpot::Empty {
                continue;
            }
            let temp = count_surrounding_paper(grid, (i, j));
            if temp < threshold {
                count += 1;
                grid[i][j] = GridSpot::EmptyingPaper;
            }
        }
    }
    count
}

fn iterated_accessible_paper(grid: &mut [Vec<GridSpot>], threshold: usize) -> usize {
    let mut count = count_accessible_paper(grid, threshold);
    println!("{}", count);
    let mut delta = count;
    while delta != 0 {
        // Finish clearing out
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == GridSpot::EmptyingPaper {
                    grid[i][j] = GridSpot::Empty;
                }
            }
        }
        delta = count_accessible_paper(grid, threshold);
        count += delta;
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

    println!("{}", iterated_accessible_paper(&mut grid, 4));
}
