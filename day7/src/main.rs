#![feature(test)]
extern crate test;

use std::cell::RefCell;
use std::collections::{HashSet, HashMap};
use std::fs;
use std::rc::Rc;

fn main() {
    let lines: Vec<Line> = fs::read_to_string("data.txt").unwrap().lines().map(|line| {
        parse(line)
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

struct Tree {
    weight: u32,
    name: String,
    children: Vec<Rc<Tree>>,
}

fn build(lines: Vec<Line>) -> Tree {
    let mut all : HashSet<&str> = HashSet::new();
    let mut children: HashSet<&str> = HashSet::new();
    let mut tree_map : HashMap<&str, (Tree, &Line)> = HashMap::new();
    for line in &lines {
        let tree = Tree {
            weight: line.weight,
            name: line.name.clone(),
            children: vec![],
        };
        all.insert(&line.name);
        line.children_names.iter().map(|n| children.insert(n));
        tree_map.insert(&line.name, (tree, line));
    }
    let roots: Vec<&&str> = all.difference(&children).collect();
    if roots.len() != 1 {
        panic!("bad stuff");
    }
    let mut stack: Vec<(String, &Tree)> = Vec::new();
    let mut root = tree_map.get(roots[0]).unwrap().0;
    tree_map.get(roots[0]).unwrap().1.children_names.iter().map(|c| {
        stack.push((c.clone(), &root));
    });
    loop {
    }
    Tree{weight: 0, name: "".to_string(), children: vec![]}
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