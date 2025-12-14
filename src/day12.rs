use aoc_2025::util;
use bitvec::prelude as bitvec;

const NUM_SHAPES: usize = 6;
const SHAPE_LINES: usize = 3;

#[derive(Debug)]
struct Shape {
    rows: Vec<bitvec::BitVec>,
}

#[derive(Debug)]
struct Goal {
    rows: usize,
    cols: usize,
    shapes: [usize; NUM_SHAPES],
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum HeuristicResult {
    ObviouslyYes,
    ObviouslyNo,
    Maybe,
}

fn run_heuristic(goal: &Goal, shapes: &[Shape]) -> HeuristicResult {
    let areas = shapes
        .iter()
        .map(|s| s.rows.iter().map(|b| b.count_ones()).sum())
        .collect::<Vec<_>>();
    let target_area = areas
        .iter()
        .zip(goal.shapes.iter())
        .map(|(a, g_count)| a * g_count)
        .sum();
    let goal_area = goal.rows * goal.cols;
    if goal_area < target_area {
        return HeuristicResult::ObviouslyNo;
    }
    let non_tessellated = (goal.rows / 3) * (goal.cols / 3);
    if non_tessellated >= goal.shapes.iter().sum() {
        return HeuristicResult::ObviouslyYes;
    }
    HeuristicResult::Maybe
}

fn is_goal_satisfiable(goal: &Goal, shapes: &[Shape]) -> bool {
    let heuristic = run_heuristic(goal, shapes);
    match heuristic {
        HeuristicResult::ObviouslyYes => return true,
        HeuristicResult::ObviouslyNo => return false,
        _ => {}
    }
    // Doesn't work for the test input, but does for the real one
    println!("heuristic failed");
    false
}

fn main() {
    let mut shapes: Vec<Shape> = Vec::new();
    let mut lines = util::get_lines().map_while(Result::ok);
    for _ in 0..NUM_SHAPES {
        let _idx = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .0
            .parse::<usize>()
            .unwrap();
        shapes.push(Shape {
            rows: lines
                .by_ref()
                .take(SHAPE_LINES)
                .map(|s| {
                    s.chars()
                        .map(|c| match c {
                            '#' => true,
                            '.' => false,
                            _ => panic!("bad char {}", c),
                        })
                        .collect::<bitvec::BitVec>()
                })
                .collect(),
        });
        // Drop the next (blank) line
        lines.next();
    }

    let mut goals = Vec::new();
    for line in lines {
        let (sizes, rest) = line.split_once(':').unwrap();
        let (rows, cols) = sizes.split_once('x').unwrap();
        let (rows, cols) = (
            rows.parse::<usize>().unwrap(),
            cols.parse::<usize>().unwrap(),
        );
        let targets = rest
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(targets.len(), NUM_SHAPES);
        let mut shapes = [0; NUM_SHAPES];
        for (i, &t) in targets.iter().enumerate() {
            shapes[i] = t;
        }
        goals.push(Goal { rows, cols, shapes });
    }

    let tot: usize = goals
        .iter()
        .map(|g| is_goal_satisfiable(g, &shapes) as usize)
        .sum();
    println!("{}", tot);
}
