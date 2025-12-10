use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

#[derive(Debug)]
struct JunctionBox {
    coords: [i64; 3],
}

impl FromStr for JunctionBox {
    type Err = Error;

    fn from_str(coords: &str) -> Result<Self> {
        let mut coords = coords.split(',');

        let x = coords
            .next()
            .ok_or_else(|| anyhow!("missing x-coordinate"))?
            .parse()?;
        let y = coords
            .next()
            .ok_or_else(|| anyhow!("missing y-coordinate"))?
            .parse()?;
        let z = coords
            .next()
            .ok_or_else(|| anyhow!("missing z-coordinate"))?
            .parse()?;

        Ok(Self { coords: [x, y, z] })
    }
}

impl JunctionBox {
    fn dist(&self, other: &Self) -> i64 {
        self.coords
            .iter()
            .zip(&other.coords)
            .map(|(coord_i, coord_j)| coord_i - coord_j)
            .map(|delta| delta.pow(2))
            .sum()
    }
}

/// Union find
fn part1(boxes: &[JunctionBox]) -> usize {
    fn find(parents: &mut [usize], i: usize) -> usize {
        if parents[i] != i {
            parents[i] = find(parents, parents[i]);
        }

        parents[i]
    }

    let mut pairs = Vec::new();
    for i in 0..boxes.len() - 1 {
        for j in i + 1..boxes.len() {
            pairs.push((i, j, boxes[i].dist(&boxes[j])));
        }
    }

    pairs.select_nth_unstable_by_key(999, |&(_, _, dist)| dist);

    let shortest = &pairs[..1000];
    let mut parents = (0..boxes.len()).collect::<Vec<_>>();

    for &(i, j, _) in shortest {
        let root_i = find(&mut parents, i);
        let root_j = find(&mut parents, j);

        if root_i != root_j {
            parents[root_i] = root_j;
        }
    }

    let mut lens = HashMap::<usize, usize>::new();
    for i in 0..boxes.len() {
        *lens.entry(find(&mut parents, i)).or_default() += 1;
    }

    let mut lens = lens.into_values().collect::<Vec<_>>();
    lens.select_nth_unstable_by(2, |i, j| j.cmp(i));

    lens.into_iter().take(3).product()
}

/// Kruskal's MST
fn part2(boxes: &[JunctionBox]) -> i64 {
    fn find(parents: &mut [usize], i: usize) -> usize {
        if parents[i] != i {
            parents[i] = find(parents, parents[i]);
        }
        parents[i]
    }

    let mut pairs = Vec::new();
    for i in 0..boxes.len() - 1 {
        for j in i + 1..boxes.len() {
            pairs.push((i, j, boxes[i].dist(&boxes[j])));
        }
    }

    let mut pairs = pairs
        .into_iter()
        .map(|(i, j, d)| Reverse((d, i, j)))
        .collect::<BinaryHeap<_>>();

    let mut parents = (0..boxes.len()).collect::<Vec<_>>();
    let mut lens = vec![1; boxes.len()];
    let mut component_len_max = 1;

    while let Some(Reverse((_, i, j))) = pairs.pop() {
        let root_i = find(&mut parents, i);
        let root_j = find(&mut parents, j);

        if root_i != root_j {
            if lens[root_i] < lens[root_j] {
                parents[root_i] = root_j;
                lens[root_j] += lens[root_i];
                component_len_max = component_len_max.max(lens[root_j]);
            } else {
                parents[root_j] = root_i;
                lens[root_i] += lens[root_j];
                component_len_max = component_len_max.max(lens[root_i]);
            }
        }

        if component_len_max == boxes.len() {
            return boxes[i].coords[0] * boxes[j].coords[0];
        }
    }

    unreachable!()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("in/day8.txt")?;
    let boxes = input
        .lines()
        .map(JunctionBox::from_str)
        .collect::<Result<Vec<_>>>()?;

    let part1 = self::part1(&boxes);
    let part2 = self::part2(&boxes);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    assert_eq!(part1, 129_564);
    assert_eq!(part2, 42_047_840);

    Ok(())
}
