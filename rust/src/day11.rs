use std::{collections::VecDeque, str::FromStr};

const N: usize = 10;
pub struct Input {
    field: Vec<u8>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input {
            field: s
                .as_bytes()
                .into_iter()
                .filter_map(|ch| match ch {
                    b'0'..=b'9' => Some(ch - b'0'),
                    _ => None,
                })
                .collect(),
        })
    }
}

impl Input {
    fn increase_energy_level(&mut self, i: usize, j: usize) -> bool {
        self.field[i * N + j] += 1;
        if self.field[i * N + j] > 9 {
            self.field[i * N + j] = 0;
            return true;
        }

        false
    }

    fn get_neighbour((dx, dy): (i8, i8), (x, y): (usize, usize)) -> Option<(usize, usize)> {
        if (dx == 0 && dy == 0)
            || (dx == -1 && x == 0)
            || (dx == 1 && x == N - 1)
            || (dy == -1 && y == 0)
            || (dy == 1 && y == N - 1)
        {
            return None;
        }

        Some(((x as i8 + dx) as usize, (y as i8 + dy) as usize))
    }

    fn evolve(&mut self) -> usize {
        let mut queue = VecDeque::new();
        for (i, j) in iproduct!(0..N, 0..N) {
            if self.increase_energy_level(i, j) {
                queue.push_back((i, j));
            }
        }
        let mut flashes = 0;
        while !queue.is_empty() {
            let coord = queue.pop_front().unwrap();
            flashes += 1;
            for dxy in iproduct!(-1..=1, -1..=1) {
                let neighbour = Self::get_neighbour(dxy, coord);
                if neighbour.is_none() {
                    continue;
                }
                let (i, j) = neighbour.unwrap();
                if self.field[i * N + j] == 0 {
                    continue;
                }
                if self.increase_energy_level(i, j) {
                    queue.push_back((i, j));
                }
            }
        }
        flashes
    }
}

pub fn input_generator(input: &str) -> Input {
    input.parse().unwrap()
}

pub fn part1(mut input: Input) -> usize {
    (0..100).map(|_| input.evolve()).sum()
}

pub fn part2(mut input: Input) -> usize {
    1 + (0..).take_while(|_| input.evolve() != N * N).count()
}
