use super::*;

#[test]
fn test_parse_triangle() {
	let [a, b, c] = parse_triangle("5 10 25");
	assert_eq!(a, 5);
	assert_eq!(b, 10);
	assert_eq!(c, 25);

	let [a, b, c] = parse_triangle("  810  679  10");
	assert_eq!(a, 810);
	assert_eq!(b, 679);
	assert_eq!(c, 10);

	let [a, b, c] = parse_triangle("  783  255  616 ");
	assert_eq!(a, 783);
	assert_eq!(b, 255);
	assert_eq!(c, 616);

	let [a, b, c] = parse_triangle("  545  626  626");
	assert_eq!(a, 545);
	assert_eq!(b, 626);
	assert_eq!(c, 626);
}

#[test]
fn test_check_triangle() {
	let t : Triangle = [5, 10, 25];
	assert_eq!(check_triangle(t), false);

	let t : Triangle = [810, 679, 10];
	assert_eq!(check_triangle(t), false);

	let t : Triangle = [783, 255, 616];
	assert_eq!(check_triangle(t), true);

	let t : Triangle = [545, 626, 626];
	assert_eq!(check_triangle(t), true);
}
