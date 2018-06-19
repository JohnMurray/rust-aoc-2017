#![feature(test)]
extern crate test;
use std::fs;
mod input;

fn main() -> Result<(), Box<std::error::Error + 'static>> {
    let lines: Vec<Vec<u32>> = fs::read_to_string("data.txt").unwrap().lines().map(|line| {
        line.split_whitespace().map(|word| word.parse::<u32>().unwrap_or(0)).collect::<Vec<u32>>()
    }).collect();
    println!("{0}", checksum(&lines, diff_min_max));
    Ok(())
}

/// Returns a sum of the rows as collected by the `row_collector`.
/// 
/// # Arguments
/// * `input` - a 2-dimentional vector of u32's
/// * `row_collector` - a function to convert a row of u32's into a single value
fn checksum(input: &Vec<Vec<u32>>, row_collector: fn(&Vec<u32>) -> u32) -> u32 {
    input.iter().map(|row| row_collector(row)).sum()
}

fn div_evenly(row: &Vec<u32>) -> u32 {
    row.iter().skip(1).fold((row[0], 0), |tup, x| {
        
    });
    0
}

fn diff_min_max(row: &Vec<u32>) -> u32 {
    let min_max = row.iter().skip(1).fold((row[0], row[0]), |tup, x| {
        match *x {
            n if n < tup.0 => (n, tup.1),
            n if n > tup.1 => (tup.0, n),
            _              => tup,
        }
    });
    min_max.1 - min_max.0
}

#[cfg(test)]
mod test_part_1 {
    use test::Bencher;

    #[test]
    fn aoc_test_min_max() {
        let test_input1 = vec!(vec!(5, 1, 9, 5));
        let test_input2 = vec!(vec!(7, 5, 3));
        let test_input3 = vec!(vec!(2, 4, 6, 8));
        let test_input4 =vec!(
            vec!(5, 1, 9, 5),
            vec!(7, 5, 3),
            vec!(2, 4, 6, 8));

        assert_eq!(::checksum(&test_input1, ::diff_min_max), 8);
        assert_eq!(::checksum(&test_input2, ::diff_min_max), 4);
        assert_eq!(::checksum(&test_input3, ::diff_min_max), 6);
        assert_eq!(::checksum(&test_input4, ::diff_min_max), 18);
    }

    #[test]
    fn aoc_test_evenly_divisible() {
        let test_input1 = vec!(vec!(5, 9, 2, 8));
        let test_input2 = vec!(vec!(9, 4, 7, 3));

        assert_eq!(::checksum(&test_input1, ::div_evenly), 4);
        assert_eq!(::checksum(&test_input2, ::div_evenly), 3);
    }

    #[bench]
    fn bench_aoc_test_4(b: &mut Bencher) {
        let test_input =vec!(
            vec!(5, 1, 9, 5),
            vec!(7, 5, 3),
            vec!(2, 4, 6, 8));
        b.iter(|| {
            ::checksum(&test_input, ::diff_min_max);
        });
    }

    #[bench]
    fn bench_aoc_random_input(b: &mut Bencher) {
        let input_data = ::input::data();
        b.iter(|| {
            ::checksum(&input_data, ::diff_min_max);
        });
    }
}