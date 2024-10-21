use std::collections::VecDeque;

#[cfg(test)]
mod tests;

pub fn aoc17() {
	println!("\nDay 17: Two Steps Forward");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/17/input.txt")
		.expect("Error reading input file");

	let passcode = input.trim().as_bytes().to_vec();

	println!("Part 1:\n{}", part1(passcode.clone()));
	println!("Part 2:\n{}", part2(passcode));
}

#[derive(Copy, Clone)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}

#[derive(Clone)]
struct Room {
	buffer: Vec<u8>,
	position: (u8, u8),
	doors: [bool; 4],
}

impl Room {
	fn new(buffer: Vec<u8>, position: (u8, u8)) -> Self {
		let doors = Self::check_directions(&buffer);
		Room {
			buffer,
			position,
			doors,
		}
	}

	fn check_directions(buffer: &[u8]) -> [bool; 4] {
		let digest = md5::compute(buffer);
		let bytes = digest.0;
		[
			(bytes[0] >> 4)  > 0xA, // Up
			(bytes[0] & 0xF) > 0xA, // Down
			(bytes[1] >> 4)  > 0xA, // Left
			(bytes[1] & 0xF) > 0xA, // Right
		]
	}
	fn next(&self, direction: Direction) -> Option<Self> {
		let (x, y) = self.position;
		let (new_x, new_y) = match direction {
			Direction::Up    if y > 0 => (x, y - 1),
			Direction::Down  if y < 3 => (x, y + 1),
			Direction::Left  if x > 0 => (x - 1, y),
			Direction::Right if x < 3 => (x + 1, y),
			_ => return None,
		};
		let mut new_buffer = self.buffer.clone();
		new_buffer.push(match direction {
			Direction::Up    => b'U',
			Direction::Down  => b'D',
			Direction::Left  => b'L',
			Direction::Right => b'R',
		});
		Some(Room::new(new_buffer, (new_x, new_y)))
	}
}

fn part1(passcode: Vec<u8>) -> String {
	let initial_room = Room::new(passcode, (0, 0));
	bfs_shortest(initial_room)
}

fn bfs_shortest(initial_room: Room) -> String {
	let passcode_len = initial_room.buffer.len();
	let mut queue = VecDeque::new();
	queue.push_back(initial_room);
	while let Some(room) = queue.pop_front() {
		if room.position == (3, 3) {
			return String::from_utf8(room.buffer[passcode_len..].to_vec())
				.expect("Invalid UTF-8");
		}
		for (dir_idx, direction) in [
			Direction::Up,
			Direction::Down,
			Direction::Left,
			Direction::Right,
		]
		.iter()
		.enumerate()
		{
			if room.doors[dir_idx] {
				if let Some(next) = room.next(*direction) {
					queue.push_back(next);
				}
			}
		}
	}
	String::new()
}

fn part2(passcode: Vec<u8>) -> usize {
	let initial_room = Room::new(passcode, (0, 0));
	longest_path(initial_room)
}

fn longest_path(initial_room: Room) -> usize {
	let passcode_len = initial_room.buffer.len();
	let mut max_length = 0;
	let mut stack = vec![initial_room];
	while let Some(room) = stack.pop() {
		if room.position == (3, 3) {
			let path_length = room.buffer.len() - passcode_len;
			max_length = max_length.max(path_length);
			continue;
		}
		for (dir_idx, direction) in [
			Direction::Up,
			Direction::Down,
			Direction::Left,
			Direction::Right,
		]
		.iter()
		.enumerate()
		{
			if room.doors[dir_idx] {
				if let Some(next_room) = room.next(*direction) {
					stack.push(next_room);
				}
			}
		}
	}
	max_length
}
