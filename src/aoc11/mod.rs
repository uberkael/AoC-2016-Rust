use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::Mutex;

/// Cada elemento se representa como un par: (piso_del_microchip, piso_del_generador)
/// Los pisos se numeran, por ejemplo, del 1 al 4.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct State {
	elevator: u8,
	pairs: Vec<(u8, u8)>, // Por ejemplo, (2, 1) significa: microchip en piso 2, generador en piso 1.
}

impl State {
	/// Ordena los pares para que la representación sea canónica.
	fn canonicalize(&mut self) {
		self.pairs.sort();
	}

	/// Verifica que el estado sea seguro.
	/// Para cada microchip, si éste no está junto a su generador,
	/// se debe asegurar que no haya ningún otro generador en el mismo piso.
	fn is_valid(&self) -> bool {
		for &(chip, gen) in &self.pairs {
			if chip != gen {
				// Si hay algún generador en el piso del microchip que no es el suyo,
				// el estado no es seguro.
				if self.pairs.iter().any(|&(_, other_gen)| other_gen == chip) {
					return false;
				}
			}
		}
		true
	}

	/// Verifica si se llegó a la solución: todos los objetos (microchips y generadores)
	/// deben estar en el piso superior.
	fn is_finished(&self, top: u8) -> bool {
		self.pairs
			.iter()
			.all(|&(chip, gen)| chip == top && gen == top)
	}

	/// Retorna los ítems que se pueden mover desde el piso del elevador.
	/// Cada ítem se identifica por: (índice_del_par, es_microchip).
	fn items_on_elevator(&self) -> Vec<(usize, bool)> {
		let mut items = Vec::new();
		for (i, &(chip, gen)) in self.pairs.iter().enumerate() {
			if chip == self.elevator {
				items.push((i, true));
			}
			if gen == self.elevator {
				items.push((i, false));
			}
		}
		items
	}
}

/// Genera los estados hijos a partir del estado actual.
/// Se consideran movimientos hacia arriba (si no se está en el piso superior)
/// y hacia abajo (si no se está en el piso inferior), moviendo 1 o 2 ítems a la vez.
fn generate_children(state: &State, top: u8) -> Vec<State> {
	let mut children = Vec::new();
	// Se obtienen los ítems que se pueden mover.
	let items = state.items_on_elevator();
	// Generar todas las combinaciones posibles de 1 y 2 ítems.
	let mut combos = Vec::new();
	for i in 0..items.len() {
		combos.push(vec![items[i]]);
	}
	for i in 0..items.len() {
		for j in (i + 1)..items.len() {
			combos.push(vec![items[i], items[j]]);
		}
	}
	// Direcciones de movimiento posibles.
	let mut directions = Vec::new();
	if state.elevator < top {
		directions.push(state.elevator + 1);
	}
	if state.elevator > 1 {
		directions.push(state.elevator - 1);
	}
	// Para cada dirección y cada combinación, se genera el nuevo estado.
	for &dest in &directions {
		for combo in &combos {
			let mut new_state = state.clone();
			new_state.elevator = dest;
			for &(idx, is_chip) in combo {
				if is_chip {
					new_state.pairs[idx].0 = dest;
				} else {
					new_state.pairs[idx].1 = dest;
				}
			}
			new_state.canonicalize();
			children.push(new_state);
		}
	}
	children
}

/// Realiza la búsqueda BFS paralela con Rayon.
fn bfs_parallel(init: State, top: u8) -> usize {
	// Conjunto de estados visitados.
	let visited = Mutex::new(HashSet::new());
	{
		let mut vis = visited.lock().unwrap();
		vis.insert(init.clone());
	}
	// "Frente" de la búsqueda BFS (nivel actual).
	let mut current = vec![init];
	let mut steps = 0;
	while !current.is_empty() {
		// Si algún estado del nivel actual es solución, retornamos la cantidad de pasos.
		if current.iter().any(|s| s.is_finished(top)) {
			return steps;
		}
		// Se generan, en paralelo, los estados hijos de cada estado del nivel actual.
		let next: Vec<State> = current
			.par_iter()
			.flat_map(|state| {
				generate_children(state, top)
					.into_iter()
					.filter(|child| child.is_valid())
					.filter_map(|child| {
						// Se sincroniza el acceso al conjunto de visitados.
						let mut vis = visited.lock().unwrap();
						if !vis.contains(&child) {
							vis.insert(child.clone());
							Some(child)
						} else {
							None
						}
					})
					.collect::<Vec<_>>()
			})
			.collect();
		current = next;
		steps += 1;
	}
	unreachable!("Siempre existe una solución");
}

fn initialize_state(pairs: Vec<(u8, u8)>) -> State {
	let mut init = State {
		elevator: 1,
		pairs,
	};
	init.canonicalize();
	init
}

pub fn part1() -> usize {
	let pairs = vec![(2, 1), (1, 1), (2, 1), (1, 1), (1, 1)];
	let init = initialize_state(pairs);
	bfs_parallel(init, 4)
}

pub fn part2() -> usize {
	let pairs = vec![(2, 1), (1, 1), (2, 1), (1, 1), (1, 1), (1, 1), (1, 1)];
	let init = initialize_state(pairs);
	bfs_parallel(init, 4)
}

pub fn aoc11() {
	println!("\nDay 11: Radioisotope Thermoelectric Generators");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
	println!("{}", part1());
	println!("{}", part2());
}
