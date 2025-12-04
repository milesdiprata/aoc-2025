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
        const DIRS: &[(isize, isize)] = &[
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

        DIRS.iter().filter_map(move |&(i_delta, j_delta)| {
            let i_new = i.checked_add_signed(i_delta)?;
            let j_new = j.checked_add_signed(j_delta)?;
            (i_new < i_len && j_new < j_len).then_some((i_new, j_new))
        })
    }

    fn accessible(&self) -> usize {
        let mut accessible = 0;

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
                        accessible += 1;
                    }
                }
            }
        }

        accessible
    }
}

fn part1(grid: &Grid) -> usize {
    grid.accessible()
}

fn main() -> Result<()> {
    let grid = Grid(
        BufReader::new(File::open("in/day4.txt")?)
            .lines()
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );

    let part1 = self::part1(&grid);

    println!("Part 1: {part1}");

    assert_eq!(part1, 1_560);

    Ok(())
}
