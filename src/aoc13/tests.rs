use super::*;

#[test]
fn test_maze() {
	let maze = Maze::new_example();
	assert_eq!(maze.grid[0][0], Cell::Open);
	assert_eq!(maze.grid[0][1], Cell::Wall);
	assert_eq!(maze.grid[0][2], Cell::Open);
	assert_eq!(maze.grid[0][3], Cell::Wall);
	assert_eq!(maze.grid[0][4], Cell::Wall);
	assert_eq!(maze.grid[0][5], Cell::Wall);
	assert_eq!(maze.grid[0][6], Cell::Wall);

	assert_eq!(maze.grid[1][0], Cell::Open);
	assert_eq!(maze.grid[2][0], Cell::Wall);
	assert_eq!(maze.grid[3][0], Cell::Wall);
	assert_eq!(maze.grid[4][0], Cell::Open);
	assert_eq!(maze.grid[5][0], Cell::Open);
	assert_eq!(maze.grid[6][0], Cell::Wall);
	// maze.print();
}

#[test]
fn test_solve() {
	let maze = Maze::new_example();
	assert_eq!(maze.solve(7,4), 11);
}
