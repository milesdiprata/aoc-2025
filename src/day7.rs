use std::collections::HashMap;
use std::collections::HashSet;
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
    fn len(&self) -> (usize, usize) {
        (
            self.grid.len(),
            self.grid.first().map(Vec::len).unwrap_or_default(),
        )
    }

    fn neighbors(&self, (row, col): (usize, usize)) -> Vec<(usize, usize)> {
        let (row_len, col_len) = self.len();

        let mut neighbors = vec![];

        match self.get((row, col)) {
            Some(Space::Start | Space::Empty) => {
                let row_new = row.checked_add(1);
                if row_new.is_some_and(|row_new| row_new < row_len) {
                    neighbors.push((row_new.unwrap(), col));
                }
            }
            Some(Space::Splitter) => {
                let row_new = row.checked_add(1);
                let col_left = col.checked_sub(1);
                let col_right = col.checked_add(1);

                if row_new.is_some_and(|row_new| row_new < row_len) {
                    if let Some(col_left) = col_left {
                        neighbors.push((row_new.unwrap(), col_left));
                    }

                    if col_right.is_some_and(|col_right| col_right < col_len) {
                        neighbors.push((row_new.unwrap(), col_right.unwrap()));
                    }
                }
            }
            None => (),
        }

        neighbors
    }

    fn get(&self, (row, col): (usize, usize)) -> Option<Space> {
        self.grid.get(row).and_then(|row| row.get(col)).copied()
    }
}

fn part1(manifold: &Manifold) -> usize {
    let mut splits = 0;
    let mut stack = vec![manifold.start];
    let mut visited = HashSet::from([manifold.start]);

    while let Some((row, col)) = stack.pop() {
        if manifold.get((row, col)) == Some(Space::Splitter) {
            splits += 1;
        }

        for (row_next, col_next) in manifold.neighbors((row, col)) {
            if visited.insert((row_next, col_next)) {
                stack.push((row_next, col_next));
            }
        }
    }

    splits
}

fn part2(manifold: &Manifold) -> usize {
    fn dfs(
        manifold: &Manifold,
        visited: &mut HashSet<(usize, usize)>,
        memo: &mut HashMap<(usize, usize), usize>,
        (row, col): (usize, usize),
    ) -> usize {
        if row == manifold.len().0.saturating_sub(1) {
            return 1;
        }

        if let Some(&timeline) = memo.get(&(row, col)) {
            return timeline;
        }

        let mut timeline = 0;
        for (row_next, col_next) in manifold.neighbors((row, col)) {
            if visited.insert((row_next, col_next)) {
                timeline += dfs(manifold, visited, memo, (row_next, col_next));
                visited.remove(&(row_next, col_next));
            }
        }

        memo.insert((row, col), timeline);

        timeline
    }

    let mut visited = HashSet::from([manifold.start]);
    let mut memo = HashMap::new();

    dfs(manifold, &mut visited, &mut memo, manifold.start)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("in/day7.txt")?;
    let manifold = Manifold::from_str(&input)?;

    let part1 = self::part1(&manifold);
    let part2 = self::part2(&manifold);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    assert_eq!(part1, 1_649);
    assert_eq!(part2, 16_937_871_060_075);

    Ok(())
}
