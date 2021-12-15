use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    str::FromStr,
};

pub struct Input {
    field: Vec<Vec<u8>>,
    n: usize,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let field = s
            .lines()
            .map(|line| line.as_bytes().into_iter().map(|ch| ch - b'0').collect())
            .collect();

        Ok(Self::new(field))
    }
}

impl Input {
    fn new(field: Vec<Vec<u8>>) -> Self {
        let n = field.len();
        assert_eq!(n, field[0].len());
        Self { field, n }
    }

    fn get_node(&self, ix: isize, iy: isize, cost_so_far: usize) -> Option<(usize, usize, usize)> {
        if ix < 0 || iy < 0 {
            return None;
        }
        let x = ix as usize;
        let y = iy as usize;
        if x >= self.n || y >= self.n {
            return None;
        }

        let cost = cost_so_far + self.field[y][x] as usize;
        Some((cost, x, y))
    }

    fn find_shortest_path(&self) -> usize {
        let mut heap = BinaryHeap::new();
        let mut visited = HashSet::new();

        heap.push(Reverse((0, 0, 0)));

        loop {
            let (node_cost, node_x, node_y) = heap.pop().unwrap().0;
            if node_x == self.n - 1 && node_y == self.n - 1 {
                return node_cost;
            }
            for new_node in [(1, 0), (0, 1), (-1isize, 0), (0, -1isize)]
                .iter()
                .map(|(dx, dy)| (node_x as isize + dx, node_y as isize + dy))
                .filter_map(|(ix, iy)| self.get_node(ix, iy, node_cost))
            {
                let pos = self.n * new_node.1 + new_node.2;
                if visited.contains(&pos) {
                    continue;
                }
                visited.insert(pos);
                heap.push(Reverse(new_node));
            }
        }
    }

    fn enlarge(self, factor: usize) -> Self {
        let field = (0..(factor * self.n))
            .map(|i| {
                let di = (i / self.n) as u8;
                let i0 = i % self.n;
                (0..(factor * self.n))
                    .map(|j| {
                        let dj = (j / self.n) as u8;
                        let j0 = j % self.n;
                        1 + (self.field[i0][j0] + di + dj - 1) % 9
                    })
                    .collect()
            })
            .collect();

        Self::new(field)
    }
}

pub fn input_generator(input: &str) -> Input {
    input.parse().unwrap()
}

pub fn part1(input: Input) -> usize {
    input.find_shortest_path()
}

pub fn part2(input: Input) -> usize {
    input.enlarge(5).find_shortest_path()
}
