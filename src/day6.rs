use std::fs;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

#[derive(Debug)]
enum Problem {
    Add(Vec<usize>),
    Mul(Vec<usize>),
}

struct Homework {
    problems: Vec<Problem>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(op: &str) -> Result<Self> {
        match op.get(0..1) {
            Some("+") => Ok(Self::Add(Vec::new())),
            Some("*") => Ok(Self::Mul(Vec::new())),
            Some(_) => Err(anyhow!(format!("invalid operation: '{op}'"))),
            None => Err(anyhow!("missing operation")),
        }
    }
}

impl FromStr for Homework {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let mut lines = input.lines().collect::<Vec<_>>();

        let mut problems = lines
            .pop()
            .ok_or_else(|| anyhow!("empty input"))?
            .split_ascii_whitespace()
            .map(Problem::from_str)
            .collect::<Result<Vec<_>>>()?;

        let nums = lines
            .into_iter()
            .map(|line| line.split_ascii_whitespace())
            .map(|nums| nums.map(str::parse::<usize>))
            .map(Iterator::collect::<Result<Vec<_>, _>>)
            .collect::<Result<Vec<_>, _>>()?;

        for col in 0..nums[0].len() {
            for row in 0..nums.len() {
                match &mut problems[col] {
                    Problem::Add(problem) | Problem::Mul(problem) => problem.push(nums[row][col]),
                }
            }
        }

        Ok(Self { problems })
    }
}

fn part1(homework: &Homework) -> usize {
    let mut sum = 0;

    for problem in &homework.problems {
        sum += match problem {
            Problem::Add(nums) => nums.iter().sum::<usize>(),
            Problem::Mul(nums) => nums.iter().product::<usize>(),
        }
    }

    sum
}

fn main() -> Result<()> {
    let input = fs::read_to_string("in/day6.txt")?;
    let homework = Homework::from_str(input.as_str())?;

    let part1 = self::part1(&homework);

    println!("Part 1: {part1}");

    assert_eq!(part1, 4_405_895_212_738);

    Ok(())
}
