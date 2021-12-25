use std::{collections::HashSet, str::FromStr};

pub struct State {
    to_east: HashSet<(usize, usize)>,
    to_south: HashSet<(usize, usize)>,
    h: usize,
    w: usize,
}

impl FromStr for State {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut to_east = HashSet::new();
        let mut to_south = HashSet::new();
        let mut h = 0;
        let mut w = 0;
        for (i, j, ch) in s.lines().enumerate().flat_map(|(i, line)| {
            line.trim_end()
                .as_bytes()
                .iter()
                .enumerate()
                .map(move |(j, ch)| (i, j, ch))
        }) {
            match ch {
                b'v' => {
                    to_south.insert((i, j));
                }
                b'>' => {
                    to_east.insert((i, j));
                }
                _ => {}
            }
            h = h.max(i);
            w = w.max(j);
        }

        Ok(Self {
            to_east,
            to_south,
            h: h + 1,
            w: w + 1,
        })
    }
}

impl State {
    fn evolve(&self) -> Option<Self> {
        let (to_east, has_moved_east) = self.evolve_east();
        let (to_south, has_moved_south) = self.evolve_south(&to_east);

        if has_moved_east || has_moved_south {
            Some(Self {
                to_east,
                to_south,
                h: self.h,
                w: self.w,
            })
        } else {
            None
        }
    }

    fn evolve_east(&self) -> (HashSet<(usize, usize)>, bool) {
        let mut result = HashSet::new();
        let mut has_moved = false;
        for &(i, j) in self.to_east.iter() {
            let next_pos = (i, (j + 1) % self.w);
            if !self.to_east.contains(&next_pos) && !self.to_south.contains(&next_pos) {
                result.insert(next_pos);
                has_moved = true;
            } else {
                result.insert((i, j));
            }
        }

        (result, has_moved)
    }

    fn evolve_south(&self, to_east: &HashSet<(usize, usize)>) -> (HashSet<(usize, usize)>, bool) {
        let mut has_moved = false;
        let mut result = HashSet::new();
        for &(i, j) in self.to_south.iter() {
            let next_pos = ((i + 1) % self.h, j);
            if !to_east.contains(&next_pos) && !self.to_south.contains(&next_pos) {
                result.insert(next_pos);
                has_moved = true;
            } else {
                result.insert((i, j));
            }
        }

        (result, has_moved)
    }
}

pub fn input_generator(s: &str) -> State {
    s.parse().unwrap()
}

pub fn part1(mut state: State) -> usize {
    let mut step = 1;

    while let Some(new_state) = state.evolve() {
        state = new_state;
        step += 1;
    }

    step
}
