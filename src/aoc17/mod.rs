#![allow(dead_code)]

use std::collections::HashMap;

#[cfg(test)]
mod tests;

pub fn aoc17() {
	println!("\nDay 17: Two Steps Forward");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/17/input.txt").unwrap();

	println!("Part 1:\n{}", part1(&input));
	println!("Part 2:\n{}", part2(&input));
}

#[derive(Copy, Clone)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}

#[derive(Debug, PartialEq)]
struct Room {
	doors: [bool; 4], // up, down, left, and right
	passcode: Vec<u8>,
	position: (u8, u8),
}

impl Room {
	fn new(passcode: Vec<u8>, position: (u8, u8)) -> Self {
		Room {
			doors: Self::check_directions(&passcode),
			passcode,
			position,
		}
	}
	fn check_directions(passcode: &[u8]) -> [bool; 4] {
		let hash = format!("{:x}", md5::compute(passcode));
		hash.chars()
			.take(4)
			.map(|c| c > 'a')
			.collect::<Vec<bool>>()
			.try_into()
			.unwrap()
	}
	fn check_limit(&self, direction: &Direction) -> bool {
		let (x, y) = self.position;
		match direction {
			Direction::Up    => y > 0,
			Direction::Down  => y < 3,
			Direction::Left  => x > 0,
			Direction::Right => x < 3,
		}
	}
	fn next(&self, direction: Direction) -> Option<Self> {
		if !self.check_limit(&direction) {
			return None;
		}
		let (x, y) = self.position;
		let (new_position, letter) = match direction {
			Direction::Up    => ((x, y.saturating_sub(1)), b'U'),
			Direction::Down  => ((x, y + 1), b'D'),
			Direction::Left  => ((x.saturating_sub(1), y), b'L'),
			Direction::Right => ((x + 1, y), b'R'),
		};
		// Concatenate the current passcode with the new letter
		let mut new_passcode = self.passcode.clone();
		new_passcode.push(letter);
		Some(Room::new(new_passcode, new_position))
	}
}

fn bfs(room: Room) -> String {
	let original_passcode = room.passcode.clone();
	let mut queue = std::collections::VecDeque::new();
	queue.push_back(room);
	while let Some(room) = queue.pop_front() {
		if room.position == (3, 3) {
			let path = room.passcode[original_passcode.len()..].to_vec();
			return String::from_utf8(path).unwrap();
		}
		let directions = [
			(Direction::Up, 0),
			(Direction::Down, 1),
			(Direction::Left, 2),
			(Direction::Right, 3),
		];
		for (direction, index) in directions.iter() {
			if room.doors[*index] {
				if let Some(next_room) = room.next(*direction) {
					queue.push_back(next_room);
				}
			}
		}
	}
	String::new()
}

fn part1(input: &str) -> String {
	let passcode = input.trim().as_bytes().to_vec();
	let initial_room = Room::new(passcode, (0, 0));
	bfs(initial_room)
}

fn dfs(
	room: Room,
	memo: &mut HashMap<(u8, u8, String), Option<usize>>,
) -> Option<usize> {
	let current_path = String::from_utf8(room.passcode.to_vec()).unwrap();
	let key = (room.position.0, room.position.1, current_path.clone());
	if let Some(&cached) = memo.get(&key) {
		return cached;
	}
	if room.position == (3, 3) {
		let res = Some(current_path.len());
		memo.insert(key, res);
		return res;
	}
	let directions = [
		(Direction::Up, 0),
		(Direction::Down, 1),
		(Direction::Left, 2),
		(Direction::Right, 3),
	];
	let mut best: Option<usize> = None;
	for (direction, index) in directions.iter() {
		if room.doors[*index] {
			if let Some(next_room) = room.next(*direction) {
				if let Some(path_len) = dfs(next_room, memo) {
					best = Some(match best {
						Some(current_best) => current_best.max(path_len),
						None => path_len,
					});
				}
			}
		}
	}
	memo.insert(key, best);
	best
}

fn longest_path(room: Room) -> usize {
	let original_len = room.passcode.len();
	let mut memo = HashMap::new();
	dfs(room, &mut memo).unwrap_or(0) - original_len
}

fn part2(input: &str) -> usize {
	let passcode = input.trim().as_bytes().to_vec();
	let initial_room = Room::new(passcode, (0, 0));
	longest_path(initial_room)
}
