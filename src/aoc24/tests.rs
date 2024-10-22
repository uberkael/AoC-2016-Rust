// cSpell:ignoreRegExp ".*"
use super::*;

#[test]
fn test_duct() {
	let d = Duct::new(
	"###########\n\
	 #.0.1..2..#\n\
	 #.3.4..5..#\n\
	 #.6.7..8..#\n\
	 #.....9...#\n\
	 ###########");
	assert_eq!(
		d.nodes[0],
		['#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#']);
	assert_eq!(
		d.nodes[1],
		['#', '.', '0', '.', '1', '.', '.', '2', '.', '.', '#']);
	assert_eq!(
		d.nodes[2],
		['#', '.', '3', '.', '4', '.', '.', '5', '.', '.', '#']);
	assert_eq!(
		d.nodes[5],
		['#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#']);
}

#[test]
fn test_bfs() {
	let d = Duct::new(
	"###########\n\
	 #.0.1..2..#\n\
	 #.3.4..5..#\n\
	 #.6.7..8..#\n\
	 #.....9...#\n\
	 ###########");
	let first = d.locations[0].1;
	let last = d.locations[d.locations.len() - 1].1;
	assert_eq!(7, d.bfs(first, last).expect("No path found"));
}

#[test]
fn test_distances() {
	let d = Duct::new(
	"###########\n\
	 #.0.1..2..#\n\
	 #.3.4..5..#\n\
	 #.6.7..8..#\n\
	 #.....9...#\n\
	 ###########");
	let distances = d.distances();
	assert_eq!(distances[0][1], 2);
	assert_eq!(distances[0][2], 5);
	assert_eq!(distances[0][3], 1);
	assert_eq!(distances[0][4], 3);
	assert_eq!(distances[0][5], 6);
	assert_eq!(distances[0][6], 2);
	assert_eq!(distances[0][7], 4);
	assert_eq!(distances[0][8], 7);
	assert_eq!(distances[0][9], 7);
}

#[test]
fn test_find_shortest() {
	let d = Duct::new(
	"###########\n\
	 #.0.1..2..#\n\
	 #.3.4..5..#\n\
	 #.6.7..8..#\n\
	 #.....9...#\n\
	 ###########");
	let distances = d.distances();
	assert_eq!(find_shortest(&distances), 13);
}
