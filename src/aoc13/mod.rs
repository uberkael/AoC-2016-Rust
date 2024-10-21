use std::collections::VecDeque;

#[cfg(test)]
mod tests;

pub fn aoc13() {
	println!("\nDay 13: A Maze of Twisty Little Cubicles");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/13/input.txt")
		.expect("Error reading the file");

	let favorite = input.trim().parse().expect("Invalid input");
	let maze = Maze::new(favorite, 50, 50);

	println!("Part 1:\n{}", part1(maze.clone()));
	println!("Part 2:\n{}", part2(maze));
}

#[derive(Debug, PartialEq, Clone)]
enum Cell {
	Wall,
	Open,
	Visited,
}

#[derive(Clone)]
struct Maze {
	pos: (usize, usize),
	grid: Vec<Vec<Cell>>,
}

impl Maze {
	fn new(favorite: usize, n: usize, m: usize) -> Self {
		let mut grid = vec![vec![Cell::Wall; n]; m];
		for y in 0..m {
			for x in 0..n {
				grid[y][x] = if Self::is_open(favorite, x, y) {
					Cell::Open
				} else {
					Cell::Wall
				};
			}
		}
		grid[1][1] = Cell::Visited;
		Maze {
			pos: (1, 1),
			grid,
		}
	}
	#[allow(dead_code)]
	fn new_example() -> Self {
		Maze::new(10, 10, 7)
	}
	fn is_open(favorite: usize, x: usize, y: usize) -> bool {
		let num = x * x + 3 * x + 2 * x * y + y + y * y + favorite;
		num.count_ones() % 2 == 0
	}
	#[allow(dead_code)]
	fn print(&self) {
		for y in 0..self.grid.len() {
			for x in 0..self.grid[y].len() {
				print!(
					"{}",
					match self.grid[y][x] {
						Cell::Wall => '#',
						Cell::Open => '.',
						Cell::Visited => 'O',
					}
				);
			}
			println!();
		}
	}
	fn solve(&self, target_x: usize, target_y: usize) -> usize {
		let mut queue = VecDeque::new();
		let mut visited = vec![vec![false; self.grid[0].len()]; self.grid.len()];

		queue.push_back((self.pos.0, self.pos.1, 0));
		visited[self.pos.1][self.pos.0] = true;

		while let Some((x, y, steps)) = queue.pop_front() {
			if x == target_x && y == target_y {
				return steps;
			}

			let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
			for (dx, dy) in directions.iter() {
				let nx = x as isize + dx;
				let ny = y as isize + dy;

				if nx >= 0
					&& ny >= 0 && (ny as usize) < self.grid.len()
					&& (nx as usize) < self.grid[0].len()
					&& !visited[ny as usize][nx as usize]
					&& self.grid[ny as usize][nx as usize] == Cell::Open
				{
					visited[ny as usize][nx as usize] = true;
					queue.push_back((nx as usize, ny as usize, steps + 1));
				}
			}
		}
		usize::MAX
	}
	fn solve_till_n(&self, n: usize) -> usize {
		let mut queue = VecDeque::new();
		let mut visited = vec![vec![false; self.grid[0].len()]; self.grid.len()];

		queue.push_back((self.pos.0, self.pos.1, 0));
		visited[self.pos.1][self.pos.0] = true;

		while let Some((x, y, steps)) = queue.pop_front() {
			if steps == n {
				return visited.iter().flatten().filter(|&&v| v).count();
			}

			let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
			for (dx, dy) in directions.iter() {
				let nx = x as isize + dx;
				let ny = y as isize + dy;

				if nx >= 0
					&& ny >= 0 && (ny as usize) < self.grid.len()
					&& (nx as usize) < self.grid[0].len()
					&& !visited[ny as usize][nx as usize]
					&& self.grid[ny as usize][nx as usize] == Cell::Open
				{
					visited[ny as usize][nx as usize] = true;
					queue.push_back((nx as usize, ny as usize, steps + 1));
				}
			}
		}
		usize::MAX
	}
}

/* Part1 */
fn part1(maze: Maze) -> usize {
	// maze.print();
	maze.solve(31, 39)
}

/* Part2 */
fn part2(maze: Maze) -> usize {
	maze.solve_till_n(50)
}
