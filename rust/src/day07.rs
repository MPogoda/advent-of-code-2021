use std::collections::HashMap;

type Input = (HashMap<usize, usize>, usize, usize);

pub fn input_generator(input: &str) -> Input {
    input
        .trim_end()
        .split(',')
        .map(|v| v.parse().unwrap())
        .fold(
            (HashMap::new(), usize::MAX, usize::MIN),
            |(mut acc, min, max), v| {
                acc.entry(v).and_modify(|e| *e += 1).or_insert(1);

                (acc, min.min(v), max.max(v))
            },
        )
}

fn solve<F>(freq: HashMap<usize, usize>, min: usize, max: usize, cost: F) -> usize
where
    F: Fn(usize, usize) -> usize,
{
    (min..=max)
        .map(|pos| {
            freq.iter()
                .map(|(&k, &v)| cost(v, if pos > k { pos - k } else { k - pos }))
                .sum()
        })
        .min()
        .unwrap()
}

pub fn part1((freq, min, max): Input) -> usize {
    solve(freq, min, max, |count, diff| count * diff)
}

pub fn part2((freq, min, max): Input) -> usize {
    solve(freq, min, max, |count, diff| count * diff * (diff + 1) / 2)
}
