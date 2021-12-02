#[derive(Clone, Copy, Debug)]
pub enum Command {
    FORWARD,
    UP,
    DOWN,
}
pub type Input = Vec<(Command, usize)>;

fn parse_command(raw: &str) -> Command {
    match raw.as_bytes()[0] {
        b'f' => Command::FORWARD,
        b'u' => Command::UP,
        _ => Command::DOWN,
    }
}

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (raw_command, value) = line.split_once(' ').unwrap();
            (parse_command(raw_command), value.parse().unwrap())
        })
        .collect()
}

pub fn part1(commands: Input) -> usize {
    let pos = commands
        .into_iter()
        .fold((0, 0), |(x, y), (cmd, v)| match cmd {
            Command::FORWARD => (x + v, y),
            Command::UP => (x, y - v),
            Command::DOWN => (x, y + v),
        });
    pos.0 * pos.1
}

pub fn part2(commands: Input) -> usize {
    let pos = commands
        .into_iter()
        .fold((0, 0, 0), |(x, y, aim), (cmd, v)| match cmd {
            Command::FORWARD => (x + v, y + aim * v, aim),
            Command::UP => (x, y, aim - v),
            Command::DOWN => (x, y, aim + v),
        });
    pos.0 * pos.1
}
