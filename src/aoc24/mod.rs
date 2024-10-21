use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

#[cfg(test)]
mod tests;

pub fn aoc24() {
	println!("\nDay 24: Air Duct Spelunking");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/24/input.txt")
		.expect("Error reading the file")
		.trim()
		.to_string();

	let d = Duct::new(&input);
	let distances = d.distances();

	println!("Part 1:\n{}", part1(&distances));
	println!("Part 2:\n{}", part2(&distances));
}

struct Duct {
	nodes: Vec<Vec<char>>,
	locations: Vec<(char, (usize, usize))>,
}

impl Duct {
	fn new(input: &str) -> Self {
		let lines: Vec<&str> = input.lines().collect();
		let height = lines.len();
		let width = lines[0].len();
		let mut nodes = vec![vec![' '; width]; height];
		let mut locations = Vec::new();
		for (y, line) in lines.iter().enumerate() {
			for (x, c) in line.chars().enumerate() {
				nodes[y][x] = c;
				if c.is_digit(10) {
					locations.push((c, (x, y)));
				}
			}
		}
		locations.sort_by_key(|&(c, _)| c);
		Duct { nodes, locations }
	}
	#[allow(unused)]
	fn print(&self) {
		for row in &self.nodes {
			for &c in row {
				print!("{}", c);
			}
			println!();
		}
	}

	fn bfs(&self, start: (usize, usize), goal: (usize, usize)) -> Option<usize> {
		let deltas = [(1, 0), (-1, 0), (0, 1), (0, -1)];
		let mut queue = VecDeque::new();
		let mut seen = HashSet::new();
		queue.push_back((start, 0));
		seen.insert(start);
		while let Some((pos, steps)) = queue.pop_front() {
			if pos == goal {
				return Some(steps);
			}
			let (x, y) = pos;
			for (dx, dy) in &deltas {
				let nx = x as isize + dx;
				let ny = y as isize + dy;
				if nx < 0 || ny < 0 {
					continue;
				}
				let nx = nx as usize;
				let ny = ny as usize;
				if ny >= self.nodes.len() || nx >= self.nodes[0].len() {
					continue;
				}
				if self.nodes[ny][nx] == '#' {
					continue;
				}
				let next = (nx, ny);
				if seen.insert(next) {
					queue.push_back((next, steps + 1));
				}
			}
		}
		None
	}

	fn distances(&self) -> Vec<Vec<usize>> {
		let n = self.locations.len();
		let mut distances = vec![vec![0; n]; n];
		for i in 0..n {
			for j in i + 1..n {
				let start = self.locations[i].1;
				let goal = self.locations[j].1;
				if let Some(dist) = self.bfs(start, goal) {
					distances[i][j] = dist;
					distances[j][i] = dist;
				} else {
					panic!("No path {} y {}", i, j);
				}
			}
		}
		distances
	}
}

fn find_shortest(distances: &Vec<Vec<usize>>) -> usize {
	let mut min_distance = usize::MAX;
	let n = distances.len();
	for perm in (1..n).permutations(n - 1) {
		let mut distance = 0;
		let mut prev = 0;
		for &i in perm.iter() {
			distance += distances[prev][i];
			prev = i;
		}
		min_distance = min_distance.min(distance);
	}
	min_distance
}

fn part1(distances: &Vec<Vec<usize>>) -> usize {
	find_shortest(&distances)
}

fn find_shortest_with_return(distances: &Vec<Vec<usize>>) -> usize {
	let mut min_distance = usize::MAX;
	let n = distances.len();
	for perm in (1..n).permutations(n - 1) {
		let mut distance = 0;
		let mut prev = 0;
		for &i in perm.iter() {
			distance += distances[prev][i];
			prev = i;
		}
		distance += distances[prev][0];
		min_distance = min_distance.min(distance);
	}
	min_distance
}

fn part2(distances: &Vec<Vec<usize>>) -> usize {
	find_shortest_with_return(&distances)
}
