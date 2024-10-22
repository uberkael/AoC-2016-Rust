use rayon::prelude::*;

#[cfg(test)]
mod tests;

pub fn aoc09() {
	println!("\nDay 9: Explosives in Cyberspace");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/09/input.txt")
		.expect("Error reading file");

	println!("Part 1:\n{}", part1(&input));
	println!("Part 2:\n{}", part2(&input));
}

fn part1(input: &str) -> usize {
	input
		.lines()
		.collect::<Vec<&str>>()
		.par_iter()
		.map(|line| decompress(line.trim().as_bytes(), false))
		.sum()
}

fn part2(input: &str) -> usize {
	input
		.lines()
		.collect::<Vec<&str>>()
		.par_iter()
		.map(|line| decompress(line.trim().as_bytes(), true))
		.sum()
}

fn decompress(slice: &[u8], recurse: bool) -> usize {
	let mut length = 0;
	let mut index = 0;

	while index < slice.len() {
		if slice[index] == b'(' {
			let (amount, repeat, marker_end) = parse_marker(&slice[index..]);
			let segment_start = index + marker_end;
			let segment_end = segment_start + amount;
			let seg_len = if recurse { // Modo recursivo
				decompress(&slice[segment_start..segment_end], true)
			} else {
				amount
			};
			length += seg_len * repeat;
			index = segment_end;
		} else { // Carácter normal, se incrementa la longitud
			length += 1;
			index += 1;
		}
	}

	length
}

/// Procesa un marcador que siempre comienza con '(' y tiene el formato AxB, por ejemplo "(3x3)".
fn parse_marker(slice: &[u8]) -> (usize, usize, usize) {
	let mut i = 1;
	let mut amount = 0;
	// Lee la cantidad hasta encontrar el delimitador 'x'
	while i < slice.len() && slice[i].is_ascii_digit() {
		amount = amount * 10 + (slice[i] - b'0') as usize;
		i += 1;
	}
	i += 1; // Salta la 'x'
	let mut repeat = 0;
	// Lee el número de repeticiones hasta encontrar el cierre ')'
	while i < slice.len() && slice[i].is_ascii_digit() {
		repeat = repeat * 10 + (slice[i] - b'0') as usize;
		i += 1;
	}
	i += 1; // Salta el ')'
	(amount, repeat, i)
}
