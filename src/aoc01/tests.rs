use super::*;

#[test]
fn instructions() {
	let a = Instructions::new("R2, L3");
	assert_eq!(a.instructions[0].turn, 'R');
	assert_eq!(a.instructions[0].distance, 2);
	assert_eq!(a.instructions[1].turn, 'L');
	assert_eq!(a.instructions[1].distance, 3);
}

#[test]
fn test_mr_taxi_moves() {
	let mut mr_taxi = MrTaxi::new();
	let instructions = Instructions::new("R2, L3");
	mr_taxi.moves(&instructions);
	assert_eq!(mr_taxi.point.x, 2);
	assert_eq!(mr_taxi.point.y, 3);
}

/* "R2, L3"
(2, 3) 5 */
#[test]
fn test1() {
	let mut mr_taxi = MrTaxi::new();
	let instructions = Instructions::new("R2, L3");
	mr_taxi.moves(&instructions);
	assert_eq!(mr_taxi.distance(), 5);
}

/* "R2, R2, R2"
(0, -2) 2 */
#[test]
fn test2() {
	let mut mr_taxi = MrTaxi::new();
	let instructions = Instructions::new("R2, R2, R2");
	mr_taxi.moves(&instructions);
	assert_eq!(mr_taxi.distance(), 2);
}

/* "R5, L5, R5, R3"
(10, 2) -> 12 */
#[test]
fn test3() {
	let mut mr_taxi = MrTaxi::new();
	let instructions = Instructions::new("R5, L5, R5, R3");
	mr_taxi.moves(&instructions);
	assert_eq!(mr_taxi.distance(), 12);
}

/* Part 2 */
/* "R8, R4, R4, R8" -> 4 */
#[test]
fn test4() {
	let mut mr_taxi = MrTaxi::new();
	let instructions = Instructions::new("R8, R4, R4, R8");
	mr_taxi.moves2(&instructions);
	assert_eq!(mr_taxi.distance(), 4);
}
