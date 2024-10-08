// #![allow(dead_code)]

#[cfg(test)]
mod tests;

pub fn aoc16() {
	println!("\nDay 16: Dragon Checksum");
	println!("━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/16/input.txt").unwrap();

	println!("Part 1:\n{:b}", part1(&input));

}

fn duplicate(a: usize, len_a: usize) -> (usize, usize) {
	let inverted = invert_reverse(a, len_a);
	let new_len = 2 * len_a + 1;
	let new_data = (a << (len_a + 1)) | (0 << len_a) | inverted;
	(new_data, new_len)
}

// Invierte y revierte los bits considerando la longitud
fn invert_reverse(data: usize, len: usize) -> usize {
	let mut reversed = 0;
	for i in 0..len {
		let bit = (data >> (len - 1 - i)) & 1; // Leer desde MSB
		reversed |= (!bit & 1) << i; // Invertir y colocar en LSB
	}
	reversed
}

// Calcula el checksum usando la longitud real de los datos
fn checksum(data: usize, len: usize) -> (usize, usize) {
	let mut sum = 0;
	let mut new_len = 0;
	for i in (0..len).step_by(2) {
		if i + 1 >= len {
			break;
		}
		let bit1 = (data >> (len - 1 - i)) & 1;
		let bit2 = (data >> (len - 1 - (i + 1))) & 1;
		sum = (sum << 1) | if bit1 == bit2 { 1 } else { 0 };
		new_len += 1;
	}
	(sum, new_len)
}

fn valid_checksum(mut data: usize, mut len: usize) -> usize {
	while len % 2 == 0 {
		let (new_data, new_len) = checksum(data, len);
		data = new_data;
		len = new_len;
	}
	data
}

fn process(initial_data: usize, initial_len: usize, size: usize) -> usize {
	let mut data = initial_data;
	let mut len = initial_len;
	// Duplicar hasta alcanzar o superar la longitud deseada
	while len < size {
		let (new_data, new_len) = duplicate(data, len);
		data = new_data;
		len = new_len;
	}
	// Recortar a la longitud deseada (bits más significativos)
	let shift = len - size;
	data >>= shift;
	len = size;
	// Calcular checksum válido
	valid_checksum(data, len)
}

fn part1(input: &str) -> usize {
	let data = usize::from_str_radix(input.trim(), 2).unwrap();
	let result = process(data, input.len(), 272);
	// println!("Resultado: {:0width$b}", result, width = 20);
	result
}
