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
    let mut operands = HashMap::<usize, Vec<i64>>::new();
    let mut operations = Vec::new();
    for line in util::get_lines().map_while(Result::ok) {
        let tokens = line.split_ascii_whitespace();
        for (i, t) in tokens.enumerate() {
            match t.parse::<Operator>() {
                Ok(op) => operations.push(op),
                Err(_) => {
                    let n = t.parse::<i64>().unwrap();
                    operands.entry(i).or_default().push(n);
                }
            }
        }
    }
    let equations = operations
        .iter()
        .enumerate()
        .map(|(i, operator)| Equation::new(&operands[&i], *operator))
        .collect::<Vec<_>>();

    println!("{}", sum_all(&equations));
}
