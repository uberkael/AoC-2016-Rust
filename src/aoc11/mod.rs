use std::collections::{HashSet, VecDeque};

#[cfg(test)]
mod tests;

pub fn aoc11() {
	println!("\nDay 11: Radioisotope Thermoelectric Generators");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
	println!("Part 1:\n{}", part1());
	println!("Part 2:\n{}", part2());
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Floor {
	gens: u8,
	chips: u8,
}

impl Floor {
	#[inline]
	fn new(gens: u8, chips: u8) -> Self {
		Floor { gens, chips }
	}
	#[inline]
	fn valid(&self) -> bool {
		self.gens == 0 || self.gens >= self.chips
	}
	#[inline]
	fn total(&self) -> u8 {
		self.gens + self.chips
	}
	#[inline]
	fn add(&self, other: &Floor) -> Floor {
		Floor::new(self.gens + other.gens, self.chips + other.chips)
	}
	#[inline]
	fn sub(&self, other: &Floor) -> Floor {
		Floor::new(self.gens - other.gens, self.chips - other.chips)
	}
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct State {
	elevator: u8,
	floors: [Floor; 4],
}

impl State {
	fn new(pairs: Vec<(u8, u8)>) -> Self {
		let mut floors = [Floor::new(0, 0); 4];
		for (chip, gen) in pairs {
			// Restamos 1 para usar índices 0-based.
			floors[(chip - 1) as usize].chips += 1;
			floors[(gen - 1) as usize].gens += 1;
		}
		State {
			elevator: 1,
			floors,
		}
	}
	fn is_valid(&self) -> bool {
		self.floors.iter().all(|f| f.valid())
	}
}

const MOVES: [Floor; 5] = [ // Posibles movimientos
	Floor { gens: 2, chips: 0 },
	Floor { gens: 1, chips: 1 },
	Floor { gens: 0, chips: 2 },
	Floor { gens: 1, chips: 0 },
	Floor { gens: 0, chips: 1 },
];

fn bfs(init: State) -> usize {
	let complete: u8 = init.floors.iter().map(|f| f.total()).sum();
	let mut visited = HashSet::new();
	let mut queue = VecDeque::new();
	queue.push_back((init, 0));
	visited.insert(init);
	while let Some((state, steps)) = queue.pop_front() {
		if state.floors[3].total() == complete {
			// Todos los items están en el último piso, retornar el número de pasos
			return steps;
		}
		let cur = (state.elevator - 1) as usize;
		if state.elevator > 1 { // DOWN
			let can_move_down = (0..cur).any(|i| state.floors[i].total() > 0);
			if can_move_down {
				let dest = state.elevator - 1;
				let dest_idx = (dest - 1) as usize;
				let mut min_moved = 3; // Máximo 2
				for m in MOVES.iter().rev() {
					if m.total() > min_moved { break; }
					if cant_move(cur, m, state) { continue; } // Not enough items
					let mut next = state.clone();
					next.elevator = dest;
					next.floors[cur] = next.floors[cur].sub(m);
					next.floors[dest_idx] = next.floors[dest_idx].add(m);
					if not_valid(cur, dest_idx, next) { continue; }
					if visited.insert(next.clone()) {
						min_moved = m.total();
						queue.push_back((next, steps + 1));
					}
				}
			}
		}
		if state.elevator < 4 { // UP
			let dest = state.elevator + 1;
			let dest_idx = (dest - 1) as usize;
			let mut max_moved = 0;
			for m in MOVES.iter() {
				if m.total() < max_moved { break; }
				if cant_move(cur, m, state) { continue; } // Not enough items
				let mut next = state.clone();
				next.elevator = dest;
				next.floors[cur] = next.floors[cur].sub(m);
				next.floors[dest_idx] = next.floors[dest_idx].add(m);
				if not_valid(cur, dest_idx, next) { continue; } // Estado inválido, saltar
				if visited.insert(next.clone()) {
					max_moved = m.total();
					queue.push_back((next, steps + 1));
				}
			}
		}
	}
	panic!("No solution found");
}

#[inline]
fn cant_move(cur: usize, m: &Floor, state: State) -> bool {
		state.floors[cur].gens < m.gens || state.floors[cur].chips < m.chips
}

#[inline]
fn not_valid(cur: usize, dest_idx: usize, next: State) -> bool {
		!next.floors[cur].valid() || !next.floors[dest_idx].valid() || !next.is_valid()
}

pub fn part1() -> usize {
	let state = State::new(vec![(2, 1), (1, 1), (2, 1), (1, 1), (1, 1)]);
	bfs(state)
}

pub fn part2() -> usize {
	let mut state = State::new(vec![(2, 1), (1, 1), (2, 1), (1, 1), (1, 1)]);
	state.floors[0].chips += 2;
	state.floors[0].gens += 2;
	bfs(state)
}
