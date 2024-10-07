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
	assert_eq!(check_triangle(&t), false);

	let t : Triangle = [810, 679, 10];
	assert_eq!(check_triangle(&t), false);

	let t : Triangle = [783, 255, 616];
	assert_eq!(check_triangle(&t), true);

	let t : Triangle = [545, 626, 626];
	assert_eq!(check_triangle(&t), true);
}

/* Test 2 */
#[test]
fn test2_parse_input2() {
	let input = "101 301 501\n102 302 502\n103 303 503";
	let triangles = parse_input2(input);
	assert!(triangles.len() == 3);
	assert_eq!(triangles[0], [101, 102, 103]);
	assert_eq!(triangles[1], [301, 302, 303]);
	assert_eq!(triangles[2], [501, 502, 503]);

	let input = "101 301 501\n102 302 502\n103 303 503\n201 401 601\n202 402 602\n203 403 603";
	let triangles = parse_input2(input);
	assert!(triangles.len() == 6);
	assert_eq!(triangles[3], [201, 202, 203]);
	assert_eq!(triangles[4], [401, 402, 403]);
	assert_eq!(triangles[5], [601, 602, 603]);
}
