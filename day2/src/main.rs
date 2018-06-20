#![feature(test)]
extern crate test;
use std::fs;
mod input;

fn main() -> Result<(), Box<std::error::Error + 'static>> {
    let lines: Vec<Vec<u32>> = fs::read_to_string("data.txt").unwrap().lines().map(|line| {
        line.split_whitespace().map(|word| word.parse::<u32>().unwrap_or(0)).collect::<Vec<u32>>()
    }).collect();

    println!("{0}", checksum(&lines, diff_min_max));
    println!("{0}", checksum(&lines, div_evenly));

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

/// Returns the division of the first two numbers found in the vector
/// that are evenly divisible.
/// 
/// # Arguments
/// * `row` - a vector of integers to serach for a divisible pair within
/// 
/// # Notes
/// This function _does_ make a copy of the provided row in order to sort
/// it before searching.
fn div_evenly(row: &Vec<u32>) -> u32 {
    let mut sorted_row = row.clone();
    sorted_row.sort();
    sorted_row.iter().rev().fold(0, |acc, x| {
        if acc > 0 {
            return acc;
        }
        for y in sorted_row.iter() {
            // stop processing the loop if the numbers are
            // greater than n/2
            if y > &(x / &2) {
                break;
            }
            if x % y == 0 {
                return x / y;
            }
        }
        acc
    })
}

/// Returns the difference of the largest and smallest numbers found within
/// the row.
/// 
/// # Arguments
/// * `row` - a vector of integers in which to find the min/max pair
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
mod tests {
    use test::Bencher;

    #[test]
    fn test_min_max() {
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
    fn test_evenly_divisible() {
        let test_input1 = vec!(vec!(5, 9, 2, 8));
        let test_input2 = vec!(vec!(9, 4, 7, 3));
        let test_input3 = vec!(vec!(3, 8, 6, 5));

        assert_eq!(::checksum(&test_input1, ::div_evenly), 4);
        assert_eq!(::checksum(&test_input2, ::div_evenly), 3);
        assert_eq!(::checksum(&test_input3, ::div_evenly), 2);
    }

    #[bench]
    fn bench_min_max(b: &mut Bencher) {
        let test_input =vec!(
            vec!(5, 1, 9, 5),
            vec!(7, 5, 3),
            vec!(2, 4, 6, 8));
        b.iter(|| {
            ::checksum(&test_input, ::diff_min_max);
        });
    }

    #[bench]
    fn bench_min_max_large_input(b: &mut Bencher) {
        let input_data = ::input::data();
        b.iter(|| {
            ::checksum(&input_data, ::diff_min_max);
        });
    }

    #[bench]
    fn bench_evenly_div(b: &mut Bencher) {
        let test_input = vec!(
            vec!(5, 9, 2, 8),
            vec!(9, 4, 7, 3),
            vec!(3, 8, 6, 5),
        );
        b.iter(|| {
            ::checksum(&test_input, ::div_evenly);
        });
    }

    #[bench]
    fn bench_evenly_div_large_input(b: &mut Bencher) {
        let input_data = ::input::data();
        b.iter(|| {
            ::checksum(&input_data, ::div_evenly);
        });
    }
}