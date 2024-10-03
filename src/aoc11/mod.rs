use std::collections::{HashMap, VecDeque};
#[cfg(test)]
mod tests;

pub fn aoc11() {
	println!("\nDay 11: Radioisotope Thermoelectric Generators");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

	// let input = std::fs::read_to_string("input/11/input.txt").unwrap();

	/* Part 1 */
	let state = vec![
		vec!["E", "PG", "TG", "TM", "AG", "RG", "RM", "CG", "CM"],
		vec!["PM", "AM"],
		vec![],
		vec![]
	];
	println!("{}", part1(state));
}

/* Part 1 */
fn part1(initial_state: Vec<Vec<&'static str>>) -> isize {
	let mut visited = HashMap::new();
	let mut queue = VecDeque::new();
	queue.push_back((initial_state.clone(), 0));
	visited.insert(encode_state(&initial_state), true);
	while let Some(entry) = queue.pop_front() {
		let (current_state, steps) = entry;
		if finished(&current_state) {
			return steps;
		}
		let next_states = generate_states(&current_state);
		for next_state in next_states {
			let encoded = encode_state(&next_state);
			if !visited.contains_key(&encoded) {
				visited.insert(encoded, true);
				queue.push_back((next_state, steps + 1));
			}
		}
	}
	-1
}

fn encode_state(state: &[Vec<&'static str>]) -> String {
	let mut encoded = String::new();
	for floor in state {
		let mut items: Vec<&str> = floor.iter().copied().collect();
		items.sort_unstable();
		encoded.push_str(&format!("{}:", items.join(",")));
	}
	encoded
}

fn find_elevator(state: &Vec<Vec<&'static str>>) -> usize {
	for (i, floor) in state.iter().enumerate() {
		if floor.get(0) == Some(&"E") {
			return i;
		}
	}
	usize::MAX
}

fn finished(state: &Vec<Vec<&'static str>>) -> bool {
	// print_state(state);
	for i in 0..state.len() - 1 {
		if !state[i].is_empty() {
			return false;
		}
	}
	true
}

fn generate_floors(f1: &Vec<&'static str>, f2: &Vec<&'static str>) -> Vec<[Vec<&'static str>; 2]> {
	let mut valid_pairs = Vec::new();
	let (source, destination, backward) = match (f1.first(), f2.first()) {
		(Some(&"E"), _) => (&f1[1..].to_vec(), f2, false), // Mover hacia abajo
		(_, Some(&"E")) => (&f2[1..].to_vec(), f1, true),  // Mover hacia arriba
		_ => panic!("E debe estar en f1[0] o f2[0]"),
	};
	// Generar combinaciones de 1 elemento
	for i in 0..source.len() {
		let to_move = vec![source[i]];
		// Crear nuevo source excluyendo el elemento movido
		let new_source: Vec<&str> = source
			.iter()
			.enumerate()
			.filter(|&(idx, _)| idx != i)
			.map(|(_, &elem)| elem)
			.collect();
		// Crear nuevo destination incluyendo el elemento movido
		let mut new_destination = vec!["E"];
		new_destination.extend(destination);
		new_destination.extend(to_move);
		// Verificar validez de ambos pisos
		if is_valid(&new_source) && is_valid(&new_destination) {
			if backward {
				valid_pairs.push([new_destination, new_source]);
			} else {
				valid_pairs.push([new_source, new_destination]);
			}
		}
	}
	if backward {
		return valid_pairs;
	}
	// Generar combinaciones de 2 elementos
	for i in 0..source.len() {
		for j in (i + 1)..source.len() {
			let to_move = vec![source[i], source[j]];
			// Crear nuevo source excluyendo los elementos movidos
			let new_source: Vec<&str> = source
				.iter()
				.enumerate()
				.filter(|&(idx, _)| idx != i && idx != j)
				.map(|(_, &elem)| elem)
				.collect();
			// Crear nuevo destination incluyendo los elementos movidos
			let mut new_destination = vec!["E"];
			new_destination.extend_from_slice(destination);
			new_destination.extend(to_move);
			// Verificar validez de ambos pisos
			if is_valid(&new_source) && is_valid(&new_destination) {
				if backward {
					valid_pairs.push([new_destination, new_source]);
				} else {
					valid_pairs.push([new_source, new_destination]);
				}
			}
		}
	}
	valid_pairs
}

fn is_valid(floor: &Vec<&'static str>) -> bool {
	// Si el piso tiene solo un elemento, es válido
	if floor.len() == 1 {
		return true;
	}
	// Si no hay ningún generador (G) en el piso, es válido
	if !floor.iter().any(|&element| element.ends_with('G')) {
		return true;
	}
	// Verificar que todo M tenga su G correspondiente
	for &element in floor {
		if element.ends_with('M') {
			let prefix = element.chars().next().unwrap();
			let g = format!("{}G", prefix);
			if !floor.contains(&&*g) {
				return false; // Si no está el G correspondiente, el piso no es válido
			}
		}
	}
	true // Si todos los M tienen su G, el piso es válido
}

fn generate_states(state: &Vec<Vec<&'static str>>) -> Vec<Vec<Vec<&'static str>>> {
	let elevator = find_elevator(state);
	let mut new_states = Vec::new();
	if elevator == 0 {
		let pairs = generate_floors(&state[elevator], &state[elevator + 1]);
		for pair in pairs {
			let [a, b] = pair;
			let mut new_state = state.clone();
			new_state[elevator] = a;
			new_state[elevator + 1] = b;
			new_states.push(new_state);
		}
	} else if elevator == state.len() - 1 {
		let pairs = generate_floors(&state[elevator], &state[elevator - 1]);
		for pair in pairs {
			let [a, b] = pair;
			let mut new_state = state.clone();
			new_state[elevator] = a;
			new_state[elevator - 1] = b;
			new_states.push(new_state);
		}
	} else {
		let pairs1 = generate_floors(&state[elevator], &state[elevator + 1]);
		let pairs2 = generate_floors(&state[elevator], &state[elevator - 1]);
		for pair in pairs1 {
			let [a, b] = pair;
			let mut new_state = state.clone();
			new_state[elevator] = a;
			new_state[elevator + 1] = b;
			new_states.push(new_state);
		}
		for pair in pairs2 {
			let [a, b] = pair;
			let mut new_state = state.clone();
			new_state[elevator] = a;
			new_state[elevator - 1] = b;
			new_states.push(new_state);
		}
	}
	new_states
}

fn print_floor(floor: &Vec<&'static str>) {
	for f in floor {
		print!("{}, ", f);
	}
	// flush
	std::io::Write::flush(&mut std::io::stdout()).unwrap();
}

fn print_state(state: &Vec<Vec<&'static str>>) {
	for (i, floor) in state.iter().enumerate().rev() {
		print!("F{} - ", i + 1);
		print_floor(floor);
		println!();
	}
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}
