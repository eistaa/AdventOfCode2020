use regex::Regex;
use std::collections::hash_map::Entry;
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

    reversed
}

fn dfs(bag: &str, graph: &Graph) -> usize {
    if !graph.contains_key(bag) {
        return 0;
    }

    let mut visited = HashSet::with_capacity(graph.len());
    let mut stack: Vec<&str> = graph.get(bag).unwrap().iter().map(|(next, _)| next.as_str()).collect();

    while !stack.is_empty() {
        let frame = stack.pop().unwrap();

        if visited.contains(frame) {
            continue;
        } else if !graph.contains_key(frame) {
            visited.insert(frame);
            continue;
        } else {
            let edges = graph.get(frame).unwrap();
            stack.extend(edges.iter().map(|(next, _)| next.as_str()));
            visited.insert(frame);
        }
    }

    visited.len()
}

fn cumulative_weight_sum(bag: &str, graph: &Graph) -> i32 {
    let mut weight_sum = 0;

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
        weight_sum = weight_sum + frame.1;
    }

    weight_sum
}

pub fn part01(filename: &Path) -> Result<String, String> {
    let graph = parse(filename)?;
    let reversed = reverse(&graph);

    let my_bags = "shiny gold";
    Ok(format!(
        "Bags that eventually contains {} bags: {}",
        my_bags,
        dfs(my_bags, &reversed)
    ))
}

pub fn part02(filename: &Path) -> Result<String, String> {
    let graph = parse(filename)?;

    let my_bags = "shiny gold";
    Ok(format!(
        "{} bags contains a total of other bags: {}",
        my_bags,
        cumulative_weight_sum(my_bags, &graph)
    ))
}
