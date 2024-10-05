use std::collections::{HashSet, VecDeque};

#[cfg(test)]
mod tests;

pub fn aoc11() {
	println!("\nDay 11: Radioisotope Thermoelectric Generators");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
	println!("Part 1:\n{}", part1());
	println!("Part 2:\n{}", part2());
}

/*  */
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct State {
	elevator: u8,
	pairs: Vec<(u8, u8)>,
}

impl State {
	fn sort(&mut self) {
		self.pairs.sort();
	}
	fn is_valid(&self) -> bool {
		self.pairs.iter().all(|&(chip, gen)| {
			chip == gen || !self.pairs.iter().any(|&(_, other_gen)| other_gen == chip)
		})
	}
	fn is_finished(&self, top: u8) -> bool {
		self.pairs.iter().all(|&(c, g)| c == top && g == top)
	}
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
	fn new(pairs: Vec<(u8, u8)>) -> Self {
		let mut init = State {
			elevator: 1,
			pairs: pairs,
		};
		init.sort();
		init
	}
	fn gen(&self, combo: &Vec<(usize, bool)>, dest: u8) -> State {
		let mut new_state = self.clone();
		new_state.elevator = dest;
		combo.iter().for_each(|&(idx, is_chip)| {
			if is_chip {
				new_state.pairs[idx].0 = dest;
			} else {
				new_state.pairs[idx].1 = dest;
			}
		});
		new_state.sort();
		new_state
	}
}

fn bfs(init: State, top: u8) -> usize {
	let mut visited = HashSet::new();
	let mut queue = VecDeque::new();
	queue.push_back((init.clone(), 0));
	visited.insert(init);
	while let Some((state, steps)) = queue.pop_front() {
		if state.is_finished(top) {
			return steps;
		}
		// Posibles direcciones de movimiento.
		let mut directions = Vec::new();
		if state.elevator < top {
			directions.push(state.elevator + 1);
		}
		if state.elevator > 1 {
			directions.push(state.elevator - 1);
		}
		// Obtiene los ítems que se pueden mover (microchips o generadores en el piso actual).
		let items = state.items_on_elevator();
		// Genera todas las combinaciones posibles de 1 o 2 ítems.
		let combos = combinations(items);
		// Para cada dirección y cada combinación, genera el nuevo estado.
		for dest in directions {
			combos.iter().for_each(|combo| {
				let new_state = state.gen(combo, dest);
				if new_state.is_valid() && !visited.contains(&new_state) {
					visited.insert(new_state.clone());
					queue.push_back((new_state, steps + 1));
				}
			});
		}
	}
	unreachable!();
}

fn combinations(items: Vec<(usize, bool)>) -> Vec<Vec<(usize, bool)>> {
	let mut combos = Vec::new();
	for i in 0..items.len() {
		combos.push(vec![items[i]]);
	}
	for i in 0..items.len() {
		for j in (i + 1)..items.len() {
			combos.push(vec![items[i], items[j]]);
		}
	}
	combos
}

pub fn part1() -> usize {
	let state = State::new(vec![(2, 1), (1, 1), (2, 1), (1, 1), (1, 1)]);
	bfs(state, 4)
}

pub fn part2() -> usize {
	let state = State::new(vec![(2, 1), (1, 1), (2, 1), (1, 1), (1, 1), (1, 1), (1, 1)]);
	bfs(state, 4)
}
