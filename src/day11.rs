use std::collections::HashMap;
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

fn dfs<'a>(
    node: &'a str,
    target: &'a str,
    adj_list: &HashMap<&'a str, &'a Vec<String>>,
    memo: &mut HashMap<(&'a str, &'a str), usize>,
) -> usize {
    if node == target {
        return 1;
    }

    if let Some(&result) = memo.get(&(node, target)) {
        return result;
    }

    let mut count = 0;

    if let Some(&neighbors) = adj_list.get(node) {
        for neighbor in neighbors {
            count += dfs(neighbor, target, adj_list, memo);
        }
    }

    memo.insert((node, target), count);

    count
}

fn part1(devices: &[Device]) -> usize {
    let mut adj_list = HashMap::with_capacity(devices.len());
    let mut memo = HashMap::new();

    for device in devices {
        adj_list.insert(device.name.as_str(), &device.outputs);
    }

    self::dfs("you", "out", &adj_list, &mut memo)
}

fn part2(devices: &[Device]) -> usize {
    let mut adj_list = HashMap::with_capacity(devices.len());
    let mut memo = HashMap::new();

    for device in devices {
        adj_list.insert(device.name.as_str(), &device.outputs);
    }

    let paths_fft_first = self::dfs("svr", "fft", &adj_list, &mut memo)
        * self::dfs("fft", "dac", &adj_list, &mut memo)
        * self::dfs("dac", "out", &adj_list, &mut memo);

    memo.clear();

    let paths_dac_first = self::dfs("svr", "dac", &adj_list, &mut memo)
        * self::dfs("dac", "fft", &adj_list, &mut memo)
        * self::dfs("fft", "out", &adj_list, &mut memo);

    paths_fft_first + paths_dac_first
}

fn main() -> Result<()> {
    let devices = BufReader::new(File::open("in/day11.txt")?)
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|line| Device::from_str(&line))
        .collect::<Result<Vec<_>>>()?;

    let part1 = self::part1(&devices);
    let part2 = self::part2(&devices);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    assert_eq!(part1, 796);
    assert_eq!(part2, 294_053_029_111_296);

    Ok(())
}
