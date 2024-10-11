#![allow(dead_code)]

#[cfg(test)]
mod tests;

pub fn aoc19() {
	println!("\nDay 19: An Elephant Named Joseph");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/19/input.txt").unwrap();

	println!("Part 1:\n{}", part1(&input));
}

/* 2(n−2^(log_2(n)⌋)+1) */
fn josephus(input: usize) -> usize {
	// Encontrar la mayor potencia de 2 menor o igual a input
	let mut largest_power = 1;
	while largest_power * 2 <= input {
		largest_power *= 2;
	}
	// Aplicar la fórmula del problema de Josephus
	let l = input - largest_power;
	if l == 0 {
		largest_power
	} else {
		2 * l + 1
	}
}

fn part1(input: &str) -> usize {
	let input = input.trim().parse().unwrap();
	josephus(input)
}
