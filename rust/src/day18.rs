use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum Node {
    Value(u8),
    Pair(Box<Node>, Box<Node>),
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse(&mut s.as_bytes().into_iter()))
    }
}

impl Node {
    fn pair(lhs: Self, rhs: Self) -> Self {
        Self::Pair(Box::new(lhs), Box::new(rhs))
    }

    fn parse<'a, T>(str: &mut T) -> Self
    where
        T: Iterator<Item = &'a u8> + Clone,
    {
        let &ch = str.next().unwrap();
        match ch {
            b'[' => {
                let lhs = Self::parse(str);
                str.next(); // ','
                let rhs = Self::parse(str);
                str.next(); // ']'
                Self::pair(lhs, rhs)
            }
            _ => Self::Value(ch - b'0'),
        }
    }

    fn add_left(&self, v: u8) -> Self {
        match self {
            Node::Value(value) => Node::Value(value + v),
            Node::Pair(lhs, rhs) => Node::pair(lhs.add_left(v), *rhs.clone()),
        }
    }

    fn add_right(&self, v: u8) -> Self {
        match self {
            Node::Value(value) => Node::Value(value + v),
            Node::Pair(lhs, rhs) => Node::pair(*lhs.clone(), rhs.add_right(v)),
        }
    }

    fn is_value(&self) -> bool {
        if let Node::Value(_) = self {
            true
        } else {
            false
        }
    }

    fn value(&self) -> u8 {
        if let Node::Value(v) = self {
            *v
        } else {
            panic!("bad node");
        }
    }

    fn try_explode_pair(depth: usize, lhs: &Self, rhs: &Self) -> Option<(Self, u8, u8)> {
        if lhs.is_value() && rhs.is_value() {
            if depth >= 4 {
                return Some((Node::Value(0), lhs.value(), rhs.value()));
            } else {
                return None;
            }
        }

        if let Some((new, left, right)) = lhs.try_explode(depth + 1) {
            Some((Node::pair(new, rhs.add_left(right)), left, 0))
        } else if let Some((new, left, right)) = rhs.try_explode(depth + 1) {
            Some((Node::pair(lhs.add_right(left), new), 0, right))
        } else {
            None
        }
    }

    fn try_explode(&self, depth: usize) -> Option<(Self, u8, u8)> {
        match self {
            Node::Value(_) => None,
            Node::Pair(lhs, rhs) => Self::try_explode_pair(depth, &*lhs, &*rhs),
        }
    }

    fn try_split(&self) -> Option<Self> {
        match self {
            Node::Value(value) => {
                if *value < 10 {
                    None
                } else {
                    Some(Node::pair(
                        Node::Value(value / 2),
                        Node::Value((value + 1) / 2),
                    ))
                }
            }
            Node::Pair(lhs, rhs) => {
                if let Some(new) = lhs.try_split() {
                    Some(Node::pair(new, *rhs.clone()))
                } else if let Some(new) = rhs.try_split() {
                    Some(Node::pair(*lhs.clone(), new))
                } else {
                    None
                }
            }
        }
    }

    fn reduce(self) -> Self {
        if let Some((exploded, _, _)) = self.try_explode(0) {
            exploded.reduce()
        } else if let Some(splitted) = self.try_split() {
            splitted.reduce()
        } else {
            self
        }
    }

    fn magnitude(self) -> usize {
        match self {
            Node::Value(v) => v as usize,
            Node::Pair(lhs, rhs) => 3 * lhs.magnitude() + 2 * rhs.magnitude(),
        }
    }
}

type Input = Vec<Node>;

pub fn input_generator(s: &str) -> Input {
    s.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part1(input: Input) -> usize {
    input
        .into_iter()
        .reduce(|lhs, rhs| Node::pair(lhs, rhs).reduce())
        .unwrap()
        .magnitude()
}

pub fn part2(input: Input) -> usize {
    iproduct!(0..input.len(), 0..input.len())
        .filter(|(i, j)| i != j)
        .map(|(i, j)| {
            Node::pair(input[i].clone(), input[j].clone())
                .reduce()
                .magnitude()
        })
        .max()
        .unwrap()
}
