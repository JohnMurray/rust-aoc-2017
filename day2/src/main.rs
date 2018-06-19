#![feature(test)]
extern crate test;

use std::fs;

mod input;

fn main() -> Result<(), Box<std::error::Error + 'static>> {
    // using the ?. operator created a scoping/lifetime issue with the data
    // from ead_to_string. Why?
    let raw_input = fs::read_to_string("data.txt").unwrap();
    let lines: Vec<Vec<u32>> = raw_input.lines().map(|line| {
        line.split_whitespace().map(|word| {
            word.parse::<u32>().unwrap_or(0)
        }).collect::<Vec<u32>>()
    }).collect();
    println!("{0}", checksum(&lines));
    Ok(())
}

fn checksum(input: &Vec<Vec<u32>>) -> u32 {
    input.iter().map(|row| {
        let min_max = row.iter()
                .skip(1)
                .fold((row[0], row[0]), min_max_tuple);
        min_max.1 - min_max.0
    }).sum()
}

fn min_max_tuple(tup: (u32, u32), x: &u32) -> (u32, u32) {
    match *x {
        n if n < tup.0 => (n, tup.1),
        n if n > tup.1 => (tup.0, n),
        _              => tup,
    }
}

#[cfg(test)]
mod test_part_1 {
    use test::Bencher;

    #[test]
    fn aoc_test_1() {
        let test_input = vec!(vec!(5, 1, 9, 5));
        assert_eq!(::checksum(&test_input), 8)
    }

    #[test]
    fn aoc_test_2() {
        let test_input = vec!(vec!(7, 5, 3));
        assert_eq!(::checksum(&test_input), 4)
    }

    #[test]
    fn aoc_test_3() {
        let test_input = vec!(vec!(2, 4, 6, 8));
        assert_eq!(::checksum(&test_input), 6)
    }

    #[test]
    fn aoc_test_4_multiple_rows() {
        let test_input =vec!(
            vec!(5, 1, 9, 5),
            vec!(7, 5, 3),
            vec!(2, 4, 6, 8));
        assert_eq!(::checksum(&test_input), 18)
    }

    #[bench]
    fn bench_aoc_test_4(b: &mut Bencher) {
        let test_input =vec!(
            vec!(5, 1, 9, 5),
            vec!(7, 5, 3),
            vec!(2, 4, 6, 8));
        b.iter(|| {
            ::checksum(&test_input);
        });
    }

    #[bench]
    fn bench_aoc_random_input(b: &mut Bencher) {
        let input_data = ::input::data();
        b.iter(|| {
            ::checksum(&input_data);
        });
    }
}