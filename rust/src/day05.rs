use std::collections::HashMap;

use regex::Regex;

type Input = Vec<Line>;

pub struct Line {
    x1: u16,
    x2: u16,
    y1: u16,
    y2: u16,
}

impl Line {
    fn parse(line: &str) -> Self {
        let captures = RE.captures(line).unwrap();
        Self {
            x1: captures.name("x1").unwrap().as_str().parse().unwrap(),
            x2: captures.name("x2").unwrap().as_str().parse().unwrap(),
            y1: captures.name("y1").unwrap().as_str().parse().unwrap(),
            y2: captures.name("y2").unwrap().as_str().parse().unwrap(),
        }
    }
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)$").unwrap();
}

pub fn input_generator(input: &str) -> Input {
    input.lines().map(Line::parse).collect()
}

pub fn part1(lines: Input) -> usize {
    let mut field = HashMap::new();
    for line in lines {
        if line.x1 == line.x2 {
            let y_min = line.y1.min(line.y2);
            let y_max = line.y1.max(line.y2);
            for y in y_min..=y_max {
                field.insert((line.x1, y), field.get(&(line.x1, y)).unwrap_or(&0) + 1);
            }
        }
        if line.y1 == line.y2 {
            let x_min = line.x1.min(line.x2);
            let x_max = line.x1.max(line.x2);
            for x in x_min..=x_max {
                field.insert((x, line.y1), field.get(&(x, line.y1)).unwrap_or(&0) + 1);
            }
        }
    }

    field.values().filter(|&v| v > &1).count()
}

pub fn part2(lines: Input) -> usize {
    let mut field = HashMap::new();
    for line in lines {
        if line.x1 == line.x2 {
            let y_min = line.y1.min(line.y2);
            let y_max = line.y1.max(line.y2);
            for y in y_min..=y_max {
                field.insert((line.x1, y), field.get(&(line.x1, y)).unwrap_or(&0) + 1);
            }
        }
        if line.y1 == line.y2 {
            let x_min = line.x1.min(line.x2);
            let x_max = line.x1.max(line.x2);
            for x in x_min..=x_max {
                field.insert((x, line.y1), field.get(&(x, line.y1)).unwrap_or(&0) + 1);
            }
        }
        if line.x1 < line.x2 && line.y1 < line.y2 && line.y2 - line.y1 == line.x2 - line.x1 {
            for i in 0..=(line.x2 - line.x1) {
                let x = line.x1 + i;
                let y = line.y1 + i;
                field.insert((x, y), field.get(&(x, y)).unwrap_or(&0) + 1);
            }
        }
        if line.x1 > line.x2 && line.y1 < line.y2 && line.y2 - line.y1 == line.x1 - line.x2 {
            for i in 0..=(line.x1 - line.x2) {
                let x = line.x1 - i;
                let y = line.y1 + i;
                field.insert((x, y), field.get(&(x, y)).unwrap_or(&0) + 1);
            }
        }
        if line.x1 > line.x2 && line.y1 > line.y2 && line.y1 - line.y2 == line.x1 - line.x2 {
            for i in 0..=(line.x1 - line.x2) {
                let x = line.x1 - i;
                let y = line.y1 - i;
                field.insert((x, y), field.get(&(x, y)).unwrap_or(&0) + 1);
            }
        }
        if line.x1 < line.x2 && line.y1 > line.y2 && line.y1 - line.y2 == line.x2 - line.x1 {
            for i in 0..=(line.x2 - line.x1) {
                let x = line.x1 + i;
                let y = line.y1 - i;
                field.insert((x, y), field.get(&(x, y)).unwrap_or(&0) + 1);
            }
        }
    }

    field.values().filter(|&v| v > &1).count()
}
