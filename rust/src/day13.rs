use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
    str::{from_utf8, FromStr},
};

enum Fold {
    X(usize),
    Y(usize),
}
pub struct Input {
    paper: HashSet<(usize, usize)>,
    folds: VecDeque<Fold>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let mut paper = HashSet::new();
        let mut folds = VecDeque::new();
        loop {
            let line = lines.next().unwrap();
            if line.is_empty() {
                break;
            }

            let (x, y) = line.split_once(',').unwrap();
            paper.insert((x.parse().unwrap(), y.parse().unwrap()));
        }
        for line in lines {
            let (dir, v) = line.split_once('=').unwrap();
            let num = v.parse().unwrap();
            folds.push_back(match *dir.as_bytes().last().unwrap() {
                b'x' => Fold::X(num),
                _ => Fold::Y(num),
            });
        }

        Ok(Self { paper, folds })
    }
}

impl Input {
    fn transform((x, y): (usize, usize), fold: &Fold) -> Option<(usize, usize)> {
        match fold {
            Fold::X(v) => match x.cmp(&v) {
                Ordering::Less => Some((x, y)),
                Ordering::Equal => None,
                Ordering::Greater => Some((2 * v - x, y)),
            },
            Fold::Y(v) => match y.cmp(&v) {
                Ordering::Less => Some((x, y)),
                Ordering::Equal => None,
                Ordering::Greater => Some((x, 2 * v - y)),
            },
        }
    }

    fn fold(mut self) -> Self {
        if self.folds.is_empty() {
            panic!("oh no");
        }

        let fold = self.folds.pop_front().unwrap();
        Self {
            folds: self.folds,
            paper: self
                .paper
                .into_iter()
                .filter_map(|coord| Input::transform(coord, &fold))
                .collect(),
        }
    }

    fn get_size(&self) -> (usize, usize) {
        self.folds.iter().fold((0, 0), |(x, y), fold| match fold {
            Fold::X(v) => (*v, y),
            Fold::Y(v) => (x, *v),
        })
    }

    fn draw(self, (w, h): (usize, usize)) {
        let mut field = Vec::with_capacity(h);
        for _ in 0..h {
            let mut row = Vec::with_capacity(w);
            row.resize(w, b'.');
            field.push(row);
        }

        for (x, y) in self.paper {
            field[y][x] = b'#';
        }

        println!("");
        for row in field {
            println!("{}", from_utf8(&row).unwrap());
        }
    }
}

pub fn input_generator(input: &str) -> Input {
    input.parse().unwrap()
}

pub fn part1(input: Input) -> usize {
    let state = input.fold();
    state.paper.len()
}

pub fn part2(mut input: Input) -> usize {
    let size = input.get_size();
    for _ in 0..input.folds.len() {
        input = input.fold();
    }

    input.draw(size);

    0
}
