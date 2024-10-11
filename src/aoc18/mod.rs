#![allow(dead_code)]

#[cfg(test)]
mod tests;

pub fn aoc18() {
	println!("\nDay 18: Like a Rogue");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/18/input.txt").unwrap();

	println!("Part 1:\n{}", part1(&input));
	// println!("Part 2:\n{}", part2(&input));

}

struct Map {
	size: usize,
	rows: Vec<Row>,
}

impl Map {
	fn new(size: usize, data: String) -> Self {
		let row = Row::new(data);
		Map::new_from_row(size, row)
	}
	fn new_from_row(size: usize, row: Row) -> Self {
		let mut r = Self { size, rows: vec![] };
		r.generate(row);
		r
	}
	fn to_string(&self) -> String {
		self.rows.iter().map(|r| r.to_string()).collect::<Vec<_>>().join("\n")
	}
	fn count_safe(&self) -> usize {
		self.rows.iter().map(|r| r.count_safe()).sum()
	}
	fn generate(&mut self, mut row: Row) {
		self.rows.push(row.clone());
		for _ in 1..self.size {
			let new_row = row.generate_row();
			self.rows.push(new_row.clone());
			row = new_row;
		}
	}
}

#[derive(Clone)]
struct Row {
	tiles: Vec<Tile>,
}

impl Row {
	fn new(data: String) -> Self {
		let row = data.chars().map(|c| Tile { trap: c == '^' }).collect();
		Self { tiles: row }
	}
	fn to_string(&self) -> String {
		self.tiles.iter().map(|t| t.to_string()).collect()
	}
	fn count_safe(&self) -> usize {
		self.tiles.iter().filter(|t| !t.trap).count()
	}
	fn generate_row(&self) -> Row {
		let mut new_row = Row { tiles: Vec::new() };
		let mut parents = [false; 3];
		for (i, tile) in self.tiles.iter().enumerate() {
			if i == 0 {
				parents[0] = false;
			} else {
				parents[0] = self.tiles[i - 1].trap;
			}
			if i == self.tiles.len() - 1 {
				parents[2] = false;
			} else {
				parents[2] = self.tiles[i + 1].trap;
			}
			parents[1] = tile.trap;
			new_row.tiles.push(Tile::new(parents));
		}
		new_row
	}
}

impl std::fmt::Display for Row {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.to_string())
	}
}
#[derive(Clone, Copy)]
struct Tile {
	trap: bool,
}

impl Tile {
	fn new(parents: [bool; 3]) -> Self {
		let trap = match parents {
			[true, true, false] => true,
			[false, true, true] => true,
			[true, false, false] => true,
			[false, false, true] => true,
			_ => false,
		};
		Self { trap }
	}
}

impl std::fmt::Display for Tile {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", if self.trap { '^' } else { '.' })
	}
}

fn part1(input: &str) -> usize {
	// let map = Map::new(10, ".^^.^.^^^^".to_string());
	let map = Map::new(40, input.trim().to_string());
	map.count_safe()
}
