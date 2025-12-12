use anyhow::{Result, anyhow};
use aoc_2025::util;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Operator {
    Add,
    Mul,
}

impl FromStr for Operator {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Mul),
            _ => Err(anyhow!("bad operation {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
struct Equation {
    operands: Vec<i64>,
    operator: Operator,
}

impl Equation {
    fn new(operands: &[i64], operator: Operator) -> Self {
        Self {
            operands: operands.to_vec(),
            operator,
        }
    }

    fn evaluate(&self) -> i64 {
        match self.operator {
            Operator::Add => self.operands.iter().sum(),
            Operator::Mul => self.operands.iter().product(),
        }
    }
}

fn sum_all(equations: &[Equation]) -> i64 {
    equations.iter().map(|eq| eq.evaluate()).sum()
}

fn main() {
    let mut operands_pt1 = HashMap::<usize, Vec<i64>>::new();
    let mut chars_pt2 = Vec::new();
    let mut operations = Vec::new();
    for line in util::get_lines().map_while(Result::ok) {
        chars_pt2.push(line.chars().collect::<Vec<_>>());
        let tokens = line.split_ascii_whitespace();
        for (i, t) in tokens.enumerate() {
            match t.parse::<Operator>() {
                Ok(op) => operations.push(op),
                Err(_) => {
                    let n = t.parse::<i64>().unwrap();
                    operands_pt1.entry(i).or_default().push(n);
                }
            }
        }
    }

    // Drop the operations
    chars_pt2.pop();

    let mut operands_pt2 = Vec::new();
    let mut cur_eq = Vec::new();
    for i in 0..chars_pt2[0].len() {
        let mut cur = 0;
        let mut done_equation = true;
        for v in chars_pt2.iter() {
            if v[i].is_ascii_digit() {
                done_equation = false;
                cur *= 10;
                cur += v[i].to_digit(10).unwrap() as i64;
            }
        }
        if done_equation {
            operands_pt2.push(cur_eq);
            cur_eq = Vec::new();
        } else {
            cur_eq.push(cur);
        }
    }
    if !cur_eq.is_empty() {
        operands_pt2.push(cur_eq);
    }

    let equations_pt1 = operations
        .iter()
        .enumerate()
        .map(|(i, operator)| Equation::new(&operands_pt1[&i], *operator))
        .collect::<Vec<_>>();

    let equations_pt2 = operations
        .iter()
        .enumerate()
        .map(|(i, operator)| Equation::new(&operands_pt2[i], *operator))
        .collect::<Vec<_>>();

    println!("{}", sum_all(&equations_pt1));
    println!("{}", sum_all(&equations_pt2));
}
