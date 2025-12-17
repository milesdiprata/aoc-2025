use std::cmp::Reverse;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Write;
use std::fs;
use std::hash::Hash;
use std::iter;
use std::str::FromStr;
use std::time::Instant;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

#[derive(Clone, PartialEq, Eq)]
struct Shape {
    coords: HashSet<(i32, i32)>,
}

#[derive(Debug)]
struct Present {
    _idx: usize,
    shape: Shape,
}

#[derive(Debug)]
struct Region {
    width: i32,
    length: i32,
    quantities: Vec<usize>,
}

impl Debug for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let row_min = self
            .coords
            .iter()
            .map(|&(row, _)| row)
            .min()
            .unwrap_or_default();
        let row_max = self
            .coords
            .iter()
            .map(|&(row, _)| row)
            .max()
            .unwrap_or_default();
        let col_min = self
            .coords
            .iter()
            .map(|&(_, col)| col)
            .min()
            .unwrap_or_default();
        let col_max = self
            .coords
            .iter()
            .map(|&(_, col)| col)
            .max()
            .unwrap_or_default();

        for row in row_min..=row_max {
            if row > row_min {
                f.write_char('\n')?;
            }

            for col in col_min..=col_max {
                if self.coords.contains(&(row, col)) {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
        }

        Ok(())
    }
}

impl FromStr for Present {
    type Err = Error;

    fn from_str(present: &str) -> Result<Self> {
        let mut lines = present.lines();

        let idx = lines
            .next()
            .ok_or_else(|| anyhow!("missing present index"))?;
        let idx = idx[0..idx.len() - 1].parse()?;

        let mut shape = Shape {
            coords: HashSet::with_capacity(9),
        };

        for (row, line) in lines.enumerate() {
            for (col, space) in line.chars().enumerate() {
                if space == '#' {
                    shape
                        .coords
                        .insert((i32::try_from(row)?, i32::try_from(col)?));
                }
            }
        }

        Ok(Self { _idx: idx, shape })
    }
}

impl FromStr for Region {
    type Err = Error;

    fn from_str(region: &str) -> Result<Self> {
        let mut split = region.split(": ");

        let mut dimensions = split
            .next()
            .ok_or_else(|| anyhow!("missing region dimensions"))?
            .split('x');
        let width = dimensions
            .next()
            .ok_or_else(|| anyhow!("missing region width"))?
            .parse()?;
        let length = dimensions
            .next()
            .ok_or_else(|| anyhow!("missing region length"))?
            .parse()?;

        let quantities = split
            .next()
            .ok_or_else(|| anyhow!("missing region present quantities"))?
            .split_ascii_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            width,
            length,
            quantities,
        })
    }
}

impl Hash for Shape {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        #[allow(clippy::collection_is_never_read)]
        let mut coords = self.coords.iter().collect::<Vec<_>>();
        coords.sort_unstable();
        coords.hash(state);
    }
}

impl Shape {
    fn to_normalized(&self) -> Self {
        let (row_anchor, col_anchor) = self.coords.iter().min().copied().unwrap_or_default();

        Self {
            coords: self
                .coords
                .iter()
                .map(|&(row, col)| (row - row_anchor, col - col_anchor))
                .collect(),
        }
    }

    fn to_rotated(&self) -> Self {
        Self {
            coords: self.coords.iter().map(|&(row, col)| (col, -row)).collect(),
        }
        .to_normalized()
    }

    fn to_flipped(&self) -> Self {
        Self {
            coords: self.coords.iter().map(|&(row, col)| (row, -col)).collect(),
        }
        .to_normalized()
    }

    fn orientations(&self) -> Vec<Self> {
        let mut orientations = Vec::with_capacity(8);
        let mut seen = HashSet::with_capacity(8);
        let mut current = self.to_normalized();

        for _ in 0..2 {
            for _ in 0..4 {
                if seen.insert(current.clone()) {
                    orientations.push(current.clone());
                }

                current = current.to_rotated();
            }

            current = current.to_flipped();
        }

        orientations
    }
}

impl Region {
    fn is_feasible(&self, presents: &[Present]) -> bool {
        let area = self
            .quantities
            .iter()
            .enumerate()
            .map(|(idx, &count)| presents[idx].shape.coords.len() * count)
            .sum::<usize>();

        if area > self.length as usize * self.width as usize {
            return false;
        }

        let mut shapes = self
            .quantities
            .iter()
            .enumerate()
            .flat_map(|(idx, &count)| iter::repeat_n((presents[idx].shape.clone(), idx), count))
            .collect::<Vec<_>>();

        // Places big shapes first
        shapes.sort_unstable_by_key(|(shape, group)| (Reverse(shape.coords.len()), *group));

        let mut all_placements = Vec::new();

        for (shape, group) in &shapes {
            let mut placements = Vec::new();

            for orientation in shape.orientations() {
                for row_target in 0..self.length {
                    for col_target in 0..self.width {
                        let placed = orientation
                            .coords
                            .iter()
                            .map(|(row, col)| (row + row_target, col + col_target))
                            .collect::<Vec<_>>();

                        let in_bounds = placed.iter().all(|&(row, col)| {
                            row >= 0 && col >= 0 && row < self.length && col < self.width
                        });

                        if in_bounds {
                            placements.push(placed);
                        }
                    }
                }
            }

            all_placements.push((*group, placements));
        }

        let mut grid = HashSet::new();
        Self::dfs(&all_placements, 0, 0, usize::MAX, &mut grid)
    }

    fn dfs(
        all_placements: &[(usize, Vec<Vec<(i32, i32)>>)],
        idx: usize,
        start: usize,
        prev: usize,
        grid: &mut HashSet<(i32, i32)>,
    ) -> bool {
        if idx == all_placements.len() {
            return true;
        }

        let (group, placements) = &all_placements[idx];
        let start = if *group == prev { start } else { 0 };

        for (i, placement) in placements.iter().enumerate().skip(start) {
            if placement.iter().all(|pos| !grid.contains(pos)) {
                grid.extend(placement.iter().copied());

                if Self::dfs(all_placements, idx + 1, i, *group, grid) {
                    return true;
                }

                for pos in placement {
                    grid.remove(pos);
                }
            }
        }

        false
    }
}

fn part1(presents: &[Present], regions: &[Region]) -> usize {
    regions
        .iter()
        .filter(|&region| region.is_feasible(presents))
        .count()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("in/day12.txt")?;
    let mut input = input.split("\n\n").collect::<Vec<_>>();

    let regions = input
        .pop()
        .ok_or_else(|| anyhow!("missing regions"))?
        .lines()
        .map(Region::from_str)
        .collect::<Result<Vec<_>>>()?;
    let presents = input
        .into_iter()
        .map(Present::from_str)
        .collect::<Result<Vec<_>>>()?;

    let start = Instant::now();
    let part1 = self::part1(&presents, &regions);

    println!(
        "Part 1: {part1} ({:?})",
        Instant::now().duration_since(start),
    );

    Ok(())
}
