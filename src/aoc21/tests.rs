// cSpell:ignoreRegExp ".*"
use super::*;

#[test]
fn test_run_operation() {
	let pass = "abcde".to_string();
	let pass = Operation::SwapPosition(4, 0).scramble(pass);
	assert_eq!(pass, "ebcda");
	let pass = Operation::SwapLetter('d', 'b').scramble(pass);
	assert_eq!(pass, "edcba");
	let pass = Operation::RotateLeft(1).scramble(pass);
	assert_eq!(pass, "dcbae");
	let pass = Operation::RotateRight(1).scramble(pass);
	assert_eq!(pass, "edcba");
	let pass = Operation::RotateRight(3).scramble(pass);
	assert_eq!(pass, "cbaed");
	let pass = Operation::RotateOnLetter('b').scramble(pass);
	assert_eq!(pass, "edcba");
	let pass = Operation::RotateOnLetter('d').scramble(pass);
	assert_eq!(pass, "baedc");
	let pass = Operation::Reverse(0, 4).scramble(pass);
	assert_eq!(pass, "cdeab");
	let pass = Operation::Reverse(1, 2).scramble(pass);
	assert_eq!(pass, "cedab");
	let pass = Operation::Move(1, 4).scramble(pass);
	assert_eq!(pass, "cdabe");
	let pass = Operation::Move(3, 0).scramble(pass);
	assert_eq!(pass, "bcdae");

	let pass = "abcde".to_string();
	let pass = Operation::SwapPosition(4, 0).scramble(pass);
	assert_eq!(pass, "ebcda");
	let pass = Operation::SwapLetter('d', 'b').scramble(pass);
	assert_eq!(pass, "edcba");
	let pass = Operation::Reverse(0, 4).scramble(pass);
	assert_eq!(pass, "abcde");
	let pass = Operation::RotateLeft(1).scramble(pass);
	assert_eq!(pass, "bcdea");
	let pass = Operation::Move(1, 4).scramble(pass);
	assert_eq!(pass, "bdeac");
	let pass = Operation::Move(3, 0).scramble(pass);
	assert_eq!(pass, "abdec");
	let pass = Operation::RotateOnLetter('b').scramble(pass);
	assert_eq!(pass, "ecabd");
	let pass = Operation::RotateOnLetter('d').scramble(pass);
	assert_eq!(pass, "decab");
}

#[test]
fn test_parse() {
	let op = Operation::parse("swap position 4 with position 0");
	assert_eq!(op, Some(Operation::SwapPosition(4, 0)));
	let op = Operation::parse("swap letter d with letter b ");
	assert_eq!(op, Some(Operation::SwapLetter('d', 'b')));
	let op = Operation::parse("reverse positions 0 through 4");
	assert_eq!(op, Some(Operation::Reverse(0, 4)));
	let op = Operation::parse("rotate left 1 step");
	assert_eq!(op, Some(Operation::RotateLeft(1)));
	let op = Operation::parse("move position 1 to position 4");
	assert_eq!(op, Some(Operation::Move(1, 4)));
	let op = Operation::parse("move position 3 to position 0");
	assert_eq!(op, Some(Operation::Move(3, 0)));
	let op = Operation::parse("rotate based on position of letter b");
	assert_eq!(op, Some(Operation::RotateOnLetter('b')));
	let op = Operation::parse("rotate based on position of letter d");
	assert_eq!(op, Some(Operation::RotateOnLetter('d')));
}

#[test]
fn test_run() {
	let mut pass = "abcde".to_string();
	let results = ["ebcda","edcba","abcde","bcdea","bdeac","abdec","ecabd","decab"];
	let instructions = "swap position 4 with position 0
	swap letter d with letter b
	reverse positions 0 through 4
	rotate left 1 step
	move position 1 to position 4
	move position 3 to position 0
	rotate based on position of letter b
	rotate based on position of letter d";
	for (line, result) in instructions.lines().zip(results.iter()) {
		println!("{}", pass);
		if let Some(op) = Operation::parse(line) {
			pass = op.scramble(pass);
			assert_eq!(pass, *result);
		}
	}
}

#[test]
fn test_inverse_run() {
	let mut pass = "decab".to_string();
	let results = ["ecabd","abdec","bdeac","bcdea","abcde","edcba","ebcda","abcde"];
	let instructions = "swap position 4 with position 0
	swap letter d with letter b
	reverse positions 0 through 4
	rotate left 1 step
	move position 1 to position 4
	move position 3 to position 0
	rotate based on position of letter b
	rotate based on position of letter d";
	for (line, result) in instructions.lines().rev().zip(results.iter()) {
		println!("{}", pass);
		if let Some(op) = Operation::parse(line) {
			pass = op.unscramble(pass);
			assert_eq!(pass, *result);
		}
	}
}
