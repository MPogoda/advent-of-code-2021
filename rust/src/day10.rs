use std::collections::VecDeque;

use itertools::Itertools;

type Input = Vec<Vec<u8>>;
pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

fn is_open(ch: u8) -> bool {
    return ch == b'(' || ch == b'[' || ch == b'{' || ch == b'<';
}
fn get_matching(ch: u8) -> u8 {
    match ch {
        b')' => b'(',
        b']' => b'[',
        b'}' => b'{',
        b'>' => b'<',
        _ => panic!("ohno"),
    }
}
fn get_syntax_error_score(ch: u8) -> usize {
    match ch {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => panic!("ohno"),
    }
}
fn get_total_syntax_error_score(line: Vec<u8>) -> Option<usize> {
    let mut stack = VecDeque::new();
    for ch in line {
        if is_open(ch) {
            stack.push_back(ch);
        } else if stack.pop_back() != Some(get_matching(ch)) {
            return Some(get_syntax_error_score(ch));
        }
    }
    None
}

pub fn part1(lines: Input) -> usize {
    lines
        .into_iter()
        .filter_map(get_total_syntax_error_score)
        .sum()
}

fn get_completion_score(ch: u8) -> usize {
    match ch {
        b'(' => 1,
        b'[' => 2,
        b'{' => 3,
        b'<' => 4,
        _ => panic!("ohno"),
    }
}
fn get_total_completion_score(line: Vec<u8>) -> Option<usize> {
    let mut stack = VecDeque::new();
    for ch in line {
        if is_open(ch) {
            stack.push_back(ch);
        } else if stack.pop_back() != Some(get_matching(ch)) {
            return None;
        }
    }
    Some(
        stack
            .into_iter()
            .map(get_completion_score)
            .rev()
            .fold(0, |acc, v| acc * 5 + v),
    )
}

pub fn part2(lines: Input) -> usize {
    let mut scores = lines
        .into_iter()
        .filter_map(get_total_completion_score)
        .collect_vec();
    let middle = scores.len() / 2;
    let (_, ans, _) = scores.select_nth_unstable(middle);
    *ans
}
