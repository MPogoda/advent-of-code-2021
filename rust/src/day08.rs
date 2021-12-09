use itertools::Itertools;

type Input = Vec<Task>;
pub struct Task {
    digits: Vec<usize>,
    output: Vec<usize>,
}

impl Task {
    fn parse(line: &str) -> Self {
        let (digits, output) = line.split_once('|').unwrap();
        Self {
            digits: digits
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(parse_digit)
                .collect(),
            output: output
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(parse_digit)
                .collect(),
        }
    }

    fn find_1(&self) -> usize {
        *self.digits.iter().find(|d| d.count_ones() == 2).unwrap()
    }
    fn find_7(&self) -> usize {
        *self.digits.iter().find(|d| d.count_ones() == 3).unwrap()
    }
    fn find_4(&self) -> usize {
        *self.digits.iter().find(|d| d.count_ones() == 4).unwrap()
    }
    fn find_8(&self) -> usize {
        *self.digits.iter().find(|d| d.count_ones() == 7).unwrap()
    }
    fn find_9(&self, d4: usize, top: usize) -> usize {
        let almost_d9 = d4 | top;
        *self
            .digits
            .iter()
            .find(|&d| d.count_ones() == 6 && (d & !almost_d9).count_ones() == 1)
            .unwrap()
    }
    fn find_5(&self, d9: usize, d1: usize) -> usize {
        *self
            .digits
            .iter()
            .find(|&d| {
                d.count_ones() == 5 && (d & d9).count_ones() == 5 && (d & d1).count_ones() == 1
            })
            .unwrap()
    }
    fn find_2(&self, d5: usize) -> usize {
        *self
            .digits
            .iter()
            .find(|&d| d.count_ones() == 5 && (d & d5).count_ones() == 3)
            .unwrap()
    }
    fn find_3(&self, d2: usize, d5: usize) -> usize {
        *self
            .digits
            .iter()
            .find(|&&d| d != d5 && d != d2 && d.count_ones() == 5)
            .unwrap()
    }
    fn find_0(&self, d1: usize, left_bottom: usize) -> usize {
        *self
            .digits
            .iter()
            .find(|&d| d.count_ones() == 6 && (d & (d1 | left_bottom)).count_ones() == 3)
            .unwrap()
    }

    fn count_easy_digits(&self) -> usize {
        let d1 = self.find_1();
        let d4 = self.find_4();
        let d7 = self.find_7();
        let d8 = self.find_8();

        self.output
            .iter()
            .filter(|&&d| d == d1 || d == d4 || d == d7 || d == d8)
            .count()
    }

    fn parse_output(&self) -> usize {
        let mut d = [0; 10];
        d[1] = self.find_1();
        d[4] = self.find_4();
        d[7] = self.find_7();
        d[8] = self.find_8();

        let top = d[7] & !d[1];
        d[9] = self.find_9(d[4], top);
        let left_bottom = d[8] & !d[9];
        d[5] = self.find_5(d[9], d[1]);
        d[6] = left_bottom | d[5];
        d[2] = self.find_2(d[5]);

        d[3] = self.find_3(d[2], d[5]);
        d[0] = self.find_0(d[1], left_bottom);

        (0..4)
            .map(|pos| {
                (0..10)
                    .find(|&i| d[i] == self.output[pos])
                    .unwrap()
                    .to_string()
            })
            .collect_vec()
            .join("")
            .parse()
            .unwrap()
    }
}

fn parse_digit(s: &str) -> usize {
    usize::from_str_radix(
        std::str::from_utf8(
            (b'a'..=b'g')
                .map(|ch| {
                    if s.as_bytes().contains(&ch) {
                        b'1'
                    } else {
                        b'0'
                    }
                })
                .collect_vec()
                .as_slice(),
        )
        .unwrap(),
        2,
    )
    .unwrap()
}

pub fn input_generator(input: &str) -> Input {
    input.lines().map(Task::parse).collect()
}

pub fn part1(entries: Input) -> usize {
    entries.iter().map(Task::count_easy_digits).sum()
}

pub fn part2(entries: Input) -> usize {
    entries.iter().map(Task::parse_output).sum()
}
