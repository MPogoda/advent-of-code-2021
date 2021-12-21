use std::{collections::HashSet, ops::RangeInclusive, str::FromStr};

pub struct State {
    algorithm: Vec<bool>,
    lit_pixels: HashSet<(isize, isize)>,
    h: isize,
    infinity_pixels_lit: bool,
}

impl FromStr for State {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();
        let algorithm = lines
            .next()
            .unwrap()
            .as_bytes()
            .into_iter()
            .map(|&ch| ch == b'#')
            .collect();

        lines.next();

        let h = lines.peek().unwrap().len() as isize;

        let middle = h / 2;

        let lit_pixels = lines
            .enumerate()
            .fold(HashSet::new(), |mut acc, (i, line)| {
                acc.extend(
                    line.as_bytes()
                        .into_iter()
                        .enumerate()
                        .filter_map(|(j, &ch)| {
                            if ch == b'#' {
                                Some((i as isize - middle, j as isize - middle))
                            } else {
                                None
                            }
                        }),
                );
                acc
            });

        Ok(Self {
            algorithm,
            h,
            lit_pixels,
            infinity_pixels_lit: false,
        })
    }
}

pub fn input_generator(s: &str) -> State {
    s.parse().unwrap()
}

impl State {
    fn should_be_lit(&self, (i, j): &(isize, isize)) -> bool {
        let index = iproduct!(
            RangeInclusive::new(i - 1, i + 1),
            RangeInclusive::new(j - 1, j + 1)
        )
        .map(|(y, x)| {
            if self.lit_pixels.contains(&(y, x)) {
                1
            } else if x < -self.h || x > self.h || y < -self.h || y > self.h {
                if self.infinity_pixels_lit {
                    1
                } else {
                    0
                }
            } else {
                0
            }
        })
        .fold(0, |acc, v| (acc << 1) + v);

        self.algorithm[index]
    }

    fn evolve(self) -> Self {
        let h = self.h + 1;

        let lit_pixels = iproduct!(RangeInclusive::new(-h, h), RangeInclusive::new(-h, h))
            .filter(|pos| self.should_be_lit(pos))
            .fold(HashSet::new(), |mut acc, pos| {
                acc.insert(pos);
                acc
            });

        let infinity_pixels_lit = !self.infinity_pixels_lit && self.algorithm[0];

        Self {
            h,
            lit_pixels,
            infinity_pixels_lit,
            algorithm: self.algorithm,
        }
    }
}
pub fn part1(state: State) -> usize {
    (0..2).fold(state, |acc, _| acc.evolve()).lit_pixels.len()
}
pub fn part2(state: State) -> usize {
    (0..50).fold(state, |acc, _| acc.evolve()).lit_pixels.len()
}
