#[cfg(test)]
mod tests;

pub fn aoc18() {
	println!("\nDay 18: Like a Rogue");
	println!("━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/18/input.txt")
		.expect("Error reading input file");

	let input = input.trim();
	let (prev, n) = parse_row(input);

	println!("Part 1:\n{}", part1(prev, n));
	println!("Part 2:\n{}", part2(prev, n));
}

fn parse_row(input: &str) -> (u128, u32) {
	let n = input.len() as u32;
	let mut row = 0;
	for (i, c) in input.chars().enumerate() {
		if c == '^' {
			row |= 1 << (n - 1 - (i as u32)); // 1 trap
		}
	}
	(row, n)
}

fn count_safe(mut prev: u128, n: u32, m: usize) -> u32 {
	let mask = (1 << n) - 1; // solo usamos n de u128
	let mut safe = n - prev.count_ones();
	for _ in 1..m {
		// izq XOR der
		prev = ((prev << 1) ^ (prev >> 1)) & mask;
		safe += n - prev.count_ones();
	}
	safe
}

fn part1(prev: u128,n: u32) -> u32 {
	count_safe(prev, n, 40)
}

fn part2(prev: u128, n: u32) -> u32 {
	count_safe(prev, n, 400_000)
}
