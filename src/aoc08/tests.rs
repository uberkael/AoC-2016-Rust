use std::{
	fmt::{Display, Formatter, Result},
	ops::Index,
};

use super::*;

#[derive(Debug, PartialEq)]
struct ScreenMini([[bool; 7]; 3]);

impl ScreenMini {
	fn from_string(input: &str) -> ScreenMini {
		let mut screen_data = [[false; 7]; 3];
		for (row_idx, line) in input.lines().enumerate() {
			for (col_idx, ch) in line.chars().enumerate() {
				screen_data[row_idx][col_idx] = match ch {
					'#' => true,
					'.' => false,
					_ => false,
				};
			}
		}
		ScreenMini(screen_data)
	}
}

/* Impl Display */
impl Display for ScreenMini {
	fn fmt(&self, f: &mut Formatter) -> Result {
		for y in 0..3 {
			for x in 0..7 {
				if self.0[y][x] {
					write!(f, "#")?;
				} else {
					write!(f, ".")?;
				}
			}
			write!(f, "\n")?;
		}
		Ok(())
	}
}

impl Scr for ScreenMini {
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

impl Index<usize> for ScreenMini {
	type Output = [bool; 7];

	fn index(&self, index: usize) -> &Self::Output {
		&self.0[index]
	}
}

fn setup() -> ScreenMini {
	let s = ScreenMini([[false; 7]; 3]);
	s
}

#[test]
fn test1_setup() {
	let s = setup();
	println!("{}", s);

	assert_eq!(s[0][0], false);
	assert_eq!(s[0][6], false);
	assert_eq!(s[2][0], false);
	assert_eq!(s[2][6], false);
	assert_eq!(
		s,
		ScreenMini::from_string(
			".......\n\
			 .......\n\
			 ......."
		)
	);
}

#[test]
fn test1_parse_command() {
	assert_eq!(parse_command("rect 3x2"), Command::Rect(3, 2));
	assert_eq!(parse_command("rotate row y=0 by 4"), Command::Row(0, 4));
	assert_eq!(parse_command("rotate column x=1 by 1"), Command::Col(1, 1));
}

#[test]
fn test1_rect() {
	let mut s = setup();
	s.run(Command::Rect(3, 2));
	println!("{}", s);

	assert_eq!(
		s,
		ScreenMini::from_string(
			"###....\n\
			 ###....\n\
			 ......."
		)
	);
}

#[test]
fn test1_column() {
	let mut s = setup();
	s.run(Command::Rect(3, 2));
	s.run(Command::Col(1, 1));
	println!("{}", s);

	assert_eq!(
		s,
		ScreenMini::from_string(
			"#.#....\n\
			 ###....\n\
			 .#....."
		)
	);
}

#[test]
fn test1_row() {
	let mut s = setup();
	s.run(Command::Rect(3, 2));
	s.run(Command::Col(1, 1));
	s.run(Command::Row(0, 4));
	println!("{}", s);

	assert_eq!(
		s,
		ScreenMini::from_string(
			"....#.#\n\
			 ###....\n\
			 .#....."
		)
	);
}

#[test]
fn test1_all() {
	let mut s = setup();
	s.run(Command::Rect(3, 2));
	s.run(Command::Col(1, 1));
	s.run(Command::Row(0, 4));
	s.run(Command::Col(1, 1));
	println!("{}", s);

	assert_eq!(
		s,
		ScreenMini::from_string(
			".#..#.#\n\
			 #.#....\n\
			 .#....."
		)
	);
}
