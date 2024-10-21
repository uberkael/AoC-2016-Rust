#[cfg(test)]
mod tests;

pub fn aoc16() {
	println!("\nDay 16: Dragon Checksum");
	println!("━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/16/input.txt")
		.expect("Error reading file");

	println!("Part 1:\n{}", part1(&input));
	println!("Part 2:\n{}", part2(&input));
}

/// Calculate checksum
fn process(initial_data: &str, size: usize) -> String {
	// Base, '1' -> true.
	let base: Vec<bool> = initial_data.trim().bytes().map(|b| b == b'1').collect();
	let base_len = base.len();

	// Pre-calcular prefijo sumado de la cadena base.
	let mut prefix = vec![0; base_len + 1];
	for i in 0..base_len {
		prefix[i + 1] = prefix[i] + (base[i] as usize);
	}

	// niveles[0] = base_len, y para k>=1: niveles[k] = 2 * niveles[k-1] + 1.
	let mut levels = vec![base_len];
	while *levels.last().expect("Empty levels") < size {
		levels.push(2 * *levels.last().expect("Empty levels") + 1);
	}
	let level = levels.len() - 1; // Nivel suficiente

	// Determinar el tamaño del bloque: la mayor potencia de dos que divide 'size'
	let block_size = size & (!(size - 1));
	let n_blocks = size / block_size;

	let mut checksum = String::with_capacity(n_blocks);
	for block in 0..n_blocks {
		let start = block * block_size;
		let end = start + block_size;
		// Número de unos en el bloque = count_ones(end) - count_ones(start)
		let ones = count_ones(&levels, &base, &prefix, level, end)
			- count_ones(&levels, &base, &prefix, level, start);
		// Si el número de unos es par, el bit del checksum es '1', de lo contrario '0'.
		checksum.push(if ones % 2 == 0 { '1' } else { '0' });
	}
	checksum
}

/// Función recursiva para calcular el número de unos en la secuencia dragon
/// para un prefijo de longitud `len`.
/// `levels` contiene la longitud de Sₖ para cada nivel k,
/// `base` es la cadena base
/// `prefix` su vector de prefijo sumado.
fn count_ones(levels: &Vec<usize>, base: &[bool], prefix: &[usize], k: usize, len: usize) -> usize {
	if k == 0 {
		// En nivel 0, la respuesta se obtiene directamente del prefijo sumado.
		return prefix[len];
	}
	let n = levels[k - 1]; // Longitud de Sₖ₋₁ (la parte A)
	if len <= n {
		count_ones(levels, base, prefix, k - 1, len)
	} else if len == n + 1 {
		// Se incluye todo A + el 0 central.
		count_ones(levels, base, prefix, k - 1, n)
	} else {
		// len > n + 1: parte del bloque B.
		let r = len - n - 1;
		// B es la inversión de A leído al revés, de modo que:
		// ones(B[0..r]) = r - (ones de A en [n-r, n))
		// Total en Sₖ[0..len) = ones(A) + ones(B[0..r]) = r + count_ones(k-1, n - r)
		r + count_ones(levels, base, prefix, k - 1, n - r)
	}
}

fn part1(input: &str) -> String {
	process(input, 272)
}

fn part2(input: &str) -> String {
	process(input, 35651584)
}
