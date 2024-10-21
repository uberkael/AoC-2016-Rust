#[cfg(test)]
mod tests;

pub fn aoc19() {
	println!("\nDay 19: An Elephant Named Joseph");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/19/input.txt")
		.expect("Error reading the file");

	let input = input.trim().parse().expect("Invalid input");

	println!("Part 1:\n{}", part1(input));
	println!("Part 2:\n{}", part2(input));
}

/* 2(n−2^(log_2(n)⌋)+1) */
fn josephus(n: usize) -> usize {
	if n == 1 { return 1; }
	let mut m = 1;
	while 2 * m <= n {
		m *= 2; // 2^⌊log_2(n)⌋
	}
	if n == m {
		m // n potencia de 2
	} else {
		2 * (n - m) + 1 // n entre m y 2m
	}
}

fn part1(input: usize) -> usize {
	josephus(input)
}

/* 3^⌊log_3(n)⌋ si n es potencia de 3
 * n−3^⌊log_3(n)⌋ si n está entre 3^⌊log_3(n)⌋ y 2*3^⌊log_3(n)⌋
 * 2n−3^⌊log_3(n)⌋ si n está entre 2*3^⌊log_3(n)⌋ y 3*3^⌊log_3(n)⌋ */
fn josephus_opposite(n: usize) -> usize {
	if n == 1 { return 1; }
	let mut m = 1;
	while 3 * m <= n {
		m *= 3; // 3^⌊log_3(n)⌋
	}
	if n == m {
		m // n potencia de 3
	} else if n <= m + m {
		n - m // n entre m y 2m
	} else {
		(2 * n) - (3 * m) // n entre 2m y 3m
	}
}

fn part2(input: usize) -> usize {
	josephus_opposite(input)
}
