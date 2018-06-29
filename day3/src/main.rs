#![feature(test)]
#![feature(stdsimd)]
extern crate test;

fn main() {
    println!("Distance for {0}: {1}", 1, manhattan_distance(1));
    println!("Distance for {0}: {1}", 12, manhattan_distance(12));
    println!("Distance for {0}: {1}", 23, manhattan_distance(23));
    println!("Distance for {0}: {1}", 1024, manhattan_distance(1024));

    let x = 368078;
    println!("Distance for {1}: {0}", manhattan_distance(x), x);
}

/// rung - the "ring" we are in
/// step - the number of additional elements held by the next ring (over the current)
/// bucket - the highest number supported by our bucketing so far
fn manhattan_distance(x: u32) -> u32 {
    if x == 1 {
        return 0;
    }
    // calculate the bucket size needed which includes counting
    // the step as well as the rung we're in. The final value for
    // step is the number of items in the current ring
    let mut rung: u32 = 0;
    let mut step: u32 = 0;
    let mut bucket: u32 = 1;
    while bucket < x {
        step += 8;
        bucket += step;
        rung += 1;
    }

    // the length of the side is 2x the ring
    // the ring, then, is the lenght to the center
    let side_length = rung * 2;

    // set x to start from 1, in the final ring
    let xx: u32 = x - (bucket - step);

    // calculate the distance from the center of the ring
    let distance: u32 = i64::abs((xx % side_length) as i64 - rung as i64) as u32;

    distance + rung
}

fn manhattan_distance_simd(x: u32) -> u32 {
    use std::simd::u32x2;

    if x == 1 {
        return 0;
    }

    let mut rung_step = u32x2::new(0, 0);
    let rung_step_inc = u32x2::new(1, 8);
    let mut bucket: u32 = 1;
    while bucket < x {
        rung_step += rung_step_inc;
        bucket += rung_step.extract(1);
    }

    let side_length = rung_step.extract(0) * 2;
    let xx: u32 = x - (bucket - rung_step.extract(1));
    let distance: u32 = i64::abs((xx % side_length) as i64 - rung_step.extract(0) as i64) as u32;

    distance + rung_step.extract(0)
}


#[cfg(test)]
mod tests {
    use test::{Bencher, black_box};

    #[test]
    fn aoc_test_part_1() {
        assert_eq!(::manhattan_distance(1), 0);
        assert_eq!(::manhattan_distance(12), 3);
        assert_eq!(::manhattan_distance(23), 2);
        assert_eq!(::manhattan_distance(1024), 31);
    }

    #[test]
    fn aoc_test_part_1_simd() {
        assert_eq!(::manhattan_distance_simd(1), 0);
        assert_eq!(::manhattan_distance_simd(12), 3);
        assert_eq!(::manhattan_distance_simd(23), 2);
        assert_eq!(::manhattan_distance_simd(1024), 31);
    }

    #[bench]
    fn aoc_bench_part_1(b: &mut Bencher) {
        let x = 28_567_190;
        b.iter(|| {
            black_box(::manhattan_distance(x));
        })
    }

    #[bench]
    fn aoc_bench_part_1_simd(b: &mut Bencher) {
        let x = 28_567_190;
        b.iter(|| {
            black_box(::manhattan_distance_simd(x));
        })
    }
}