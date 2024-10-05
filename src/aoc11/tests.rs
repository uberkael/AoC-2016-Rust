use super::*;

#[test]
fn test_example_state() {
	let state = State::new(vec![(1, 1), (1, 0), (0, 0)]);
	assert_eq!(state.elevator, 1);
	assert_eq!(state.pairs.len(), 3);
	assert_eq!(state.pairs[0], (0, 0));
	assert_eq!(state.pairs[1], (1, 0));
	assert_eq!(state.pairs[2], (1, 1));
}

#[test]
fn test_finished() {
	let state = State::new(vec![(1, 1), (1, 0), (0, 0)]);
	assert_eq!(state.is_finished(3), false);
	let state = State::new(vec![(3, 3), (3, 3), (3, 3)]);
	assert_eq!(state.is_finished(3), true);
	let state = State::new(vec![(3, 2), (3, 3), (3, 3)]);
	assert_eq!(state.is_finished(3), false);
}

#[test]
fn test_is_valid() {
	// [HM, HG]
	let state = State::new(vec![(0, 0)]);
	assert_eq!(state.is_valid(), true);
	// [HM] [HG]
	let state = State::new(vec![(0, 1)]);
	assert_eq!(state.is_valid(), true);
	// [HG] [HM]
	let state = State::new(vec![(1, 0)]);
	assert_eq!(state.is_valid(), true);
	// [] [HM, HG]
	let state = State::new(vec![(1, 1)]);
	assert_eq!(state.is_valid(), true);
	// [HG] [HM LM LG]
	let state = State::new(vec![(1, 0), (1, 1)]);
	assert_eq!(state.is_valid(), false);
	// [LM] [HM HG LG]
	let state = State::new(vec![(1, 1), (0, 1)]);
	assert_eq!(state.is_valid(), true);
}

#[test]
fn test_combinations() {
	let state = State::new(vec![(1, 1), (1, 0), (0, 0)]);
	let items = state.items_on_elevator();
	let combos = combinations(items);
	println!("{:?}", combos);
	let result = vec![
		vec![(1, true)],
		vec![(2, true)],
		vec![(2, false)],
		vec![(1, true), (2, true)],
		vec![(1, true), (2, false)],
		vec![(2, true), (2, false)]];
}
