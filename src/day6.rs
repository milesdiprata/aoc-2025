use std::fs;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

trait Solvable {
    fn problems(&self) -> &[Problem];
}

#[derive(Debug)]
enum Problem {
    Add(Vec<usize>),
    Mul(Vec<usize>),
}

struct Homework {
    problems: Vec<Problem>,
}

struct Homework2 {
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

        let row_len = nums.len();
        let col_len = nums.first().map(Vec::len).unwrap_or_default();

        #[allow(clippy::needless_range_loop)]
        for col in 0..col_len {
            for row in 0..row_len {
                problems[col].push(nums[row][col]);
            }
        }

        Ok(Self { problems })
    }
}

impl FromStr for Homework2 {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let mut lines = input.lines().collect::<Vec<_>>();

        let mut problems = lines
            .pop()
            .ok_or_else(|| anyhow!("empty input"))?
            .split_ascii_whitespace()
            .map(Problem::from_str)
            .collect::<Result<Vec<_>>>()?;

        let grid = lines
            .into_iter()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let row_len = grid.len();
        let col_len = grid.first().map(Vec::len).unwrap_or_default();

        let mut idx = problems.len().saturating_sub(1);

        for col in (0..col_len).rev() {
            if (0..row_len).all(|row| grid[row][col].is_ascii_whitespace()) {
                idx = idx.saturating_sub(1);
                continue;
            }

            let mut num = 0usize;

            #[allow(clippy::needless_range_loop)]
            for row in 0..row_len {
                if let Some(digit) = grid[row][col].to_digit(10) {
                    num *= 10;
                    num += digit as usize;
                }
            }

            problems[idx].push(num);
        }

        Ok(Self { problems })
    }
}

impl Solvable for Homework {
    fn problems(&self) -> &[Problem] {
        self.problems.as_slice()
    }
}

impl Solvable for Homework2 {
    fn problems(&self) -> &[Problem] {
        self.problems.as_slice()
    }
}

impl Problem {
    fn push(&mut self, num: usize) {
        match self {
            Self::Add(nums) | Self::Mul(nums) => nums.push(num),
        }
    }
}

fn solve<S>(homework: &S) -> usize
where
    S: Solvable,
{
    let mut sum = 0;

    for problem in homework.problems() {
        sum += match problem {
            Problem::Add(nums) => nums.iter().sum::<usize>(),
            Problem::Mul(nums) => nums.iter().product::<usize>(),
        };
    }

    sum
}

fn main() -> Result<()> {
    let input = fs::read_to_string("in/day6.txt")?;

    let homework = Homework::from_str(input.as_str())?;
    let homework2 = Homework2::from_str(input.as_str())?;

    let part1 = self::solve(&homework);
    let part2 = self::solve(&homework2);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    assert_eq!(part1, 4_405_895_212_738);
    assert_eq!(part2, 7_450_962_489_289);

    Ok(())
}
