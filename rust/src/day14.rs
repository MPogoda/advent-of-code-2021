use std::{collections::HashMap, str::FromStr};

type Pair = (u8, u8);
pub struct Input {
    template: HashMap<Pair, usize>,
    rules: HashMap<Pair, u8>,
    last: u8,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let template_string = lines.next().unwrap().as_bytes();
        let template = template_string
            .windows(2)
            .map(|slice| (slice[0], slice[1]))
            .fold(HashMap::new(), |mut acc, v| {
                *acc.entry(v).or_insert(0) += 1;
                acc
            });
        let last = *template_string.last().unwrap();

        lines.next();

        let rules = lines.map(|line| line.split_once(" -> ").unwrap()).fold(
            HashMap::new(),
            |mut acc, (k, v)| {
                let key = (k.as_bytes()[0], k.as_bytes()[1]);
                acc.insert(key, v.as_bytes()[0]);
                acc
            },
        );

        Ok(Self {
            template,
            rules,
            last,
        })
    }
}

impl Input {
    fn evolve(self) -> Self {
        let template = self
            .template
            .iter()
            .fold(HashMap::new(), |mut acc, (&k, v)| {
                match self.rules.get(&k) {
                    None => {
                        *acc.entry(k).or_insert(0) += v;
                    }
                    Some(&middle) => {
                        let left = (k.0, middle);
                        let right = (middle, k.1);
                        *acc.entry(left).or_insert(0) += v;
                        *acc.entry(right).or_insert(0) += v;
                    }
                }

                acc
            });
        Self {
            template,
            rules: self.rules,
            last: self.last,
        }
    }

    fn answer(self) -> usize {
        let freqs = self.template.into_iter().fold(
            [(self.last, 1)].into(),
            |mut acc: HashMap<u8, usize>, (k, v)| {
                *acc.entry(k.0).or_insert(0) += v;
                acc
            },
        );
        let min = freqs.values().min().unwrap();
        let max = freqs.values().max().unwrap();
        max - min
    }
}

pub fn input_generator(input: &str) -> Input {
    input.parse().unwrap()
}

pub fn part1(input: Input) -> usize {
    (0..10).into_iter().fold(input, |v, _| v.evolve()).answer()
}

pub fn part2(input: Input) -> usize {
    (0..40).into_iter().fold(input, |v, _| v.evolve()).answer()
}
