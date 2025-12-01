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

fn crack_password(rotations: &[Rotation]) -> usize {
    let mut zero_count = 0usize;
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
            zero_count += 1;
        }
    }

    zero_count
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

    let password = self::crack_password(&rotations);

    println!("Part 1: {password}");

    Ok(())
}
