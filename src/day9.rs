use std::fs;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use geo::Contains;
use geo::Coord;
use geo::LineString;
use geo::Polygon;
use geo::Rect;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Rectangle {
    points: [Point; 4],
}

struct Theatre {
    polygon: Polygon,
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

    #[allow(clippy::cast_precision_loss)]
    const fn to_geo_coord(&self) -> Coord<f64> {
        Coord {
            x: self.x as f64,
            y: self.y as f64,
        }
    }
}

impl Rectangle {
    fn x_min(&self) -> i64 {
        self.points.iter().map(|p| p.x).min().unwrap_or_default()
    }

    fn x_max(&self) -> i64 {
        self.points.iter().map(|p| p.x).max().unwrap_or_default()
    }

    fn y_min(&self) -> i64 {
        self.points.iter().map(|p| p.y).min().unwrap_or_default()
    }

    fn y_max(&self) -> i64 {
        self.points.iter().map(|p| p.y).max().unwrap_or_default()
    }

    fn area(&self) -> i64 {
        (self.x_max() - self.x_min() + 1) * (self.y_max() - self.y_min() + 1)
    }

    #[allow(clippy::cast_precision_loss)]
    fn to_geo_rect(&self) -> Rect<f64> {
        Rect::new(
            Coord {
                x: self.x_min() as f64 + 0.5,
                y: self.y_min() as f64 + 0.5,
            },
            Coord {
                x: self.x_max() as f64 - 0.5,
                y: self.y_max() as f64 - 0.5,
            },
        )
    }
}

impl Theatre {
    fn new(red_tiles: &[Point]) -> Self {
        let coords = red_tiles.iter().map(Point::to_geo_coord).collect();

        Self {
            polygon: Polygon::new(LineString::new(coords), vec![]),
        }
    }

    fn contains_rect(&self, rect: &Rectangle) -> bool {
        let rect = rect.to_geo_rect();
        self.polygon.contains(&rect)
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

fn part2(points: &[Point]) -> i64 {
    let mut area_max = 0;

    let theatre = Theatre::new(points);

    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            if let Some(rect) = points[i].try_rectangle(&points[j]) {
                let area = rect.area();

                if area > area_max && theatre.contains_rect(&rect) {
                    area_max = area;
                }
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
    let part2 = self::part2(&points);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    assert_eq!(part1, 4_749_838_800);
    assert_eq!(part2, 1_624_057_680);

    Ok(())
}
