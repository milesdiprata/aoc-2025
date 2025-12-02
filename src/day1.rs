use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

#[derive(Debug)]
enum Rotation {
    Left(usize),
    Right(usize),
}

impl FromStr for Rotation {
    type Err = Error;

    fn from_str(str: &str) -> Result<Self> {
        let dir = str
            .chars()
            .next()
            .ok_or_else(|| anyhow!("missing rotation direction"))?;

        let dist = str
            .get(1..)
            .ok_or_else(|| anyhow!("missing rotation distance"))?
            .parse::<usize>()?;

        match dir {
            'L' => Ok(Self::Left(dist)),
            'R' => Ok(Self::Right(dist)),
            _ => Err(anyhow!("invalid rotation direction")),
        }
    }
}

fn part1(rotations: &[Rotation]) -> usize {
    let mut count = 0usize;
    let mut dial = 50usize;

    for rotation in rotations {
        match rotation {
            Rotation::Left(dist) => {
                let dist = dist % 100;
                if dist > dial {
                    let delta = dist - dial;
                    dial = 100 - delta;
                } else {
                    dial -= dist;
                }
            }
            Rotation::Right(dist) => {
                let dist = dist % 100;
                dial += dist;
                dial %= 100;
            }
        }

        if dial == 0 {
            count += 1;
        }
    }

    count
}

#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
fn part2(rotations: &[Rotation]) -> usize {
    let mut count = 0usize;
    let mut dial = 50isize;

    for rotation in rotations {
        let old = dial;

        match *rotation {
            Rotation::Left(dist) => dial -= dist as isize,
            Rotation::Right(dist) => dial += dist as isize,
        }

        count += if dial > old {
            dial.div_euclid(100) - old.div_euclid(100)
        } else {
            (old - 1).div_euclid(100) - (dial - 1).div_euclid(100)
        } as usize;
    }

    count
}

fn main() -> Result<()> {
    let file = File::open("in/day1.txt")?;
    let reader = BufReader::new(file);

    let rotations = reader
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|line| line.parse::<Rotation>())
        .collect::<Result<Vec<_>>>()?;

    let password1 = self::part1(&rotations);
    let password2 = self::part2(&rotations);

    println!("Part 1: {password1}");
    println!("Part 2: {password2}");

    assert_eq!(password1, 1120);
    assert_eq!(password2, 6554);

    Ok(())
}
