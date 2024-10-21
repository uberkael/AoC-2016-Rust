use num::integer::lcm;

#[cfg(test)]
mod tests;

pub fn aoc15() {
	println!("\nDay 15: Timing is Everything");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/15/input.txt")
		.expect("Error reading the file");

	println!("Part 1:\n{}", part1(&input));
	println!("Part 2:\n{}", part2(&input));
}

struct Disc {
	positions: usize,
	initial: usize,
}

impl Disc {
	fn new(positions: usize, initial: usize) -> Self {
		Self { positions, initial }
	}
}

fn parse_input(input: &str) -> Vec<Disc> {
	input.lines().map(|line| {
		let parts: Vec<_> = line[0..line.len() - 1]
			.split_whitespace()
			.filter_map(|s| s.parse().ok())
			.collect();
		Disc::new(parts[0], parts[1])
		}).collect()
}

/* Part1 */
fn part1(input: &str) -> usize {
	/* Chinese Remainder Theorem */
	let discs = parse_input(input);
	let mut time = 0;
	let mut step = 1;
	for (i, disc) in discs.iter().enumerate() {
		let num_positions = disc.positions;
		let remainder = (num_positions - ((disc.initial + i + 1) % num_positions)) % num_positions;
		while time % num_positions != remainder {
			time += step;
		}
		step = lcm(step, num_positions);
	}
	time
}

/* Part2 */
fn part2(input: &str) -> usize {
	let discs: Vec<Disc> = parse_input(&input).into_iter().chain(vec![Disc::new(11, 0)]).collect();
	let mut time = 0;
	let mut step = 1;
	for (i, disc) in discs.iter().enumerate() {
		let num_positions = disc.positions;
		let remainder = (num_positions - ((disc.initial + i + 1) % num_positions)) % num_positions;
		while time % num_positions != remainder {
			time += step;
		}
		step = lcm(step, num_positions);
	}
	time
}
