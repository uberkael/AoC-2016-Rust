use super::*;

#[test]
fn test_disc() {
	let disk = Disc::new(5, 4);
	assert_eq!(disk.initial, 4);
	assert_eq!(disk.positions, 5);
}

#[test]
fn test_parse() {
	let input = "Disc #1 has 5 positions; at time=0, it is at position 4.";
	let discs = parse_input(input);
	assert_eq!(discs.len(), 1);
	assert_eq!(discs[0].initial, 4);
	assert_eq!(discs[0].positions, 5);
}
