use super::*;

#[test]
fn test_tile() {
	// let parents = [true, true, false];
	// let tile = Tile::new(parents);
	// assert_eq!(tile.trap, true);
}

#[test]
fn test_count_safe() {
	let (row, n) = parse_row(".^^^^");
	assert_eq!(row, 0b01111);
	assert_eq!(n, 5);
	assert_eq!(count_safe(row, n, 1), 1);

	let (row, n) = parse_row("..^^.");
	assert_eq!(count_safe(row, n, 1), 3);

	let (row, n) = parse_row(".^^^^");
	assert_eq!(count_safe(row, n, 1), 1);

	let (row, n) = parse_row("^^..^");
	assert_eq!(count_safe(row, n, 1), 2);

	let (row, n) = parse_row("^^..^");
	assert_eq!(count_safe(row, n, 1), 2);
	assert_eq!(count_safe(row, n, 2), 3);
	assert_eq!(count_safe(row, n, 3), 5);
	assert_eq!(count_safe(row, n, 4), 6);
	assert_eq!(count_safe(row, n, 5), 8);
}
