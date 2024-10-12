// cSpell:ignoreRegExp ".*"
use super::*;

#[test]
fn test_run_operation() {
	let pass = "abcde".to_string();
	let pass = run_operation(pass, Operation::SwapPosition(4, 0));
	assert_eq!(pass, "ebcda");
	let pass = run_operation(pass, Operation::SwapLetter('d', 'b'));
	assert_eq!(pass, "edcba");
	let pass = run_operation(pass, Operation::RotateLeft(1));
	assert_eq!(pass, "dcbae");
	let pass = run_operation(pass, Operation::RotateRight(1));
	assert_eq!(pass, "edcba");
	let pass = run_operation(pass, Operation::RotateRight(3));
	assert_eq!(pass, "cbaed");
	let pass = run_operation(pass, Operation::RotateOnLetter('b'));
	assert_eq!(pass, "edcba");
	let pass = run_operation(pass, Operation::RotateOnLetter('d'));
	assert_eq!(pass, "baedc");
	let pass = run_operation(pass, Operation::Reverse(0, 4));
	assert_eq!(pass, "cdeab");
	let pass = run_operation(pass, Operation::Reverse(1, 2));
	assert_eq!(pass, "cedab");
	let pass = run_operation(pass, Operation::Move(1, 4));
	assert_eq!(pass, "cdabe");
	let pass = run_operation(pass, Operation::Move(3, 0));
	assert_eq!(pass, "bcdae");

	let pass = "abcde".to_string();
	let pass = run_operation(pass, Operation::SwapPosition(4, 0));
	assert_eq!(pass, "ebcda");
	let pass = run_operation(pass, Operation::SwapLetter('d', 'b'));
	assert_eq!(pass, "edcba");
	let pass = run_operation(pass, Operation::Reverse(0, 4));
	assert_eq!(pass, "abcde");
	let pass = run_operation(pass, Operation::RotateLeft(1));
	assert_eq!(pass, "bcdea");
	let pass = run_operation(pass, Operation::Move(1, 4));
	assert_eq!(pass, "bdeac");
	let pass = run_operation(pass, Operation::Move(3, 0));
	assert_eq!(pass, "abdec");
	let pass = run_operation(pass, Operation::RotateOnLetter('b'));
	assert_eq!(pass, "ecabd");
	let pass = run_operation(pass, Operation::RotateOnLetter('d'));
	assert_eq!(pass, "decab");
}

#[test]
fn test_parse() {
	let op = parse_operation("swap position 4 with position 0");
	assert_eq!(op, Some(Operation::SwapPosition(4, 0)));
	let op = parse_operation("swap letter d with letter b ");
	assert_eq!(op, Some(Operation::SwapLetter('d', 'b')));
	let op = parse_operation("reverse positions 0 through 4");
	assert_eq!(op, Some(Operation::Reverse(0, 4)));
	let op = parse_operation("rotate left 1 step");
	assert_eq!(op, Some(Operation::RotateLeft(1)));
	let op = parse_operation("move position 1 to position 4");
	assert_eq!(op, Some(Operation::Move(1, 4)));
	let op = parse_operation("move position 3 to position 0");
	assert_eq!(op, Some(Operation::Move(3, 0)));
	let op = parse_operation("rotate based on position of letter b");
	assert_eq!(op, Some(Operation::RotateOnLetter('b')));
	let op = parse_operation("rotate based on position of letter d");
	assert_eq!(op, Some(Operation::RotateOnLetter('d')));
}
