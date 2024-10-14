// #![allow(dead_code)]

#[cfg(test)]
mod tests;

pub fn aoc22() {
	println!("\nDay 22: Grid Computing");
	println!("━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/22/input.txt").unwrap();

	let mut nodes = Nodes::new();
	for line in input.lines() {
		if let Some(node) = Node::parse(line) {
			nodes.add(node);
		}
	}
	println!("Part 1:\n{}", part1(&nodes));
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
		Node {
			x,
			y,
			size,
			used,
			avail,
		}
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
	fn viable_count(&self) -> usize {
		let mut count = 0;
		let mut avail: Vec<_> = self.nodes.iter().map(|n| n.avail).collect();
		avail.sort_unstable_by(|a, b| b.cmp(a));
		for n in &self.nodes {
			if n.used > 0 {
				let pos = avail.partition_point(|&a| a >= n.used);
				if n.used <= n.avail {
					count += pos - 1; // el mismo no cuenta
				} else {
					count += pos;
				}
			}
		}
		count
	}
}

fn part1(nodes: &Nodes) -> usize {
	nodes.viable_count()
}
