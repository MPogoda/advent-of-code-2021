use std::collections::VecDeque;

use itertools::Itertools;

pub struct Field {
    field: Vec<Vec<u8>>,
    h: usize,
    w: usize,
}

impl Field {
    fn parse(input: &str) -> Self {
        let field = input
            .lines()
            .map(|line| line.as_bytes().to_vec())
            .collect_vec();
        let h = field.len();
        let w = field[0].len();

        Self { field, h, w }
    }

    fn sum_of_risk_levels(&self) -> usize {
        iproduct!(0..self.h, 0..self.w)
            .filter_map(|(i, j)| {
                let c = self.field[i][j];
                if i < self.h - 1 && self.field[i + 1][j] <= c {
                    return None;
                }
                if i > 0 && self.field[i - 1][j] <= c {
                    return None;
                }
                if j < self.w - 1 && self.field[i][j + 1] <= c {
                    return None;
                }
                if j > 0 && self.field[i][j - 1] <= c {
                    return None;
                }
                Some((c - b'0') as usize + 1)
            })
            .sum()
    }

    fn find_basin(&mut self, i: usize, j: usize) -> Option<usize> {
        if self.field[i][j] == b'9' {
            return None;
        }
        let mut queue = VecDeque::new();
        let mut size = 0;
        queue.push_back((i, j));
        while !queue.is_empty() {
            let (x, y) = queue.pop_front().unwrap();
            if self.field[x][y] == b'9' {
                continue;
            }
            size += 1;
            self.field[x][y] = b'9';
            if x < self.h - 1 {
                queue.push_back((x + 1, y));
            }
            if x > 0 {
                queue.push_back((x - 1, y));
            }
            if y < self.w - 1 {
                queue.push_back((x, y + 1));
            }
            if y > 0 {
                queue.push_back((x, y - 1));
            }
        }
        Some(size)
    }

    fn find_basins(mut self) -> Vec<usize> {
        iproduct!(0..self.h, 0..self.w)
            .filter_map(|(i, j)| self.find_basin(i, j))
            .collect()
    }
}

pub fn input_generator(input: &str) -> Field {
    Field::parse(input)
}

pub fn part1(field: Field) -> usize {
    field.sum_of_risk_levels()
}

pub fn part2(field: Field) -> usize {
    let mut basins = field.find_basins();
    let (less_than, third, _) = basins.select_nth_unstable_by(2, |lhs, rhs| rhs.cmp(lhs));
    less_than.iter().product::<usize>() * *third
}
