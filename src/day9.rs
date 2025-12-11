use std::fs;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Rectangle {
    points: [Point; 4],
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(point: &str) -> Result<Self> {
        let mut point = point.split(',');

        let x = point
            .next()
            .ok_or_else(|| anyhow!("missing x-coordinate"))?
            .parse()?;
        let y = point
            .next()
            .ok_or_else(|| anyhow!("missing y-coordinate"))?
            .parse()?;

        Ok(Self { x, y })
    }
}

impl Point {
    const fn try_rectangle(&self, other: &Self) -> Option<Rectangle> {
        if self.x == other.x || self.y == other.y {
            return None;
        }

        let p1 = Self {
            x: self.x,
            y: self.y,
        };
        let p2 = Self {
            x: self.x,
            y: other.y,
        };
        let p3 = Self {
            x: other.x,
            y: other.y,
        };
        let p4 = Self {
            x: other.x,
            y: self.y,
        };

        Some(Rectangle {
            points: [p1, p2, p3, p4],
        })
    }
}

impl Rectangle {
    fn area(&self) -> i64 {
        let x_max = self
            .points
            .iter()
            .map(|point| point.x)
            .max()
            .unwrap_or_default();
        let x_min = self
            .points
            .iter()
            .map(|point| point.x)
            .min()
            .unwrap_or_default();
        let y_max = self
            .points
            .iter()
            .map(|point| point.y)
            .max()
            .unwrap_or_default();
        let y_min = self
            .points
            .iter()
            .map(|point| point.y)
            .min()
            .unwrap_or_default();

        (x_max - x_min + 1) * (y_max - y_min + 1)
    }
}

fn part1(points: &[Point]) -> i64 {
    let mut area_max = 0;

    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            if let Some(rect) = points[i].try_rectangle(&points[j]) {
                area_max = area_max.max(rect.area());
            }
        }
    }

    area_max
}

fn main() -> Result<()> {
    let input = fs::read_to_string("in/day9.txt")?;
    let points = input
        .lines()
        .map(Point::from_str)
        .collect::<Result<Vec<_>>>()?;

    let part1 = self::part1(&points);

    println!("Part 1: {part1}");

    assert_eq!(part1, 4_749_838_800);

    Ok(())
}
