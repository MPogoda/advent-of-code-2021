use anyhow::Result;
use aoc_next::{aoc_main, parser, solution, solver, Aoc};

use rust_advent_of_code_2021::{
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
};

const AOC: Aoc = Aoc {
    allow_download: true,
    year: 2021,
    solutions: &[
        solution! { 1, parser!{ day01::input_generator }, solver!{ day01::part1 }},
        solution! { 1, parser!{ day01::input_generator }, solver!{ day01::part2 }},
        solution! { 2, parser!{ day02::input_generator }, solver!{ day02::part1 }},
        solution! { 2, parser!{ day02::input_generator }, solver!{ day02::part2 }},
        solution! { 3, parser!{ day03::input_generator }, solver!{ day03::part1 }},
        solution! { 3, parser!{ day03::input_generator }, solver!{ day03::part2 }},
        solution! { 4, parser!{ day04::input_generator }, solver!{ day04::part1 }},
        solution! { 4, parser!{ day04::input_generator }, solver!{ day04::part2 }},
        solution! { 5, parser!{ day05::input_generator }, solver!{ day05::part1 }},
        solution! { 5, parser!{ day05::input_generator }, solver!{ day05::part2 }},
        solution! { 6, parser!{ day06::input_generator }, solver!{ day06::part1 }},
        solution! { 6, parser!{ day06::input_generator }, solver!{ day06::part2 }},
        solution! { 7, parser!{ day07::input_generator }, solver!{ day07::part1 }},
        solution! { 7, parser!{ day07::input_generator }, solver!{ day07::part2 }},
        solution! { 8, parser!{ day08::input_generator }, solver!{ day08::part1 }},
        solution! { 8, parser!{ day08::input_generator }, solver!{ day08::part2 }},
        solution! { 9, parser!{ day09::input_generator }, solver!{ day09::part1 }},
        solution! { 9, parser!{ day09::input_generator }, solver!{ day09::part2 }},
        solution! { 10, parser!{ day10::input_generator }, solver!{ day10::part1 }},
        solution! { 10, parser!{ day10::input_generator }, solver!{ day10::part2 }},
    ],
};

pub fn main() -> Result<()> {
    aoc_main(AOC)
}
