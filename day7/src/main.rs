#![feature(test)]
extern crate test;

use std::collections::{HashMap};
use std::fs;

fn main() {
    let lines: HashMap<String, Line> = fs::read_to_string("data.txt").unwrap().lines().map(|line| {
        let line = parse(line);
        (line.name.clone(), line)
    }).collect();
}

struct Line {
    name: String,
    weight: u32,
    children_names: Vec<String>,
}

fn parse(input: &str) -> Line {
    // Assume length of at least 2. If > 2, 3rd token is garbage, 4+ are child names
    let parts: Vec<&str> = input.split_whitespace().collect();
    let weight_len = parts[1].len();
    Line {
        name: parts[0].to_string(),
        weight: parts[1][1..(weight_len - 1)].parse::<u32>().unwrap_or(0),
        children_names: parts.iter().skip(3).map(|p| p.replace(",", "")).collect(),
    }
}

struct RefNode {
    name: String,
}

struct Tree {
    name: String,
    weight: u32,
    children: Vec<Tree>,
}
impl Tree {
    fn find_mut(&mut self, name: &str) -> Option<&mut Tree> {
        let mut stack = vec![self];
        while !stack.is_empty() {
            if let Some(node) = stack.pop() {
                if node.name == name {
                    return Some(node)
                }
            }
        }
        None
    }
}

fn build(lines: &HashMap<String, Line>) -> Tree {
    let mut parentMap = HashMap::new();
    for (k, v) in lines.iter() {

    }
    Tree {
        name: "".to_string(),
        weight: 0,
        children: vec![]
    }
}



#[cfg(test)]
mod tests {
    // use test::{Bencher, black_box};

    /*
    #[test]
    fn aoc_test_part_1_simd() {
        assert_eq!(::manhattan_distance_simd(1), 0);
        assert_eq!(::manhattan_distance_simd(12), 3);
        assert_eq!(::manhattan_distance_simd(23), 2);
        assert_eq!(::manhattan_distance_simd(1024), 31);
    }

    #[bench]
    fn aoc_bench_part_1_simd(b: &mut Bencher) {
        let x = 28_567_190;
        b.iter(|| {
            black_box(::manhattan_distance_simd(x));
        })
    }
    */
}