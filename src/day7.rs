use std::fmt::Debug;
use std::fmt::Write;
use std::fs;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Space {
    Start,
    Empty,
    Splitter,
    Beam,
}

struct Manifold {
    grid: Vec<Vec<Space>>,
    start: (usize, usize),
}

impl Debug for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => f.write_char('S'),
            Self::Empty => f.write_char('.'),
            Self::Splitter => f.write_char('^'),
            Self::Beam => f.write_char('|'),
        }
    }
}

impl Debug for Manifold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.grid.iter().enumerate() {
            if i > 0 {
                f.write_char('\n')?;
            }

            for space in row {
                write!(f, "{space:?}")?;
            }
        }

        Ok(())
    }
}

impl FromStr for Manifold {
    type Err = Error;

    fn from_str(grid: &str) -> Result<Self> {
        let grid = grid
            .lines()
            .map(|line| line.chars().map(Space::new).collect::<Result<Vec<_>>>())
            .collect::<Result<Vec<_>>>()?;

        let mut start = None;
        for (idx_row, row) in grid.iter().enumerate() {
            for (idx_col, &space) in row.iter().enumerate() {
                if space == Space::Start {
                    if start.is_some() {
                        return Err(anyhow!("multiple start locations"));
                    }

                    start = Some((idx_row, idx_col));
                }
            }
        }

        let start = start.ok_or_else(|| anyhow!("missing start location"))?;

        Ok(Self { grid, start })
    }
}

impl Space {
    fn new(space: char) -> Result<Self> {
        match space {
            'S' => Ok(Self::Start),
            '.' => Ok(Self::Empty),
            '^' => Ok(Self::Splitter),
            _ => Err(anyhow!(format!("invalid space: '{space}'"))),
        }
    }
}

impl Manifold {
    fn get(&self, (row, col): (usize, usize)) -> Option<Space> {
        self.grid.get(row).and_then(|row| row.get(col)).copied()
    }

    fn get_mut(&mut self, (row, col): (usize, usize)) -> Option<&mut Space> {
        self.grid.get_mut(row).and_then(|row| row.get_mut(col))
    }
}

fn part1(manifold: &mut Manifold) -> usize {
    let mut splits = 0;
    let mut beams = vec![manifold.start];

    while let Some((row, col)) = beams.pop() {
        match manifold.get((row, col)) {
            Some(Space::Start | Space::Empty) => beams.push((row.saturating_add(1), col)),
            Some(Space::Splitter) => {
                splits += 1;

                beams.push((row.saturating_add(1), col.saturating_sub(1)));
                beams.push((row.saturating_add(1), col.saturating_add(1)));
            }
            Some(Space::Beam) | None => continue,
        }

        if let Some(space) = manifold.get_mut((row, col)) {
            *space = Space::Beam;
        }
    }

    splits
}

fn main() -> Result<()> {
    let input = fs::read_to_string("in/day7.txt")?;
    let mut manifold = Manifold::from_str(&input)?;

    let part1 = self::part1(&mut manifold);

    println!("Part 1: {part1}");

    assert_eq!(part1, 1_649);

    Ok(())
}
