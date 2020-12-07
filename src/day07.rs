use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fs;
use std::path::Path;
use std::str::FromStr;

type Graph = HashMap<String, Vec<(String, i32)>>;

fn parse(filename: &Path) -> Result<Graph, String> {
    let re = Regex::new(r"^(\d+)\s+(.*)\s+bags?\.?$").unwrap();

    Ok(fs::read_to_string(filename)
        .map_err(|err| format!("Failed to read data for day 06: {}", err))?
        .lines()
        .map(|line| {
            let [bag, contents]: [&str; 2] = line.split(" bags contain ").collect::<Vec<&str>>().try_into().unwrap();
            (
                bag.to_owned(),
                if contents != "no other bags." {
                    contents
                        .split(", ")
                        .map(|token| {
                            let fields = re.captures(token).unwrap();
                            (
                                /* inner bag */ fields.get(2).unwrap().as_str().to_owned(),
                                /* count */ i32::from_str(fields.get(1).unwrap().as_str()).unwrap(),
                            )
                        })
                        .collect::<Vec<(String, i32)>>()
                } else {
                    Vec::new()
                },
            )
        })
        .collect::<Graph>())
}

fn reverse(graph: &Graph) -> Graph {
    let mut reversed = Graph::with_capacity(graph.len());

    for (node, edges) in graph.iter() {
        for (next, count) in edges.iter() {
            reversed
                .entry(next.to_owned())
                .and_modify(|edges| edges.push((node.to_owned(), *count)))
                .or_insert(vec![(node.to_owned(), *count)]);
        }
    }

    for (node, _) in graph.iter() {
        reversed.entry(node.to_owned()).or_insert(Vec::new());
    }

    reversed
}

#[derive(Clone, Copy, Debug, Default)]
struct DfsResult {
    visited: usize,
    cumulative_weight: i32,
}

fn dfs(bag: &str, graph: &Graph) -> DfsResult {
    if !graph.contains_key(bag) {
        return DfsResult::default();
    }

    let mut cumulative_weights = 0;
    let mut visited = HashSet::with_capacity(graph.len());
    let mut stack: Vec<(&str, i32)> = graph
        .get(bag)
        .unwrap()
        .iter()
        .map(|(next, weight)| (next.as_str(), *weight))
        .collect();

    while !stack.is_empty() {
        let frame = stack.pop().unwrap();

        let edges = graph.get(frame.0).unwrap();
        stack.extend(edges.iter().map(|(next, weight)| (next.as_str(), frame.1 * weight)));
        visited.insert(frame.0);
        cumulative_weights = cumulative_weights + frame.1;
    }

    DfsResult {
        visited: visited.len(),
        cumulative_weight: cumulative_weights,
    }
}

pub fn part01(filename: &Path) -> Result<String, String> {
    let reversed = reverse(&parse(filename)?);

    let bag = "shiny gold";
    Ok(format!(
        "Bags that eventually contains {} bags: {}",
        bag,
        dfs(bag, &reversed).visited
    ))
}

pub fn part02(filename: &Path) -> Result<String, String> {
    let graph = parse(filename)?;

    let bag = "shiny gold";
    Ok(format!(
        "{} bags contains a total of other bags: {}",
        bag,
        dfs(bag, &graph).cumulative_weight
    ))
}
