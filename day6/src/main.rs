#![feature(stdsimd)]
#![feature(test)]
#![feature(type_ascription)]
extern crate test;

use std::collections::HashMap;

fn main() {
    // let mut v: Vec<u8> = vec![0, 2, 7, 0];
    let mut v: Vec<u8> = vec![5, 1, 10, 0, 1, 7, 13, 14, 3, 12, 8, 10, 7, 12, 0, 6];
    let balance_info = balance(&mut v);
    println!("Balanced Banks: {:?}\nIterations: {}\nCycle Distance: {}",
             &v,
             balance_info.iterations,
             balance_info.cycle_distance);
}

struct BalanceInfo {
    iterations: u64,
    cycle_distance: u64,
}

fn balance(banks: &mut Vec<u8>) -> BalanceInfo {
    if banks.len() == 0 {
        return BalanceInfo { iterations: 0, cycle_distance: 0 };
    }

    let mut iterations: u64 = 0;
    let mut distributions: HashMap<Vec<u8>, u64> = HashMap::new();

    // infinitely look until a duplicate memory layout is detected
    loop {
        iterations += 1;

        // find the index of the maximum item
        let max_index = banks.iter().enumerate().fold((0, &0): (usize, &u8), |acc, val| {
            if val.1 > acc.1 { (val.0, val.1) } else { acc }
        }).0;
        let blocks = banks[max_index];
        banks[max_index] = 0;

        // calculate the value to add to (all banks)
        let all_sum = (blocks as usize / banks.len()) as u8;

        // calculate the left-overs
        let remainder = (blocks as usize % banks.len()) as u8;

        // increment each bank with appropriate block count
        let mut i: usize = 0;
        while i < banks.len() {
            // calculate distance from max index
            let offset: usize =
                if max_index > i { banks.len() - max_index + i }
                else if max_index == i { 0 }
                else { i - max_index };
            let carry =
                if offset > 0 && offset <= remainder as usize { 1 }
                else { 0 };
            banks[i] += all_sum + carry;
            i += 1
        }

        if distributions.contains_key(banks) {
            break;
        }
        distributions.insert(banks.clone(), iterations);
    }

    BalanceInfo {
        iterations: iterations,
        cycle_distance: iterations - distributions.get(banks).unwrap(),
    }
}



#[cfg(test)]
mod tests {
    use test::{Bencher, black_box};

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