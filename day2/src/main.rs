use std::fs;


fn main() -> Result<(), Box<std::error::Error + 'static>> {
    // using the ?. operator created a scoping/lifetime issue with the data
    // from read_to_string. Why?
    let raw_input = fs::read_to_string("data.txt").unwrap();
    let lines: Vec<Vec<u32>> = raw_input.lines().map(|line| {
        line.split_whitespace().map(|word| {
            word.parse::<u32>().unwrap_or(0)
        }).collect::<Vec<u32>>()
    }).collect();
    println!("{0}", checksum(lines));
    Ok(())
}

fn checksum(input: Vec<Vec<u32>>) -> u32 {
    input.iter().map(|row| {
        let min_max = row.iter()
                .skip(1)
                .fold((row[0], row[0]), min_max_tuple);
        min_max.1 - min_max.0
    }).sum()
}

fn min_max_tuple(tup: (u32, u32), x: &u32) -> (u32, u32) {
    if *x < tup.0 {
        return (*x, tup.1);
    }
    else if *x > tup.1 {
        return (tup.0, *x);
    }
    return tup;
}



#[cfg(test)]
mod test_part_1 {
    #[test]
    fn aoc_test_1() {
        assert_eq!(::checksum(vec!(vec!(5, 1, 9, 5))), 8)
    }

    #[test]
    fn aoc_test_2() {
        assert_eq!(::checksum(vec!(vec!(7, 5, 3))), 4)
    }

    #[test]
    fn aoc_test_3() {
        assert_eq!(::checksum(vec!(vec!(2, 4, 6, 8))), 6)
    }

    #[test]
    fn aoc_test_4_multiple_rows() {
        assert_eq!(::checksum(vec!(
            vec!(5, 1, 9, 5),
            vec!(7, 5, 3),
            vec!(2, 4, 6, 8)
        )), 18)
    }
}