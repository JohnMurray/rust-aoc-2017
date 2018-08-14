#![feature(test)]
extern crate test;
extern crate combine;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use combine::parser::char::{letter, space, string, digit};
use combine::parser::choice::optional;
use combine::stream::state::State;
use combine::*;

#[cfg(feature="std")]
use combine::stream::easy;
#[cfg(feature="std")]
use combine::stream::state::SourcePosition;

fn main() {
    let mut data_file = File::open("data.txt").unwrap();
    let mut text = String::new();
    data_file.read_to_string(&mut text).unwrap();

    let lines = parse(&mut text);
    let mut register_bank: HashMap<&str, i32> = HashMap::new();

    let mut highest_reg_value: i32 = 0;

    for line in &lines {
        let comp_reg_val = *register_bank.get(&*line.conditional.left_operand_register).unwrap_or(&0);
        let comp_to = line.conditional.right_operand_value;
        let comp_eval = match line.conditional.comparison_operator {
            ComparisonOperator::LessThan           => comp_reg_val <  comp_to,
            ComparisonOperator::LessThanEqualTo    => comp_reg_val <= comp_to,
            ComparisonOperator::EqualTo            => comp_reg_val == comp_to,
            ComparisonOperator::NotEqualTo         => comp_reg_val != comp_to,
            ComparisonOperator::GreaterThanEqualTo => comp_reg_val >= comp_to,
            ComparisonOperator::GreaterThan        => comp_reg_val >  comp_to,
        };
        if comp_eval {
            let reg_val = *register_bank.get(&*line.register_name).unwrap_or(&0);
            let new_val = match line.instruction {
                Instruction::Increment => reg_val + line.by,
                Instruction::Decrement => reg_val - line.by,
            };
            register_bank.insert(&line.register_name, new_val);
            if new_val > highest_reg_value {
                highest_reg_value = new_val;
            }
        }
    }

    println!("Largest value computed: {:?}", register_bank.values().max());
    println!("Largest temp value seen: {:?}", highest_reg_value);
}

fn parse(text: &mut String) -> Vec<Line>
{
    let res = lines().easy_parse(
        State::new(&text[..]));
    res.unwrap().0
}

fn whitespace<I>() -> impl Parser<Input = I>
    where
        I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    skip_many(space())
}

fn line<I>() -> impl Parser<Input = I, Output = Line>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>
{
    (parse_register_action(), parse_conditional()).map(|(action, cond)| Line {
        register_name: action.0,
        instruction: action.1,
        by: action.2,
        conditional: cond
    })
}

fn lines<I>() -> impl Parser<Input = I, Output = Vec<Line>>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>
{
    many(line().skip(whitespace()))
}

fn parse_register_action<I>() -> impl Parser<Input = I, Output = (String, Instruction, i32)>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>
{
    (
        many1(letter()),
        many1(space()),
        many1(letter()),
        many1(space()),
        optional(token('-')),
        many1(digit())
    ).map(|(reg, _, instruc, _, neg, digits): (String, String, String, String, Option<char>, String)| {
        let mut by = digits.parse::<i32>().unwrap();
        if neg.is_some() {
            by = -by;
        }
        let instruction = match &instruc[..] {
            "inc" => Instruction::Increment,
            _     => Instruction::Decrement,
        };
        (reg, instruction, by)
    })
}

fn operator<I>() -> impl Parser<Input = I, Output = ComparisonOperator>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>
{
    (
        token('<')
            .or(token('>'))
            .or(token('!'))
            .or(token('=')),
        optional(token('='))
    ).map(|(t1, t2): (char, Option<char>)| {
        match (t1, t2) {
            ('<', Some('=')) => ComparisonOperator::LessThanEqualTo,
            ('<', None)      => ComparisonOperator::LessThan,
            ('>', Some('=')) => ComparisonOperator::GreaterThanEqualTo,
            ('>', None)      => ComparisonOperator::GreaterThan,
            ('!', Some('=')) => ComparisonOperator::NotEqualTo,
            ('=', Some('=')) => ComparisonOperator::EqualTo,
            _ => panic!("Invalid comparison operator: {:?} {:?}", t1, t2),
        }
    })
}

fn parse_conditional<I>() -> impl Parser<Input = I, Output = Conditional>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>
{
    (
        many1(space()),
        string("if"),
        many1(space()),
        many1(letter()), // register
        many1(space()),
        operator(),
        many1(space()),
        optional(token('-')),
        many1(digit())
    ).map(|(_, _, _, reg, _, op, _, neg, digits): (String, &'static str, String, String, String, ComparisonOperator, String, Option<char>, String)| {
        let mut than = digits.parse::<i32>().unwrap();
        if neg.is_some() {
            than = -than;
        }
        Conditional {
            left_operand_register: reg,
            right_operand_value: than,
            comparison_operator: op,
        }
    })
}

#[derive(PartialEq, Debug)]
enum Instruction {
    Increment,
    Decrement,
}

#[derive(PartialEq, Debug)]
struct Line {
    register_name: String,
    instruction: Instruction,
    by: i32,
    conditional: Conditional,
}

#[derive(PartialEq, Debug)]
enum ComparisonOperator {
    LessThan,
    LessThanEqualTo,
    EqualTo,
    NotEqualTo,
    GreaterThanEqualTo,
    GreaterThan,
}

#[derive(PartialEq, Debug)]
struct Conditional {
    left_operand_register: String,
    right_operand_value: i32,
    comparison_operator: ComparisonOperator
}

#[cfg(test)]
mod tests {
    // use test::{Bencher, black_box};
    use ::*;

    #[test]
    fn parse_positive_inc() {
        let input = "a inc 5 if b <= 6";
        let res = line().parse(input);
        let line = Line {
            register_name: "a".to_string(),
            instruction: Instruction::Increment,
            by: 5,
            conditional: Conditional {
                left_operand_register: "b".to_string(),
                comparison_operator: ComparisonOperator::LessThanEqualTo,
                right_operand_value: 6,
            },
        };
        assert_eq!(res, Ok((line, "")));
    }

    #[test]
    fn parse_negative_dec() {
        let input = "abc dec -23 if dgef != -37";
        let res = line().parse(input);
        let line = Line {
            register_name: "abc".to_string(),
            instruction: Instruction::Decrement,
            by: -23,
            conditional: Conditional {
                left_operand_register: "dgef".to_string(),
                comparison_operator: ComparisonOperator::NotEqualTo,
                right_operand_value: -37,
            },
        };
        assert_eq!(res, Ok((line, "")));
    }

    /*
    #[bench]
    fn aoc_bench_part_1_simd(b: &mut Bencher) {
        let x = 28_567_190;
        b.iter(|| {
            black_box(::manhattan_distance_simd(x));
        })
    }
    */
}