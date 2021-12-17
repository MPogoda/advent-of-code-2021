use std::str::FromStr;

use itertools::Itertools;

#[derive(PartialEq, Eq)]
enum ID {
    Sum,
    Prod,
    Min,
    Max,
    Value,
    Gt,
    Lt,
    Eq,
}

impl ID {
    fn from_num(n: u16) -> Self {
        match n {
            0 => Self::Sum,
            1 => Self::Prod,
            2 => Self::Min,
            3 => Self::Max,
            4 => Self::Value,
            5 => Self::Gt,
            6 => Self::Lt,
            7 => Self::Eq,
            _ => panic!("ohno id"),
        }
    }
}

pub struct Packet {
    id: ID,
    version: u8,
    value: Option<usize>,
    packets: Option<Vec<Packet>>,
}

type Input = Packet;

impl FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s
            .trim_end()
            .as_bytes()
            .iter()
            .flat_map(|&ch| Packet::hex_to_binary(ch));
        Ok(Packet::parse_from_iter(&mut input))
    }
}

impl Packet {
    fn hex_to_binary(ch: u8) -> &'static [u8] {
        match ch {
            b'0' => &[b'0', b'0', b'0', b'0'],
            b'1' => &[b'0', b'0', b'0', b'1'],
            b'2' => &[b'0', b'0', b'1', b'0'],
            b'3' => &[b'0', b'0', b'1', b'1'],
            b'4' => &[b'0', b'1', b'0', b'0'],
            b'5' => &[b'0', b'1', b'0', b'1'],
            b'6' => &[b'0', b'1', b'1', b'0'],
            b'7' => &[b'0', b'1', b'1', b'1'],
            b'8' => &[b'1', b'0', b'0', b'0'],
            b'9' => &[b'1', b'0', b'0', b'1'],
            b'A' => &[b'1', b'0', b'1', b'0'],
            b'B' => &[b'1', b'0', b'1', b'1'],
            b'C' => &[b'1', b'1', b'0', b'0'],
            b'D' => &[b'1', b'1', b'0', b'1'],
            b'E' => &[b'1', b'1', b'1', b'0'],
            b'F' => &[b'1', b'1', b'1', b'1'],
            _ => panic!("oh no hex"),
        }
    }

    fn parse_num<'a, T>(iter: T) -> usize
    where
        T: Iterator<Item = &'a u8>,
    {
        let mut ans = 0usize;
        for &ch in iter {
            ans <<= 1;
            if ch == b'1' {
                ans += 1;
            }
        }

        ans
    }

    fn fetch_n_bits<'a, T>(iter: &mut T, n: usize) -> u16
    where
        T: Iterator<Item = &'a u8> + Clone,
    {
        let slice_iter = iter.clone().take(n);
        iter.nth(n - 1);
        Packet::parse_num(slice_iter) as u16
    }

    fn fetch_value<'a, T>(iter: &mut T) -> usize
    where
        T: Iterator<Item = &'a u8> + Clone,
    {
        let mut acc = 0usize;
        loop {
            let is_last = *iter.next().unwrap() == b'0';
            acc <<= 4;
            acc += Packet::parse_num(iter.clone().take(4));
            iter.nth(3);
            if is_last {
                break;
            }
        }

        acc
    }

    fn parse_packets_length_based<'a, T>(iter: &mut T) -> Vec<Packet>
    where
        T: Iterator<Item = &'a u8> + Clone,
    {
        let length = Packet::fetch_n_bits(iter, 15) as usize;
        let subpackets = iter.clone().take(length).collect_vec();
        iter.nth(length - 1);

        let mut packets = Vec::new();
        let mut sub_iter = subpackets.into_iter();
        while sub_iter.clone().count() > 6 {
            packets.push(Packet::parse_from_iter(&mut sub_iter));
        }
        packets
    }

    fn parse_packets_num_based<'a, T>(iter: &mut T) -> Vec<Packet>
    where
        T: Iterator<Item = &'a u8> + Clone,
    {
        let num = Packet::fetch_n_bits(iter, 11) as usize;
        let mut packets = Vec::with_capacity(num);
        for _ in 0..num {
            packets.push(Packet::parse_from_iter(iter));
        }
        packets
    }

    fn parse_from_iter<'a, T>(iter: &mut T) -> Self
    where
        T: Iterator<Item = &'a u8> + Clone,
    {
        let version = Packet::fetch_n_bits(iter, 3) as u8;
        let id = ID::from_num(Packet::fetch_n_bits(iter, 3));
        if id == ID::Value {
            let value = Some(Packet::fetch_value(iter));
            Self {
                id,
                version,
                value,
                packets: None,
            }
        } else {
            let is_length_based = *iter.next().unwrap() == b'0';
            let packets = Some(if is_length_based {
                Packet::parse_packets_length_based(iter)
            } else {
                Packet::parse_packets_num_based(iter)
            });

            Self {
                id,
                version,
                value: None,
                packets,
            }
        }
    }

    fn versions_sum(self) -> usize {
        self.packets
            .unwrap_or(Vec::new())
            .into_iter()
            .map(Packet::versions_sum)
            .sum::<usize>()
            + self.version as usize
    }

    fn process(self) -> usize {
        match self.id {
            ID::Sum => self.packets.unwrap().into_iter().map(Packet::process).sum(),
            ID::Prod => self
                .packets
                .unwrap()
                .into_iter()
                .map(Packet::process)
                .product(),
            ID::Min => self
                .packets
                .unwrap()
                .into_iter()
                .map(Packet::process)
                .min()
                .unwrap(),
            ID::Max => self
                .packets
                .unwrap()
                .into_iter()
                .map(Packet::process)
                .max()
                .unwrap(),
            ID::Value => self.value.unwrap(),
            ID::Gt => {
                let mut packets = self.packets.unwrap();
                assert_eq!(packets.len(), 2);
                let rhs = packets.pop().unwrap().process();
                let lhs = packets.pop().unwrap().process();
                if lhs > rhs {
                    1
                } else {
                    0
                }
            }
            ID::Lt => {
                let mut packets = self.packets.unwrap();
                assert_eq!(packets.len(), 2);
                let rhs = packets.pop().unwrap().process();
                let lhs = packets.pop().unwrap().process();
                if lhs < rhs {
                    1
                } else {
                    0
                }
            }
            ID::Eq => {
                let mut packets = self.packets.unwrap();
                assert_eq!(packets.len(), 2);
                let rhs = packets.pop().unwrap().process();
                let lhs = packets.pop().unwrap().process();
                if lhs == rhs {
                    1
                } else {
                    0
                }
            }
        }
    }
}

pub fn input_generator(input: &str) -> Input {
    input.parse().unwrap()
}

pub fn part1(packet: Input) -> usize {
    packet.versions_sum()
}

pub fn part2(packet: Input) -> usize {
    packet.process()
}
