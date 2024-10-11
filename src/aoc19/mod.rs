#![allow(dead_code)]

#[cfg(test)]
mod tests;

pub fn aoc19() {
	println!("\nDay 19: An Elephant Named Joseph");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/19/input.txt").unwrap();

	println!("Part 1:\n{}", part1(&input));
	println!("Part 2:\n{}", part2(&input));
}

/* 2(n−2^(log_2(n)⌋)+1) */
fn josephus(n: usize) -> usize {
	let z = (n as f64).log2().floor();
	2 * (n - 2usize.pow(z as u32)) + 1
}

fn part1(input: &str) -> usize {
	let input = input.trim().parse().unwrap();
	josephus(input)
}

/* 3^⌊log_3(n)⌋ si n es potencia de 3
 * n−3^⌊log_3(n)⌋ si n está entre 3^⌊log_3(n)⌋ y 2*3^⌊log_3(n)⌋
 * 2n−3^⌊log_3(n)⌋ si n está entre 2*3^⌊log_3(n)⌋ y 3*3^⌊log_3(n)⌋ */
fn josephus_opposite(n: usize) -> usize {
	let z = (n as f64).log(3.0).floor() as u32;
	if n == 3usize.pow(z) {
		3usize.pow(z)
	} else if n <= 2 * 3usize.pow(z) {
		n - 3usize.pow(z)
	} else {
		(2 * n) - 3usize.pow(z)
	}
}

fn part2(input: &str) -> usize {
	let n: usize = input.trim().parse().unwrap();
	josephus_opposite(n)
}
