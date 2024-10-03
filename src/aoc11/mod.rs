use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use rayon::prelude::*;

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

	/* Part 2 */
	// let state = vec![
	// 	vec!["E", "PG", "TG", "TM", "AG", "RG", "RM", "CG", "CM", "EG", "EM", "DG", "DM"],
	// 	vec!["PM", "AM"],
	// 	vec![],
	// 	vec![]
	// ];
	// println!("{}", part1(state));
}

/* Part 1 */
pub fn part1(initial_state: Vec<Vec<&'static str>>) -> isize {
	let visited = std::sync::Mutex::new(HashSet::new());
	let mut queue = VecDeque::new();
	queue.push_back((initial_state.clone(), 0));
	visited.lock().unwrap().insert(encode_state(&initial_state));
	while let Some((current, steps)) = queue.pop_front() {
		if finished(&current) {
			return steps;
		}
		// Procesar estados hijos en paralelo
		let children: Vec<_> = generate_states(&current)
			.par_iter()
			.filter_map(|state| {
				let encoded = encode_state(state);
				if visited.lock().unwrap().insert(encoded) {
					Some(state.clone())
				} else {
					None
				}
			})
			.collect();
		// Agregar al queue secuencialmente
		for child in children {
			queue.push_back((child, steps + 1));
		}
	}
	-1
}

fn encode_state(state: &[Vec<&'static str>]) -> u64 {
    let mut hasher = DefaultHasher::new();
    for floor in state {
        // Creamos una copia de los elementos del piso y los ordenamos para
        // obtener una representación canónica independientemente del orden.
        let mut items: Vec<&str> = floor.iter().copied().collect();
        items.sort_unstable();
        // Concatena los elementos del piso en el hasher.
        items.hash(&mut hasher);
        // Incorpora un delimitador (por ejemplo, un número fijo) para separar cada piso.
        0xdead_beefu64.hash(&mut hasher);
    }
    hasher.finish()
}

fn find_elevator(state: &Vec<Vec<&'static str>>) -> usize {
	state
		.iter()
		.position(|floor| floor.get(0) == Some(&"E"))
		.unwrap_or(usize::MAX)
}

fn finished(state: &Vec<Vec<&'static str>>) -> bool {
    // Se asume que todos los objetos en los pisos inferiores al último deben haber sido trasladados.
    state.iter().take(state.len() - 1).all(|floor| floor.len() <= 1)
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
