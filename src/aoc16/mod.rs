// #![allow(dead_code)]

#[cfg(test)]
mod tests;

pub fn aoc16() {
	println!("\nDay 16: Dragon Checksum");
	println!("━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/16/input.txt").unwrap();

	println!("Part 1:\n{}", part1(&input));
}

type BitVec = Vec<u8>;

fn duplicate(a: &BitVec) -> BitVec {
	let mut new_data = a.clone();
	new_data.push(0); // Añade el 0 central
	// Añade los bits invertidos en orden inverso
	new_data.extend(a.iter().rev().map(|&b| 1 - b));
	new_data
}

fn checksum(data: &BitVec) -> BitVec {
	data.chunks(2)
		.map(|pair| if pair[0] == pair[1] { 1 } else { 0 })
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
		.chars()
		.map(|c| if c == '1' { 1 } else { 0 })
		.collect();
	while data.len() < size {
		data = duplicate(&data);
	}
	data.truncate(size); // trim size
	valid_checksum(data)
}

fn stringify(data: &BitVec) -> String {
	data.iter()
		.map(|&b| if b == 1 { '1' } else { '0' })
		.collect()
}

fn part1(input: &str) -> String {
	let input = input.trim();
	let result = process(input, 272);
	stringify(&result)
}
