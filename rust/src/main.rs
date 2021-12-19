use anyhow::Result;
use aoc_next::{aoc_main, parser, solution, solver, Aoc};

use rust_advent_of_code_2021::{
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19,
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
        solution! { 11, parser!{ day11::input_generator }, solver!{ day11::part1 }},
        solution! { 11, parser!{ day11::input_generator }, solver!{ day11::part2 }},
        solution! { 12, parser!{ day12::input_generator }, solver!{ day12::part1 }},
        solution! { 12, parser!{ day12::input_generator }, solver!{ day12::part2 }},
        solution! { 13, parser!{ day13::input_generator }, solver!{ day13::part1 }},
        solution! { 13, parser!{ day13::input_generator }, solver!{ day13::part2 }},
        solution! { 14, parser!{ day14::input_generator }, solver!{ day14::part1 }},
        solution! { 14, parser!{ day14::input_generator }, solver!{ day14::part2 }},
        solution! { 15, parser!{ day15::input_generator }, solver!{ day15::part1 }},
        solution! { 15, parser!{ day15::input_generator }, solver!{ day15::part2 }},
        solution! { 16, parser!{ day16::input_generator }, solver!{ day16::part1 }},
        solution! { 16, parser!{ day16::input_generator }, solver!{ day16::part2 }},
        solution! { 17, parser!{ day17::input_generator }, solver!{ day17::part1 }},
        solution! { 17, parser!{ day17::input_generator }, solver!{ day17::part2 }},
        solution! { 18, parser!{ day18::input_generator }, solver!{ day18::part1 }},
        solution! { 18, parser!{ day18::input_generator }, solver!{ day18::part2 }},
        solution! { 19, parser!{ day19::input_generator }, solver!{ day19::part1 }},
        solution! { 19, parser!{ day19::input_generator }, solver!{ day19::part2 }},
    ],
};

pub fn main() -> Result<()> {
    aoc_main(AOC)
}
