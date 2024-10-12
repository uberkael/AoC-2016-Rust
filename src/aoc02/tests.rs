// cSpell:ignoreRegExp U|D|R|L|
use super::*;

/* Methods only for testing */
impl Keypad {
	fn new_at(c: u32) -> Keypad {
		let mut k = Keypad::new();
		for y in 0..3 {
			for x in 0..3 {
				if k.keys[y][x] == c {
					k.x = x;
					k.y = y;
				}
			}
		}
		k
	}
	fn value_at(&self, x: usize, y: usize) -> u32 {
		self.keys[y][x]
	}
}

impl Keypad2 {
	fn new_at(c: char) -> Keypad2 {
		let mut k = Keypad2::new();
		for y in 0..7 {
			for x in 0..7 {
				if k.keys[y][x] == c {
					k.x = x;
					k.y = y;
				}
			}
		}
		k
	}
}

#[test]
fn test_new() {
	let keypad = Keypad::new_at(5);
	assert_eq!(keypad.value(), 5);
}

#[test]
fn test_keypad() {
	let keypad = Keypad::new();

	assert_eq!(keypad.value_at(0, 0), 7);
	assert_eq!(keypad.value_at(1, 1), 5);
	assert_eq!(keypad.value_at(2, 2), 3);

	assert_eq!(keypad.value_at(0, 0), 7);
	assert_eq!(keypad.value_at(1, 0), 8);
	assert_eq!(keypad.value_at(2, 0), 9);

	assert_eq!(keypad.value_at(0, 0), 7);
	assert_eq!(keypad.value_at(0, 1), 4);
	assert_eq!(keypad.value_at(0, 2), 1);
}

#[test]
fn test_movement() {
	/* ULL, ends in (0, 2) */
	let input = "ULL";
	let mut keypad = Keypad::new();
	let instructions = Instructions::new(input);
	keypad.follow(instructions);

	assert_eq!(keypad.x, 0);
	assert_eq!(keypad.y, 2);

	assert!(keypad.value() == 1);
}

#[test]
fn test_1() {
	/* UUL -> 1 */
	let instructions = Instructions::new("ULL");

	let mut keypad = Keypad::new();
	keypad.follow(instructions);

	assert_eq!(keypad.value(), 1);
}

#[test]
fn test_2() {
	/* RRDDD -> 9 */
	let instructions = Instructions::new("RRDDD");

	let mut keypad = Keypad::new_at(1);
	keypad.follow(instructions);

	assert_eq!(keypad.value(), 9);
}

#[test]
fn test_3() {
	/* LURDL -> 8 */
	let instructions = Instructions::new("LURDL");

	let mut keypad = Keypad::new_at(9);
	keypad.follow(instructions);

	assert_eq!(keypad.value(), 8);
}

#[test]
fn test_4() {
	/* UUUUD -> 5 */
	let instructions = Instructions::new("UUUUD");

	let mut keypad = Keypad::new_at(8);
	keypad.follow(instructions);

	assert_eq!(keypad.value(), 5);
}

#[test]
fn test_1234() {
	let input = "ULL\nRRDDD\nLURDL\nUUUUD";
	let result = part1(input);

	assert_eq!(result, "1985");
}

/* Part 2 */
#[test]
fn test_2_1() {
	/* ULL -> 5 */
	let instructions = Instructions::new("ULL");

	let mut keypad = Keypad2::new();
	keypad.follow(instructions);

	assert_eq!(keypad.value(), '5');
}

#[test]
fn test_2_2() {
	/* RRDDD -> A */
	let instructions = Instructions::new("RRDDD");

	let mut keypad = Keypad2::new();
	keypad.follow(instructions);

	assert_eq!(keypad.value(), 'D');
}

#[test]
fn test_2_3() {
	/* LURDL -> A */
	let instructions = Instructions::new("LURDL");

	let mut keypad = Keypad2::new_at('D');
	keypad.follow(instructions);

	assert_eq!(keypad.value(), 'B');
}

#[test]
fn test_2_4() {
	/* UUUUD -> 5 */
	let instructions = Instructions::new("UUUUD");

	let mut keypad = Keypad2::new_at('5');
	keypad.follow(instructions);

	assert_eq!(keypad.value(), '5');
}

#[test]
fn test_2_1234() {
	let input = "ULL\nRRDDD\nLURDL\nUUUUD";
	let result = part2(input);

	assert_eq!(result, "5DB3");
}
