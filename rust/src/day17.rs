use std::str::FromStr;

use regex::Regex;

pub struct Input {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^target area: x=(?P<x1>.+)\.\.(?P<x2>.+), y=(?P<y1>.+)\.\.(?P<y2>.+)$")
            .unwrap();
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = RE.captures(s).unwrap();
        Ok(Self {
            x1: captures.name("x1").unwrap().as_str().parse().unwrap(),
            x2: captures.name("x2").unwrap().as_str().parse().unwrap(),
            y1: captures.name("y1").unwrap().as_str().parse().unwrap(),
            y2: captures.name("y2").unwrap().as_str().parse().unwrap(),
        })
    }
}

impl Input {
    fn fly(&self, mut vx: i32, mut vy: i32) -> Option<i32> {
        let (mut x, mut y) = (0, 0);
        let mut max_y = 0;

        while x <= self.x2 && y >= self.y1 {
            x += vx;
            y += vy;

            max_y = max_y.max(y);

            vx = 0.max(vx - 1);
            vy -= 1;

            if x >= self.x1 && x <= self.x2 && y >= self.y1 && y <= self.y2 {
                return Some(max_y);
            }
        }
        None
    }
}

pub fn input_generator(input: &str) -> Input {
    input.trim_end().parse().unwrap()
}

pub fn part1(input: Input) -> i32 {
    iproduct!(input.y1..=(-input.y1), 0..=input.x2)
        .filter_map(|(vy, vx)| input.fly(vx, vy))
        .max()
        .unwrap()
}

pub fn part2(input: Input) -> usize {
    iproduct!(input.y1..=(-input.y1), 0..=input.x2)
        .filter_map(|(vy, vx)| input.fly(vx, vy))
        .count()
}
