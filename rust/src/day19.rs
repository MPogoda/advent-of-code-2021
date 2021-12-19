use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    iter::Peekable,
    str::{FromStr, Lines},
};

use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(.+),(.+),(.+)$").unwrap();
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Beacon {
    coords: [i32; 3],
}

impl FromStr for Beacon {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = RE.captures(s).unwrap();
        let x = captures.get(1).unwrap().as_str().parse().unwrap();
        let y = captures.get(2).unwrap().as_str().parse().unwrap();
        let z = captures.get(3).unwrap().as_str().parse().unwrap();

        Ok(Self { coords: [x, y, z] })
    }
}

impl Beacon {
    fn adjust(&self, rotate: &[usize; 3], signs: &[i32; 3]) -> Self {
        Self {
            coords: [
                signs[0] * self.coords[rotate[0]],
                signs[1] * self.coords[rotate[1]],
                signs[2] * self.coords[rotate[2]],
            ],
        }
    }

    fn diff(&self, other: &Self) -> Self {
        Self {
            coords: [
                self.coords[0] - other.coords[0],
                self.coords[1] - other.coords[1],
                self.coords[2] - other.coords[2],
            ],
        }
    }

    fn shift(&self, diff: &[i32; 3]) -> Self {
        Self {
            coords: [
                self.coords[0] + diff[0],
                self.coords[1] + diff[1],
                self.coords[2] + diff[2],
            ],
        }
    }
}

pub struct Scanner {
    id: usize,
    beacons: Vec<Beacon>,
}

impl Scanner {
    fn parse(id: usize, lines: &mut Peekable<Lines>) -> Self {
        lines.next();
        let mut beacons = Vec::with_capacity(30);
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            beacons.push(line.parse().unwrap());
        }

        Self { id, beacons }
    }

    fn adjust(&self, rotate: &[usize; 3], signs: &[i32; 3]) -> Self {
        Self {
            id: self.id,
            beacons: self
                .beacons
                .iter()
                .map(|b| b.adjust(rotate, signs))
                .collect(),
        }
    }
}

pub struct Input {
    scanners: Vec<Scanner>,
    n: usize,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();
        let mut scanners = Vec::with_capacity(40);

        while lines.peek().is_some() {
            scanners.push(Scanner::parse(scanners.len(), &mut lines));
        }

        Ok(Self::new(scanners))
    }
}

const ROTATES: [[usize; 3]; 6] = [
    [0, 1, 2],
    [0, 2, 1],
    [1, 0, 2],
    [1, 2, 0],
    [2, 0, 1],
    [2, 1, 0],
];

const SIGNS: [[i32; 3]; 8] = [
    [1, 1, 1],
    [1, 1, -1],
    [1, -1, 1],
    [1, -1, -1],
    [-1, 1, 1],
    [-1, 1, -1],
    [-1, -1, 1],
    [-1, -1, -1],
];
impl Input {
    fn new(scanners: Vec<Scanner>) -> Self {
        Self {
            n: scanners.len(),
            scanners,
        }
    }

    fn find_global_positions_for_scanners_and_beacons(self) -> (usize, HashMap<usize, [i32; 3]>) {
        let mut matched_scanners = HashMap::new();
        matched_scanners.entry(0).or_insert([0; 3]);

        let rotations: Vec<Vec<Scanner>> = self
            .scanners
            .iter()
            .skip(1)
            .map(|scanner| {
                iproduct!(ROTATES.iter(), SIGNS.iter())
                    .map(|(rotate, signs)| scanner.adjust(rotate, signs))
                    .collect()
            })
            .collect();

        let mut known_beacons: HashSet<Beacon> = self
            .scanners
            .first()
            .unwrap()
            .beacons
            .iter()
            .cloned()
            .collect();

        while matched_scanners.len() != self.n {
            for scanner in &self.scanners {
                if matched_scanners.contains_key(&scanner.id) {
                    continue;
                }

                for rotate in &rotations[scanner.id - 1] {
                    let overlap = iproduct!(rotate.beacons.iter(), known_beacons.iter())
                        .fold(HashMap::new(), |mut acc, (this_beacon, known_beacon)| {
                            *acc.entry(known_beacon.diff(this_beacon)).or_insert(0) += 1;
                            acc
                        })
                        .iter()
                        .find(|(_, v)| v >= &&12)
                        .map(|(k, _)| k.coords);
                    if let Some(diff) = overlap {
                        matched_scanners.insert(scanner.id, diff);

                        known_beacons
                            .extend(rotate.beacons.iter().map(|beacon| beacon.shift(&diff)));
                        break;
                    }
                }
            }
        }

        (known_beacons.len(), matched_scanners)
    }
}

pub fn input_generator(input: &str) -> Input {
    input.parse().unwrap()
}

pub fn part1(input: Input) -> usize {
    input.find_global_positions_for_scanners_and_beacons().0
}

pub fn part2(input: Input) -> i32 {
    let (_, matched_scanners) = input.find_global_positions_for_scanners_and_beacons();

    iproduct!(matched_scanners.values(), matched_scanners.values())
        .map(|(lhs, rhs)| lhs.iter().zip(rhs.iter()).map(|(l, r)| (l - r).abs()).sum())
        .max()
        .unwrap()
}
