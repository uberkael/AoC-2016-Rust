// cSpell:ignoreRegExp ".*"
use super::*;

#[test]
fn test_run_operation() {
	let mut pass = "abcde".as_bytes().to_vec();
	Operation::SwapPosition(4, 0).scramble(&mut pass);
	assert_eq!(pass, "ebcda".as_bytes().to_vec());
	Operation::SwapLetter(b'd', b'b').scramble(&mut pass);
	assert_eq!(pass, "edcba".as_bytes().to_vec());
	Operation::RotateLeft(1).scramble(&mut pass);
	assert_eq!(pass, "dcbae".as_bytes().to_vec());
	Operation::RotateRight(1).scramble(&mut pass);
	assert_eq!(pass, "edcba".as_bytes().to_vec());
	Operation::RotateRight(3).scramble(&mut pass);
	assert_eq!(pass, "cbaed".as_bytes().to_vec());
	Operation::RotateOnLetter(b'b').scramble(&mut pass);
	assert_eq!(pass, "edcba".as_bytes().to_vec());
	Operation::RotateOnLetter(b'd').scramble(&mut pass);
	assert_eq!(pass, "baedc".as_bytes().to_vec());
	Operation::Reverse(0, 4).scramble(&mut pass);
	assert_eq!(pass, "cdeab".as_bytes().to_vec());
	Operation::Reverse(1, 2).scramble(&mut pass);
	assert_eq!(pass, "cedab".as_bytes().to_vec());
	Operation::Move(1, 4).scramble(&mut pass);
	assert_eq!(pass, "cdabe".as_bytes().to_vec());
	Operation::Move(3, 0).scramble(&mut pass);
	assert_eq!(pass, "bcdae".as_bytes().to_vec());

	let mut pass = "abcde".as_bytes().to_vec();
	Operation::SwapPosition(4, 0).scramble(&mut pass);
	assert_eq!(pass, "ebcda".as_bytes().to_vec());
	Operation::SwapLetter(b'd', b'b').scramble(&mut pass);
	assert_eq!(pass, "edcba".as_bytes().to_vec());
	Operation::Reverse(0, 4).scramble(&mut pass);
	assert_eq!(pass, "abcde".as_bytes().to_vec());
	Operation::RotateLeft(1).scramble(&mut pass);
	assert_eq!(pass, "bcdea".as_bytes().to_vec());
	Operation::Move(1, 4).scramble(&mut pass);
	assert_eq!(pass, "bdeac".as_bytes().to_vec());
	Operation::Move(3, 0).scramble(&mut pass);
	assert_eq!(pass, "abdec".as_bytes().to_vec());
	Operation::RotateOnLetter(b'b').scramble(&mut pass);
	assert_eq!(pass, "ecabd".as_bytes().to_vec());
	Operation::RotateOnLetter(b'd').scramble(&mut pass);
	assert_eq!(pass, "decab".as_bytes().to_vec());
}

#[test]
fn test_parse() {
	let op = Operation::parse("swap position 4 with position 0");
	assert_eq!(op, Some(Operation::SwapPosition(4, 0)));
	let op = Operation::parse("swap letter d with letter b ");
	assert_eq!(op, Some(Operation::SwapLetter(b'd', b'b')));
	let op = Operation::parse("reverse positions 0 through 4");
	assert_eq!(op, Some(Operation::Reverse(0, 4)));
	let op = Operation::parse("rotate left 1 step");
	assert_eq!(op, Some(Operation::RotateLeft(1)));
	let op = Operation::parse("move position 1 to position 4");
	assert_eq!(op, Some(Operation::Move(1, 4)));
	let op = Operation::parse("move position 3 to position 0");
	assert_eq!(op, Some(Operation::Move(3, 0)));
	let op = Operation::parse("rotate based on position of letter b");
	assert_eq!(op, Some(Operation::RotateOnLetter(b'b')));
	let op = Operation::parse("rotate based on position of letter d");
	assert_eq!(op, Some(Operation::RotateOnLetter(b'd')));
}

#[test]
fn test_run() {
	let mut pass = "abcde".as_bytes().to_vec();
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
		// println!("{}", pass);
		if let Some(op) = Operation::parse(line) {
			op.scramble(&mut pass);
			assert_eq!(pass, result.as_bytes());
		}
	}
}

#[test]
fn test_inverse_run() {
	let mut pass = "decab".as_bytes().to_vec();
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
		// println!("{}", pass);
		if let Some(op) = Operation::parse(line) {
			op.unscramble(&mut pass);
			assert_eq!(pass, result.as_bytes());
		}
	}
}
