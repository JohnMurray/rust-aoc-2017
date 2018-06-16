use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("data.txt")?;
    let mut contents = String::new();
    let mut numbers: Vec<i32> = vec![];
    file.read_to_string(&mut contents)?;
    for c in contents.chars() {
        c.to_digit(10).map(|d| numbers.push(d as i32));
    }
    println!("{0}", sum_captcha_2(numbers));
    Ok(())
}

fn sum_captcha(nums: Vec<i32>) -> i32 {
    let mut prev: i32 = -2;
    let mut sum: i32 = 0;
    for n in &nums {
        if *n == prev {
            sum += n;
        }
        prev = *n;
    }
    let first: Option<&i32> = nums.get(0);
    let default: i32 = -1;
    if &prev == first.unwrap_or(&default) {
        sum += prev;
    }
    return sum
}

fn sum_captcha_2(nums: Vec<i32>) -> i32 {
    let length = nums.len();
    let half = length / 2;
    let mut i: usize = 0;
    let mut sum: i32 = 0;
    for n in &nums {
        if *n == nums[(i + half) % length] {
            sum += n;
        }
        i += 1;
    }
    return sum;
}

#[cfg(test)]
mod test_part_1 {
    #[test]
    fn aoc_test_1() {
        assert_eq!(::sum_captcha(vec!(1, 1, 2, 2)), 3);
    }
    
    #[test]
    fn aoc_test_2() {
        assert_eq!(::sum_captcha(vec!(1, 1, 1, 1)), 4);
    }

    #[test]
    fn aoc_test_3() {
        assert_eq!(::sum_captcha(vec!(1, 2, 3, 4)), 0);
    }

    #[test]
    fn aoc_test_4() {
        assert_eq!(::sum_captcha(vec!(9, 1, 2, 1, 2, 1, 2, 9)), 9);
    }
}

#[cfg(test)]
mod test_part_2 {
    #[test]
    fn aoc_test_1() {
        assert_eq!(::sum_captcha_2(vec!(1, 2, 1, 2)), 6);
    }

    #[test]
    fn aoc_test_2() {
        assert_eq!(::sum_captcha_2(vec!(1, 2, 2, 1)), 0);
    }

    #[test]
    fn aoc_test_3() {
        assert_eq!(::sum_captcha_2(vec!(1, 2, 3, 4, 2, 5)), 4);
    }

    #[test]
    fn aoc_test_4() {
        assert_eq!(::sum_captcha_2(vec!(1, 2, 3, 1, 2, 3)), 12);
    }

    #[test]
    fn aoc_test_5() {
        assert_eq!(::sum_captcha_2(vec!(1, 2, 1, 3, 1, 4, 1, 5)), 4);
    }
}