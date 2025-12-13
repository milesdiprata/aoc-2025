use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

#[derive(Debug)]
struct Device {
    name: String,
    outputs: Vec<String>,
}

impl FromStr for Device {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        let mut line = line.split(": ");

        let name = line
            .next()
            .ok_or_else(|| anyhow!("missing device name"))?
            .to_string();

        let outputs = line
            .next()
            .ok_or_else(|| anyhow!("missing device outputs"))?
            .split_ascii_whitespace()
            .map(str::to_string)
            .collect();

        Ok(Self { name, outputs })
    }
}

fn part1(devices: &[Device]) -> usize {
    fn dfs<'a>(
        node: &'a str,
        target: &'a str,
        adj_list: &HashMap<&'a str, &'a Vec<String>>,
        visited: &mut HashSet<&'a str>,
    ) -> usize {
        if visited.contains(&node) {
            return 0;
        }

        if node == target {
            return 1;
        }

        let mut count = 0;
        visited.insert(node);
        for neighbor in adj_list[node] {
            count += dfs(neighbor, target, adj_list, visited);
        }
        visited.remove(node);

        count
    }

    let mut adj_list = HashMap::with_capacity(devices.len());
    let mut visited = HashSet::with_capacity(devices.len());

    for device in devices {
        adj_list.insert(device.name.as_str(), &device.outputs);
    }

    dfs("you", "out", &adj_list, &mut visited)
}

fn main() -> Result<()> {
    let devices = BufReader::new(File::open("in/day11.txt")?)
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|line| Device::from_str(&line))
        .collect::<Result<Vec<_>>>()?;

    let part1 = self::part1(&devices);

    println!("Part 1: {part1}");

    assert_eq!(part1, 796);

    Ok(())
}
