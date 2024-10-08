#![allow(dead_code)]

// use std::collections::HashMap;
// use rayon::prelude::*;

#[cfg(test)]
mod tests;

pub fn aoc15() {
	println!("\nDay 15: Timing is Everything");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/15/input.txt").unwrap();

	println!("Part 1:\n{}", part1(&input));
}

struct Disc {
	positions: usize,
	initial: usize,
}

impl Disc {
	fn new(positions: usize, initial: usize) -> Self {
		Self { positions, initial }
	}
	fn position(&self, time: usize) -> usize {
		(self.initial + time) % self.positions
	}
}

fn parse_input(input: &str) -> Vec<Disc> {
	input .lines().map(|line| {
			let parts: Vec<_> = line[0..line.len() - 1]
				.split_whitespace()
				.filter_map(|s| s.parse().ok())
				.collect();
			Disc::new(parts[0], parts[1])
		}).collect()
}

/* Part1 */
fn part1(input: &str) -> usize {
	let discs = parse_input(input);
	let mut align = 0;
	let mut time = 0;
	loop {
		discs.iter().enumerate().for_each(|(i, d)| {
			if d.position(time + i + 1) == 0 {
				align += 1;
			}
		});
		if align == discs.len() {
			return time;
		}
		align = 0;
		time += 1;
	}
}
