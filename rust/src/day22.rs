use std::{collections::HashMap, str::FromStr};

use regex::Regex;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Cube {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    z1: i32,
    z2: i32,
}

pub struct Command {
    on: bool,
    cube: Cube,
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^o(n|f)f? x=(.+)\.\.(.+),y=(.+)\.\.(.+),z=(.+)\.\.(.+)$").unwrap();
}
impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = RE.captures(s).unwrap();
        let on = captures
            .get(1)
            .unwrap()
            .as_str()
            .as_bytes()
            .first()
            .unwrap()
            == &b'n';
        let x1 = captures.get(2).unwrap().as_str().parse().unwrap();
        let x2 = captures.get(3).unwrap().as_str().parse().unwrap();
        let y1 = captures.get(4).unwrap().as_str().parse().unwrap();
        let y2 = captures.get(5).unwrap().as_str().parse().unwrap();
        let z1 = captures.get(6).unwrap().as_str().parse().unwrap();
        let z2 = captures.get(7).unwrap().as_str().parse().unwrap();

        Ok(Self {
            on,
            cube: Cube {
                x1,
                x2,
                y1,
                y2,
                z1,
                z2,
            },
        })
    }
}

type Input = Vec<Command>;

pub fn input_generator(s: &str) -> Input {
    s.lines().map(|line| line.parse().unwrap()).collect()
}

impl Cube {
    fn intersect(&self, other: &Self) -> Option<Self> {
        let x1 = self.x1.max(other.x1);
        let y1 = self.y1.max(other.y1);
        let z1 = self.z1.max(other.z1);

        let x2 = self.x2.min(other.x2);
        let y2 = self.y2.min(other.y2);
        let z2 = self.z2.min(other.z2);

        if x1 <= x2 && y1 <= y2 && z1 <= z2 {
            Some(Self {
                x1,
                y1,
                z1,
                x2,
                y2,
                z2,
            })
        } else {
            None
        }
    }

    fn volume(&self) -> i64 {
        let x = (self.x2 - self.x1 + 1) as i64;
        let y = (self.y2 - self.y1 + 1) as i64;
        let z = (self.z2 - self.z1 + 1) as i64;
        x * y * z
    }

    fn clamp(self, val: i32) -> Option<Self> {
        let x1 = self.x1.max(-val);
        let y1 = self.y1.max(-val);
        let z1 = self.z1.max(-val);

        let x2 = self.x2.min(val);
        let y2 = self.y2.min(val);
        let z2 = self.z2.min(val);
        if x1 <= x2 && y1 <= y2 && z1 <= z2 {
            Some(Self {
                x1,
                x2,
                y1,
                y2,
                z1,
                z2,
            })
        } else {
            None
        }
    }
}

impl Command {
    fn clamp_cube(self, val: i32) -> Option<Command> {
        if let Some(cube) = self.cube.clamp(val) {
            Some(Command { on: self.on, cube })
        } else {
            None
        }
    }
}

fn solve<T: Iterator<Item = Command>>(commands: T) -> u64 {
    let mut state: HashMap<Cube, i64> = HashMap::new();
    for Command { on, cube } in commands {
        for (old_cube, sign) in state.clone() {
            if let Some(icube) = cube.intersect(&old_cube) {
                *state.entry(icube).or_insert(0) -= sign;
            }
        }
        if on {
            *state.entry(cube).or_insert(0) += 1;
        }
    }

    state
        .into_iter()
        .fold(0, |acc, (cube, sign)| acc + sign * cube.volume()) as u64
}

pub fn part1(commands: Input) -> u64 {
    solve(
        commands
            .into_iter()
            .filter_map(|command| command.clamp_cube(50)),
    )
}

pub fn part2(commands: Input) -> u64 {
    solve(commands.into_iter())
}
