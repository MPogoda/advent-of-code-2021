type Input = Vec<u16>;

pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part1(measurements: Input) -> usize {
    measurements
        .into_iter()
        .fold((u16::MAX, 0), |(prev, ans), current| {
            (current, if prev < current { ans + 1 } else { ans })
        })
        .1
}

pub fn part2(measurements: Input) -> usize {
    measurements
        .windows(3)
        .fold((u16::MAX, 0), |(prev, ans), current_window| {
            let current = current_window.into_iter().sum();
            (current, if prev < current { ans + 1 } else { ans })
        })
        .1
}
