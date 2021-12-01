use anyhow::Result;
use aoc_next::{aoc_main, parser, solution, solver, Aoc};

use rust_advent_of_code_2021::day01;

const AOC: Aoc = Aoc {
    allow_download: true,
    year: 2021,
    solutions: &[
        solution! { 1, parser!{ day01::input_generator }, solver!{ day01::part1 }},
        solution! { 1, parser!{ day01::input_generator }, solver!{ day01::part2 }},
    ],
};

pub fn main() -> Result<()> {
    aoc_main(AOC)
}
