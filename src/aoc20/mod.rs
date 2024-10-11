#![allow(dead_code)]

#[cfg(test)]
mod tests;

pub fn aoc20() {
	println!("\nDay 20: Firewall Rules");
	println!("━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/20/input.txt").unwrap();

	println!("Part 1:\n{}", part1(&input));
	println!("Part 2:\n{}", part2(&input));
}
