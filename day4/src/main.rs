use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut valid_count: usize = 0;
    let mut valid_anagram_count: usize = 0;
    for line in contents.lines() {
        let tokens = tokenize(line);
        if is_passphrase_valid(&tokens) {
            valid_count += 1;
        }
        if is_passphrase_anagram_valid(&tokens) {
            valid_anagram_count += 1;
        }
    }
    println!("Valid tokens: {}", valid_count);
    println!("Valid anagram tokens: {}", valid_anagram_count);
}

fn is_passphrase_valid(words: &Vec<&str>) -> bool {
    let mut set = HashSet::new();
    for word in words {
        if set.contains(word) {
            return false;
        }
        set.insert(word);
    }
    true
}

fn is_passphrase_anagram_valid(words: &Vec<&str>) -> bool {
    use std::iter::FromIterator;
    let mut set = HashSet::new();
    for word in words {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort_by(|a, b| a.cmp(b));
        let sorted_word = String::from_iter(chars);

        if set.contains(&sorted_word) {
            return false;
        }
        set.insert(sorted_word);
    }
    true
}

fn tokenize<'a>(line: &'a str) -> Vec<&'a str> {
    let mut tokens = Vec::new();
    let mut start: usize = 0;
    for (i, c) in line.chars().enumerate() {
        if c.is_whitespace() {
            if i > 0 && (i - 1) > start {
                tokens.push(&line[start..i]);
                start = i + 1;
            }
        }
    }
    if start < (line.len() - 1) {
        tokens.push(&line[start..])
    }
    tokens
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc_part_1() {
        let line1 = tokenize("aa bb cc dd ee");
        let line2 = tokenize("aa bb cc dd aa");
        let line3 = tokenize("aa bb cc dd aaa");

        assert_eq!(is_passphrase_valid(&line1), true);
        assert_eq!(is_passphrase_valid(&line2), false);
        assert_eq!(is_passphrase_valid(&line3), true);
    }

    #[test]
    fn aoc_part_2() {
        let line1 = tokenize("abcde fghij");
        let line2 = tokenize("abcde xyz ecdab");
        let line3 = tokenize("a ab abc abd abf abj");
        let line4 = tokenize("iiii oiii ooii oooi oooo");
        let line5 = tokenize("oiii ioii iioi iiio");

        assert_eq!(is_passphrase_anagram_valid(&line1), true);
        assert_eq!(is_passphrase_anagram_valid(&line2), false);
        assert_eq!(is_passphrase_anagram_valid(&line3), true);
        assert_eq!(is_passphrase_anagram_valid(&line4), true);
        assert_eq!(is_passphrase_anagram_valid(&line5), false);
    }
}
