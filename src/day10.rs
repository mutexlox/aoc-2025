use aoc_2025::util;
use bitvec::prelude as bitvec;
use good_lp::{
    Expression, ProblemVariables, Solution, SolverModel, Variable, constraint, default_solver,
    variable,
};
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Problem {
    goal: bitvec::BitVec,
    buttons: Vec<bitvec::BitVec>,
    joltages: Vec<i32>,
}

fn min_steps_for_goal(problem: &Problem) -> usize {
    fn min_steps_helper(
        problem: &Problem,
        steps: usize,
        possibles: &HashSet<bitvec::BitVec>,
    ) -> Option<usize> {
        if steps > problem.buttons.len() {
            return None;
        }
        let mut new_possibles = HashSet::new();
        for p in possibles.iter() {
            if p.not_any() {
                return Some(steps);
            }
            for b in problem.buttons.iter() {
                new_possibles.insert(p.clone() ^ b.clone());
            }
        }
        min_steps_helper(problem, steps + 1, &new_possibles)
    }

    let mut possibles = HashSet::new();
    possibles.insert(problem.goal.clone());
    min_steps_helper(problem, 0, &possibles).unwrap()
}

fn sum_required_steps(problems: &[Problem]) -> usize {
    problems.iter().map(min_steps_for_goal).sum()
}

fn min_presses_for_joltages(problem: &Problem) -> i64 {
    let vars = vec![variable().integer().min(0); problem.buttons.len()];
    let mut problem_var = ProblemVariables::new();
    let v: Vec<Variable> = problem_var.add_all(vars);

    let objective: Expression = v.iter().sum();
    let mut model = problem_var.minimise(objective).using(default_solver);
    model.set_parameter("loglevel", "0");

    for i in 0..problem.joltages.len() {
        let vi: Expression = v
            .iter()
            .zip(problem.buttons.iter())
            .map(|(v, b)| *v * (b[i] as i32))
            .sum();
        model = model.with(constraint!(vi == problem.joltages[i]));
    }

    let solution = model.solve().unwrap();

    solution.eval(v.iter().sum::<Expression>()) as i64
}

fn sum_required_presses(problems: &[Problem]) -> i64 {
    problems.iter().map(min_presses_for_joltages).sum()
}

fn main() {
    let overall_re =
        Regex::new(r"^\[(?<goal>[\.#]+)\] (?<buttons>(?:\([\d,]+\) ?)+) \{(?<joltages>[\d,]+)}$")
            .unwrap();
    let button_re = Regex::new(r"\((?<flips>[\d,]+)\)").unwrap();

    let mut problems = Vec::new();

    for line in util::get_lines().map_while(Result::ok) {
        let caps = overall_re.captures(&line).unwrap();
        let goal = &caps["goal"];
        let buttons = &caps["buttons"];
        let joltages = &caps["joltages"];

        let mut goal_vec = bitvec::BitVec::new();
        for c in goal.chars() {
            goal_vec.push(match c {
                '.' => false,
                '#' => true,
                _ => panic!("bad char {} in goal", c),
            });
        }

        let button_caps = button_re.captures_iter(buttons);
        let mut buttons_vecvec = Vec::new();
        for cap in button_caps {
            let mut buttons_vec = bitvec::BitVec::new();
            buttons_vec.resize(goal_vec.len(), false);
            for c in cap["flips"].split(',').map(|s| s.parse::<usize>().unwrap()) {
                buttons_vec.set(c, true);
            }
            buttons_vecvec.push(buttons_vec);
        }

        let joltages = joltages
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        problems.push(Problem {
            goal: goal_vec,
            buttons: buttons_vecvec,
            joltages,
        });
    }
    println!("{}", sum_required_steps(&problems));
    println!("{}", sum_required_presses(&problems));
}
