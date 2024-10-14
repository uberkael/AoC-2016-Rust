// #![allow(dead_code)]

#[cfg(test)]
mod tests;

pub fn aoc22() {
	println!("\nDay 22: Grid Computing");
	println!("━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/22/input.txt").unwrap();

	println!("Part 1:\n{}", part1(&input));
	// println!("Part 2:\n{}", part2(&input));
	// let a = parse_size("92T").unwrap();
	// println!("{}", a);
	// let b = parse_size("501T").unwrap();
	// println!("{}", b);
}

#[derive(Debug, PartialEq)]
struct Node {
	x: u8,
	y: u8,
	size: u16,
	used: u16,
	avail: u16,
}

impl Node {
	fn new(x: u8, y: u8, size: u16, used: u16, avail: u16) -> Self {
		Node { x, y, size, used, avail }
	}
	fn parse(input: &str) -> Option<Self> {
		if !input.starts_with("/dev/grid/node-") {
			return None;
		}
		let parts: Vec<&str> = input.split_whitespace().collect();
		let cord = parts[0].split('-').collect::<Vec<&str>>();
		let x: u8 = parse_cord(cord[1])?;
		let y: u8 = parse_cord(cord[2])?;
		let size = parse_size(parts[1])?;
		let used = parse_size(parts[2])?;
		let avail = parse_size(parts[3])?;
		Some(Node::new(x, y, size, used, avail))
	}
	fn viable(&self, other: &Node) -> bool {
		self.used != 0 && self.used <= other.avail
	}
}

fn parse_cord(cord: &str) -> Option<u8> {
	cord[1..].parse().ok()
}

fn parse_size(s: &str) -> Option<u16> {
	let a = &s[..s.len() - 1];
	a.parse().ok()
}

struct Nodes {
	nodes: Vec<Node>,
}

impl Nodes {
	fn new() -> Self {
		Nodes { nodes: Vec::new() }
	}
	fn add(&mut self, node: Node) {
		self.nodes.push(node);
	}
	fn viable_pairs(&self) -> usize {
		let mut count = 0;
		for i in 0..self.nodes.len() {
			for j in 0..self.nodes.len() {
				if i != j && self.nodes[i].viable(&self.nodes[j]) {
					count += 1;
				}
			}
		}
		count
	}
}

fn part1(input: &str) -> usize {
	let mut nodes = Nodes::new();
	for line in input.lines() {
		if let Some(node) = Node::parse(line) {
			nodes.add(node);
		}
	}
	nodes.viable_pairs()
}
