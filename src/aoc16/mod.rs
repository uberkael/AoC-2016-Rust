use rayon::prelude::*;

#[cfg(test)]
mod tests;

pub fn aoc16() {
	println!("\nDay 16: Dragon Checksum");
	println!("━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/16/input.txt").unwrap();

	println!("Part 1:\n{}", part1(&input));
	println!("Part 2:\n{}", part2(&input));
}

type BitVec = Vec<bool>;

fn duplicate(a: &BitVec) -> BitVec {
	let mut new_data = a.clone();
	new_data.push(false); // Añade el 0 central (false)
	// Añade los bits invertidos en orden inverso de forma paralela
	let inverted: Vec<bool> = a
		.par_iter()
		.rev()
		.map(|&b| !b)
		.collect();
	new_data.extend(inverted);
	new_data
}

fn checksum(data: &BitVec) -> BitVec {
	data.par_chunks(2)
		.map(|pair| pair[0] == pair[1])
		.collect()
}

fn valid_checksum(mut data: BitVec) -> BitVec {
	while data.len() % 2 == 0 {
		data = checksum(&data);
	}
	data
}

fn process(initial_data: &str, size: usize) -> BitVec {
	let mut data: BitVec = initial_data
		.trim()
		.as_bytes()
		.par_iter()
		.map(|&c| c == 0b1)
		.collect();
	while data.len() < size {
		data = duplicate(&data);
	}
	data.truncate(size);
	valid_checksum(data)
}

fn stringify(data: &BitVec) -> String {
	data.par_iter()
		.map(|&b| if b { '1' } else { '0' })
		.collect()
}

fn part1(input: &str) -> String {
	let input = input.trim();
	let result = process(input, 272);
	stringify(&result)
}

fn part2(input: &str) -> String {
	let input = input.trim();
	let result = process(input, 35651584);
	stringify(&result)
}
