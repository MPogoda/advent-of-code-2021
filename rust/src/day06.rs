type Input = Vec<usize>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect()
}

const PERIOD: usize = 7;
const STATE_SIZE: usize = PERIOD + 2;
struct State {
    state: [usize; STATE_SIZE],
}

impl State {
    fn init(ds: Input) -> Self {
        Self {
            state: ds.iter().fold([0; STATE_SIZE], |mut acc, &v| {
                acc[v] += 1;
                acc
            }),
        }
    }
    fn evolve(self) -> Self {
        Self {
            state: self
                .state
                .iter()
                .enumerate()
                .fold([0; STATE_SIZE], |mut acc, (k, &v)| {
                    if k == 0 {
                        acc[PERIOD - 1] = v;
                        acc[PERIOD + 1] = v;
                    } else {
                        acc[k - 1] += v;
                    }
                    acc
                }),
        }
    }

    fn answer(self) -> usize {
        self.state.iter().sum()
    }
}

pub fn part1(ds: Input) -> usize {
    (0..80)
        .fold(State::init(ds), |state, _| state.evolve())
        .answer()
}

pub fn part2(ds: Input) -> usize {
    (0..256)
        .fold(State::init(ds), |state, _| state.evolve())
        .answer()
}
