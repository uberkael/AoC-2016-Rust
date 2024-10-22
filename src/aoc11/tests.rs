use super::*;

#[test]
fn test_example_state() {
	let state = State::new(vec![(2, 1), (1, 1), (2, 1), (1, 1), (1, 1)]);
	assert_eq!(state.elevator, 1);

	assert_eq!(state.floors[0].chips, 3);
	assert_eq!(state.floors[0].gens, 5);

	assert_eq!(state.floors[1].chips, 2);
	assert_eq!(state.floors[1].gens, 0);

	assert_eq!(state.floors[2].chips, 0);
	assert_eq!(state.floors[2].gens, 0);

	assert_eq!(state.floors[3].chips, 0);
	assert_eq!(state.floors[3].gens, 0);
}

#[test]
fn test_cant_move() {
	let state = State::new(vec![(2, 2), (2, 1), (1, 1)]);
	let cur = 0;
	let m = Floor { gens: 1, chips: 1 };
	assert_eq!(cant_move(cur, &m, state), false);
	let state = State::new(vec![(3, 3), (3, 3), (3, 3)]);
	let cur = 0;
	let m = Floor { gens: 1, chips: 1 };
	assert_eq!(cant_move(cur, &m, state), true);
	let state = State::new(vec![(3, 2), (3, 3), (3, 3)]);
	let cur = 0;
	let m = Floor { gens: 1, chips: 1 };
	assert_eq!(cant_move(cur, &m, state), true);
}

#[test]
fn test_is_valid() {
	// [HM, HG]
	let state = State::new(vec![(1, 1)]);
	assert_eq!(state.is_valid(), true);
	// [HM] [HG]
	let state = State::new(vec![(1, 2)]);
	assert_eq!(state.is_valid(), true);
	// [HG] [HM]
	let state = State::new(vec![(2, 1)]);
	assert_eq!(state.is_valid(), true);
	// [] [HM, HG]
	let state = State::new(vec![(2, 2)]);
	assert_eq!(state.is_valid(), true);
	// [HG] [HM LM LG]
	let state = State::new(vec![(2, 1), (2, 2)]);
	assert_eq!(state.is_valid(), false);
	// [LM] [HM HG LG]
	let state = State::new(vec![(2, 2), (1, 2)]);
	assert_eq!(state.is_valid(), true);
}
