use std::collections::HashMap;

type Input = (u8, u8);

fn parse_player(s: &str) -> u8 {
    s.split_once(": ").unwrap().1.parse().unwrap()
}

pub fn input_generator(s: &str) -> Input {
    let mut lines = s.lines();
    let p1 = parse_player(lines.next().unwrap());
    let p2 = parse_player(lines.next().unwrap());
    (p1, p2)
}

struct Die {
    rolls: usize,
    next: u8,
}

impl Die {
    fn new() -> Self {
        Self { rolls: 0, next: 1 }
    }

    fn roll(&mut self) -> u8 {
        let r = self.next;

        self.rolls += 1;
        self.next += 1;
        if self.next > 100 {
            self.next = 1;
        }

        r
    }
}

fn perform_moves(pos: u8, die: &mut Die) -> u8 {
    (0..3)
        .into_iter()
        .map(|_| die.roll())
        .fold(pos, |p, v| 1 + ((p - 1 + v) % 10))
}

pub fn part1((p1, p2): Input) -> usize {
    let mut die = Die::new();

    let mut players = [(p1, 0), (p2, 0)];
    let mut turn = 0;

    const W: usize = 1000;

    while players.iter().all(|(_, score)| score < &W) {
        let player = turn % 2;
        let next_pos = perform_moves(players[player].0, &mut die);
        players[player].0 = next_pos;
        players[player].1 += next_pos as usize;

        turn += 1;
    }

    die.rolls * players[0].1.min(players[1].1)
}

const W: u8 = 21;
const MOVES_COUNT: [(u8, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn play(start: u8) -> Vec<usize> {
    let mut wins_at_round = Vec::new();

    let mut state = HashMap::new();
    state.insert((start, 0), 1);

    while !state.is_empty() {
        let mut this_round_wins = 0;
        let mut next_state = HashMap::new();

        for ((pos, score), v) in state {
            for (moves, count) in MOVES_COUNT {
                let next_pos = 1 + ((pos + moves - 1) % 10);
                let next_score = score + next_pos;
                let num_games = v * count;

                if next_score >= W {
                    this_round_wins += num_games;
                } else {
                    *next_state.entry((next_pos, next_score)).or_insert(0) += num_games;
                }
            }
        }

        state = next_state;
        wins_at_round.push(this_round_wins);
    }

    wins_at_round
}

pub fn part2((p1, p2): Input) -> usize {
    let p1wins = play(p1);
    let p2wins = play(p2);

    let total_p1 = p1wins
        .iter()
        .zip(p2wins.iter())
        .fold((0, 1), |(r, prod), (w1, w2)| {
            let next_r = r + w1 * prod;
            let next_prod = 27 * prod - w2;
            (next_r, next_prod)
        })
        .0;
    let total_p2 = p1wins
        .iter()
        .zip(p2wins.iter())
        .fold((0, 1), |(r, prod), (w1, w2)| {
            let next_prod = 27 * prod - w1;
            (r + w2 * next_prod, next_prod)
        })
        .0;
    total_p1.max(total_p2)
}
