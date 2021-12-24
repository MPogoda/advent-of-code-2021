use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    rc::Rc,
    str::FromStr,
};

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn from_byte(v: u8) -> Self {
        match v {
            b'A' => Self::Amber,
            b'B' => Self::Bronze,
            b'C' => Self::Copper,
            b'D' => Self::Desert,
            _ => panic!("wrong byte {}", v),
        }
    }

    fn get_desired_room(&self) -> usize {
        match self {
            Self::Amber => 0,
            Self::Bronze => 1,
            Self::Copper => 2,
            Self::Desert => 3,
        }
    }

    fn get_cost(&self) -> usize {
        match self {
            Self::Amber => 1,
            Self::Bronze => 10,
            Self::Copper => 100,
            Self::Desert => 1000,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct State {
    hallway: [Option<Amphipod>; 11],
    side_rooms: [Vec<Amphipod>; 4],
    room_capacity: usize,
}

impl FromStr for State {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let room_capacity = 2;
        let hallway = [None; 11];

        let lines = s.lines().skip(2).take(2).collect_vec();

        let side_rooms = [3, 5, 7, 9].map(|j| {
            [1, 0]
                .iter()
                .map(|&i| Amphipod::from_byte(lines[i].as_bytes()[j]))
                .collect()
        });

        Ok(Self {
            hallway,
            side_rooms,
            room_capacity,
        })
    }
}

impl State {
    fn is_final(&self) -> bool {
        self.hallway.iter().all(Option::is_none)
            && self.side_rooms.iter().enumerate().all(|(j, room)| {
                room.len() == self.room_capacity && room.iter().all(|a| a.get_desired_room() == j)
            })
    }

    fn get_next_states(&self) -> Vec<(usize, Rc<State>)> {
        let mut result = Vec::new();

        result.extend(
            self.hallway
                .iter()
                .enumerate()
                .filter_map(|(i, a)| a.as_ref().map(|&v| (i, v)))
                .filter_map(|(i, a)| self.try_move_from_hallway(i, a)),
        );

        for room in 0..self.side_rooms.len() {
            self.try_move_into_hallway(room, &mut result);
        }

        result
    }

    fn try_move_into_hallway(&self, room: usize, result: &mut Vec<(usize, Rc<State>)>) {
        if self.side_rooms[room].is_empty() {
            return;
        }
        if self.side_rooms[room]
            .iter()
            .all(|a| a.get_desired_room() == room)
        {
            return;
        }

        let hallway_index = 2 * (room + 1);
        for pos in (0..hallway_index).rev() {
            if [2, 4, 6, 8].contains(&pos) {
                continue;
            }
            if let Some(v) = self.try_move_into_hallway_at_pos(room, pos) {
                result.push(v);
            } else {
                break;
            }
        }

        for pos in hallway_index + 1..self.hallway.len() {
            if [2, 4, 6, 8].contains(&pos) {
                continue;
            }
            if let Some(v) = self.try_move_into_hallway_at_pos(room, pos) {
                result.push(v);
            } else {
                break;
            }
        }
    }

    fn try_move_into_hallway_at_pos(&self, room: usize, pos: usize) -> Option<(usize, Rc<State>)> {
        if self.hallway[pos].is_some() {
            return None;
        }

        let mut next_state = self.clone();
        let amphipod = next_state.side_rooms[room].pop().unwrap();
        next_state.hallway[pos] = Some(amphipod);

        let hallway_index = 2 * (room + 1);
        let steps = hallway_index.max(pos) - hallway_index.min(pos) + next_state.room_capacity
            - next_state.side_rooms[room].len();
        let cost = amphipod.get_cost() * steps;

        Some((cost, Rc::new(next_state)))
    }

    fn try_move_from_hallway(
        &self,
        hallway_index: usize,
        amphipod: Amphipod,
    ) -> Option<(usize, Rc<State>)> {
        let desired_room = amphipod.get_desired_room();
        if self.side_rooms[desired_room]
            .iter()
            .any(|a| a.get_desired_room() != desired_room)
        {
            return None;
        }

        let target_hallway = 2 * (desired_room + 1);
        let from = target_hallway.min(hallway_index);
        let to = target_hallway.max(hallway_index);

        if (from..to).any(|i| i != hallway_index && self.hallway[i].is_some()) {
            return None;
        }

        let steps = to - from + self.room_capacity - self.side_rooms[desired_room].len();
        let cost = steps * amphipod.get_cost();

        let mut next_state = self.clone();
        next_state.hallway[hallway_index] = None;
        next_state.side_rooms[desired_room].push(amphipod);

        Some((cost, Rc::new(next_state)))
    }
}

fn solve(init: State) -> usize {
    let mut pqueue = BinaryHeap::new();
    let mut energy_to = HashMap::new();

    let rc_init = Rc::new(init);
    energy_to.insert(rc_init.clone(), 0);
    pqueue.push(Reverse((0, rc_init)));

    while let Some(Reverse((cost, state))) = pqueue.pop() {
        if state.is_final() {
            return cost;
        }

        if cost > *energy_to.get(&state).unwrap_or(&usize::MAX) {
            continue;
        }

        for (extra_cost, next_state) in state.get_next_states() {
            let next_cost = extra_cost + cost;
            if next_cost < *energy_to.get(&next_state).unwrap_or(&usize::MAX) {
                energy_to.insert(next_state.clone(), next_cost);
                pqueue.push(Reverse((next_cost, next_state)));
            }
        }
    }

    panic!("Did not find the solution!");
}

pub fn input_generator(s: &str) -> State {
    s.parse().unwrap()
}

pub fn part1(init: State) -> usize {
    solve(init)
}

pub fn part2(mut init: State) -> usize {
    init.room_capacity = 4;

    init.side_rooms[0].insert(1, Amphipod::Desert);
    init.side_rooms[0].insert(2, Amphipod::Desert);

    init.side_rooms[1].insert(1, Amphipod::Bronze);
    init.side_rooms[1].insert(2, Amphipod::Copper);

    init.side_rooms[2].insert(1, Amphipod::Amber);
    init.side_rooms[2].insert(2, Amphipod::Bronze);

    init.side_rooms[3].insert(1, Amphipod::Copper);
    init.side_rooms[3].insert(2, Amphipod::Amber);

    solve(init)
}
