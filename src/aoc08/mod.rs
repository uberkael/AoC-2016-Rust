#[cfg(test)]
mod tests;

pub fn aoc08() {
	println!("\nDay 8: Two-Factor Authentication");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/08/input.txt")
		.expect("Error reading file");

	/* Part 1 */
	println!("Part 1:\n{}", part1(&input));

	/* Part 2 */
	println!("Part 2:\n{}", part2(&input));

}

fn part1(s: &str) -> usize {
	let mut screen = Screen([[false; 50]; 6]);
	for line in s.lines() {
		let command = parse_command(line);
		screen.run(command);
	}
	screen.0.iter().map(|row| row.iter().filter(|&&x| x).count()).sum()
}

#[derive(Debug, PartialEq)]
enum Command {
	Rect(usize, usize),
	Row(usize, usize),
	Col(usize, usize),
}

fn parse_command(s: &str) -> Command {
	let mut parts = s.split_whitespace();
	match parts.next().unwrap_or_default() {
		"rect" => {
			let data = parts
				.next()
				.unwrap_or_default()
				.split("x")
				.collect::<Vec<&str>>();
			let w: usize = data[0].parse().unwrap_or_default();
			let h: usize = data[1].parse().unwrap_or_default();
			Command::Rect(w, h)
		},
		"rotate" => {
			let t = parts.next().unwrap_or_default();
			let n = parts.next()
				.unwrap_or_default()
				.split("=")
				.nth(1)
				.unwrap_or_default()
				.parse()
				.unwrap_or_default();
			let _ = parts.next();
			let by: usize = parts.next()
				.unwrap_or_default()
				.parse()
				.unwrap_or_default();
			match t {
				"row" => Command::Row(n, by),
				"column" => Command::Col(n, by),
				_ => panic!("Invalid command"),
			}
		},
		_ => panic!("Invalid command"),
	}
}

impl Command {
	fn run(&self, s: &mut impl Scr) {
		match self {
			Command::Rect(w, h) => {
				for y in 0..*h {
					for x in 0..*w {
						s.set_pixel(x, y, true);
					}
				}
			},
			Command::Col(n, by) => {
				let (col, _) = s.dims();
				let mut v: Vec<_> = (0..col).map(|y| s.get_pixel(*n, y)).collect();
				v.rotate_right(*by);
				for y in 0..col {
					s.set_pixel(*n, y, v[y]);
				}
			},
			Command::Row(n, by) => {
				let (_, row) = s.dims();
				let mut v: Vec<_> = (0..row).map(|x| s.get_pixel(x, *n)).collect();
				v.rotate_right(*by);
				for x in 0..row {
					s.set_pixel(x, *n, v[x]);
				}
			},
		}
	}
}

struct Screen([[bool; 50]; 6]);

trait Scr {
	fn get_pixel(&self, x: usize, y: usize) -> bool;
	fn set_pixel(&mut self, x: usize, y: usize, value: bool);
	fn dims(&self) -> (usize, usize);
	fn run(&mut self, c: Command);
}

impl Scr for Screen {
	fn get_pixel(&self, x: usize, y: usize) -> bool {
		self.0[y][x]
	}
	fn set_pixel(&mut self, x: usize, y: usize, value: bool) {
		self.0[y][x] = value;
	}
	fn dims(&self) -> (usize, usize) {
		(self.0.len(), self.0[0].len())
	}
	fn run(&mut self, c: Command) {
		c.run(self);
	}
}

/* Part 2 */
impl std::fmt::Display for Screen {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		for row in self.0.iter() {
			for &pixel in row.iter() {
				write!(f, "{}", if pixel { '█' } else { '‧' })?;
			}
			writeln!(f)?;
		}
		Ok(())
	}
}

fn part2(s: &str) -> Screen {
	let mut screen = Screen([[false; 50]; 6]);
	for line in s.lines() {
		let command = parse_command(line);
		screen.run(command);
	}
	screen
}
