use std::str::{FromStr, Lines};

use regex::Regex;

lazy_static! {
    static ref RE_A: Regex = Regex::new("^add x (.+)$").unwrap();
    static ref RE_B: Regex = Regex::new("^div z (.+)$").unwrap();
    static ref RE_C: Regex = Regex::new("^add y (.+)$").unwrap();
}

// There are 14 blocks (for each input digit)
// Each block has:
// - w = INPUT[i] // line 0
// - x = z % 26 + {A} // {A} is number from line 5
// - z = z / {B} // {B} is number from line 4
// - y = (x != w) * 25 + 1
// -    z = z * y
// -    z += (w + {C}) * (x != w) // {C} is number from line 15
//
// last block can be described as:
// - if x != w { z *= 26; z += w + {C} }
struct DigitFunc {
    a: i64,
    b: i64,
    c: i64,
}

impl DigitFunc {
    fn parse_with_regex(re: &Regex, s: &str) -> i64 {
        re.captures(s)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap()
    }
    fn from_lines(lines: &mut Lines) -> Self {
        let b = Self::parse_with_regex(&RE_B, lines.nth(4).unwrap());
        let a = Self::parse_with_regex(&RE_A, lines.next().unwrap());
        let c = Self::parse_with_regex(&RE_C, lines.nth(9).unwrap());

        lines.nth(1);

        Self { a, b, c }
    }

    fn calculate(&self, mut z: i64, w: i64) -> i64 {
        let x = self.a + (z % 26);
        z /= self.b;
        if x != w {
            z *= 26;
            z += w + self.c;
        }
        z
    }
}

pub struct Input {
    digit_funcs: Vec<DigitFunc>,
    max_z: Vec<i64>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = Vec::new();

        let mut lines = s.lines();
        while lines.clone().next().is_some() {
            digits.push(DigitFunc::from_lines(&mut lines));
        }

        Ok(Self {
            max_z: Vec::with_capacity(digits.len()),
            digit_funcs: digits,
        })
    }
}

impl Input {
    fn fill_max_z(&mut self) {
        let mut max_z = self.digit_funcs.iter().map(|o| o.b).product();
        for o in &self.digit_funcs {
            self.max_z.push(max_z);
            max_z /= o.b;
        }
    }

    fn solve<R>(mut self, digits: R) -> i64
    where
        R: Iterator<Item = i64> + Clone,
    {
        self.fill_max_z();
        self.solve_impl(digits, 0, 0).unwrap()
    }

    fn solve_impl<R>(&mut self, digits: R, pos: usize, current_z: i64) -> Option<i64>
    where
        R: Iterator<Item = i64> + Clone,
    {
        if pos == 14 {
            return if current_z == 0 { Some(0) } else { None };
        }
        if current_z > self.max_z[pos] {
            return None;
        }

        for w in digits.clone() {
            let z = self.digit_funcs[pos].calculate(current_z, w);
            if let Some(v) = self.solve_impl(digits.clone(), pos + 1, z) {
                return Some(w * 10i64.pow(13 - pos as u32) + v);
            }
        }

        None
    }
}

pub fn input_generator(s: &str) -> Input {
    s.parse().unwrap()
}

pub fn part1(input: Input) -> i64 {
    input.solve((1..10).rev())
}

pub fn part2(input: Input) -> i64 {
    input.solve(1..10)
}
