use regex::Regex;

type Input = Vec<Line>;

pub struct Line {
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize,
}

const SIZE: usize = 1000;

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
        Regex::new(r"^(?P<x1>\d{1, 3}),(?P<y1>\d{1, 3}) -> (?P<x2>\d{1, 3}),(?P<y2>\d{1, 3})$")
            .unwrap();
}

pub fn input_generator(input: &str) -> Input {
    input.lines().map(Line::parse).collect()
}

pub fn part1(lines: Input) -> usize {
    let mut field = [[0; SIZE]; SIZE];
    for line in lines {
        if line.x1 == line.x2 {
            let y_min = line.y1.min(line.y2);
            let y_max = line.y1.max(line.y2);
            for y in y_min..=y_max {
                field[y][line.x1] += 1;
            }
        } else if line.y1 == line.y2 {
            let x_min = line.x1.min(line.x2);
            let x_max = line.x1.max(line.x2);
            for x in x_min..=x_max {
                field[line.y1][x] += 1;
            }
        }
    }

    field.iter().flatten().filter(|&&v| v > 1).count()
}

pub fn part2(lines: Input) -> usize {
    let mut field = [[0; SIZE]; SIZE];
    for line in lines {
        if line.x1 == line.x2 {
            let y_min = line.y1.min(line.y2);
            let y_max = line.y1.max(line.y2);
            for y in y_min..=y_max {
                field[y][line.x1] += 1;
            }
        } else if line.y1 == line.y2 {
            let x_min = line.x1.min(line.x2);
            let x_max = line.x1.max(line.x2);
            for x in x_min..=x_max {
                field[line.y1][x] += 1;
            }
        } else if line.x1 < line.x2 && line.y1 < line.y2 && line.y2 - line.y1 == line.x2 - line.x1 {
            for i in 0..=(line.x2 - line.x1) {
                let x = line.x1 + i;
                let y = line.y1 + i;
                field[y][x] += 1;
            }
        } else if line.x1 > line.x2 && line.y1 < line.y2 && line.y2 - line.y1 == line.x1 - line.x2 {
            for i in 0..=(line.x1 - line.x2) {
                let x = line.x1 - i;
                let y = line.y1 + i;
                field[y][x] += 1;
            }
        } else if line.x1 > line.x2 && line.y1 > line.y2 && line.y1 - line.y2 == line.x1 - line.x2 {
            for i in 0..=(line.x1 - line.x2) {
                let x = line.x1 - i;
                let y = line.y1 - i;
                field[y][x] += 1;
            }
        } else if line.x1 < line.x2 && line.y1 > line.y2 && line.y1 - line.y2 == line.x2 - line.x1 {
            for i in 0..=(line.x2 - line.x1) {
                let x = line.x1 + i;
                let y = line.y1 - i;
                field[y][x] += 1;
            }
        }
    }

    field.iter().flatten().filter(|&&v| v > 1).count()
}
