extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

#[macro_use]
extern crate scan_fmt;

pub mod d01; // rocket fuel
pub mod d02; // intcode v1
pub mod d03; // crossed wires
pub mod d04; // password rules
pub mod d05; // intcode upgrades, realtive and input
pub mod d06; // counting orbits
pub mod d07; // intcode, phase amplifier circuit
pub mod d08; // image layering

aoc_lib! { year = 2019 }
