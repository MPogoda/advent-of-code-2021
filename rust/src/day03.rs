type Input = Vec<Vec<u8>>;

pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

pub fn part1(diagnostics: Input) -> usize {
    let word_length = diagnostics[0].len();
    let len = diagnostics.len();

    let mut rates_acc = Vec::with_capacity(word_length);
    rates_acc.resize(word_length, 0);
    let rates = diagnostics.into_iter().fold(rates_acc, |mut acc, line| {
        for i in 0..word_length {
            acc[i] += if line[i] == b'1' { 1 } else { 0 }
        }
        acc
    });

    let gamma = usize::from_str_radix(
        &rates
            .iter()
            .map(|&r| if r > len / 2 { '1' } else { '0' })
            .collect::<String>(),
        2,
    )
    .unwrap();
    let epsilon = usize::from_str_radix(
        &rates
            .iter()
            .map(|&r| if r > len / 2 { '0' } else { '1' })
            .collect::<String>(),
        2,
    )
    .unwrap();
    gamma * epsilon
}

fn choose_by_bit(diagnostics: Input, bit_number: usize, select_most_common: bool) -> Input {
    let len = diagnostics.len();
    if len == 1 {
        return diagnostics;
    }

    let rate = diagnostics
        .iter()
        .filter(|line| line[bit_number] == b'1')
        .count();

    let bit = if select_most_common != (2 * rate >= len) {
        b'1'
    } else {
        b'0'
    };

    diagnostics
        .into_iter()
        .filter(|line| line[bit_number] == bit)
        .collect()
}

fn find_one(mut diagnostics: Input, select_most_common: bool) -> usize {
    let word_length = diagnostics[0].len();
    for i in 0..word_length {
        diagnostics = choose_by_bit(diagnostics, i, select_most_common);
    }

    usize::from_str_radix(std::str::from_utf8(&diagnostics[0]).unwrap(), 2).unwrap()
}

pub fn part2(diagnostics: Input) -> usize {
    let oxygen = find_one(diagnostics.clone(), true);
    let co2 = find_one(diagnostics.clone(), false);

    oxygen * co2
}
