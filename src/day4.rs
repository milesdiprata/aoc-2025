use std::fmt::Display;
use std::fmt::Write;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use anyhow::Result;

struct Grid(Vec<Vec<char>>);

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for &c in row {
                f.write_char(c)?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl Grid {
    fn len(&self) -> (usize, usize) {
        (self.0.len(), self.0[0].len())
    }

    fn neighbors(&self, i: usize, j: usize) -> impl Iterator<Item = (usize, usize)> {
        const DELTAS: &[(isize, isize)] = &[
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];

        let (i_len, j_len) = self.len();

        DELTAS.iter().filter_map(move |&(i_delta, j_delta)| {
            let i_new = i.checked_add_signed(i_delta)?;
            let j_new = j.checked_add_signed(j_delta)?;
            (i_new < i_len && j_new < j_len).then_some((i_new, j_new))
        })
    }

    fn accessible(&self) -> Option<Vec<(usize, usize)>> {
        let mut accessible = vec![];

        let (i_len, j_len) = self.len();

        for i in 0..i_len {
            for j in 0..j_len {
                if self.0[i][j] == '@' {
                    let mut adj = 0;

                    for (i_new, j_new) in self.neighbors(i, j) {
                        if self.0[i_new][j_new] == '@' {
                            adj += 1;
                        }
                    }

                    if adj < 4 {
                        accessible.push((i, j));
                    }
                }
            }
        }

        (!accessible.is_empty()).then_some(accessible)
    }

    fn remove(&mut self, i: usize, j: usize) {
        self.0[i][j] = 'x';
    }
}

fn part1(grid: &Grid) -> usize {
    grid.accessible()
        .map(|accessible| accessible.len())
        .unwrap_or_default()
}

fn part2(grid: &mut Grid) -> usize {
    let mut removed = 0;

    while let Some(accessible) = grid.accessible() {
        for (i, j) in accessible {
            grid.remove(i, j);
            removed += 1;
        }
    }

    removed
}

fn main() -> Result<()> {
    let mut grid = Grid(
        BufReader::new(File::open("in/day4.txt")?)
            .lines()
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );

    let part1 = self::part1(&grid);
    let part2 = self::part2(&mut grid);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    assert_eq!(part1, 1_560);
    assert_eq!(part2, 9_609);

    Ok(())
}
