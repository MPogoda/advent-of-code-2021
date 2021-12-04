use std::collections::HashSet;

type Input = (Vec<u8>, Vec<Field>);

pub struct Field {
    field: Vec<Vec<Option<u8>>>,
    id: usize,
}

impl Field {
    fn play(self: &mut Self, n: u8) -> bool {
        for i in 0..self.field.len() {
            for j in 0..self.field[i].len() {
                if self.field[i][j] == Some(n) {
                    self.field[i][j] = None;

                    if self.field[i].iter().all(|x| x.is_none()) {
                        return true;
                    }
                    if self.field.iter().all(|row| row[j].is_none()) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn score(self: &Self, n: u8) -> usize {
        let sum: usize = self
            .field
            .iter()
            .map(|row| {
                row.iter()
                    .map(|x| -> usize { x.unwrap_or(0).into() })
                    .sum::<usize>()
            })
            .sum();
        n as usize * sum
    }
}

pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();
    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    lines.next();

    let mut fields = Vec::new();
    while lines.clone().next().is_some() {
        let field = lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .map(|l| {
                l.split(' ')
                    .filter(|n| !n.is_empty())
                    .map(|n| n.parse().ok())
                    .collect()
            })
            .collect();
        fields.push(Field {
            field,
            id: fields.len(),
        });
    }

    (numbers, fields)
}

pub fn part1((numbers, mut fields): Input) -> usize {
    for n in numbers {
        for field in &mut fields {
            if field.play(n) {
                return field.score(n);
            }
        }
    }
    0
}

pub fn part2((numbers, mut fields): Input) -> usize {
    let total = fields.len();
    let mut completed = HashSet::new();
    for n in numbers {
        for field in &mut fields {
            if completed.contains(&field.id) {
                continue;
            }
            if field.play(n) {
                completed.insert(field.id);
                if completed.len() == total {
                    return field.score(n);
                }
            }
        }
    }
    0
}
