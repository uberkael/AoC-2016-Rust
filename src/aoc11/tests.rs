use super::*;

#[test]
fn test_example_state() {
	let state = vec![vec!["E", "HM", "LM"], vec!["HG"], vec!["LG"]];
	assert_eq!(state.len(), 3);
	assert_eq!(state[0], vec!["E", "HM", "LM"]);
	assert_eq!(state[1], vec!["HG"]);
	assert_eq!(state[2], vec!["LG"]);
}

#[test]
fn test_finished() {
	let state = vec![
		vec![],
		vec![],
		vec!["E", "LG", "HG", "HM", "LM"]
	];
	assert_eq!(finished(&state), true);
	let state = vec![
		vec!["E", "HM", "LM"],
		vec!["HG"],
		vec!["LG"]
	];
	assert_eq!(finished(&state), false);
	let state = vec![
		vec!["LM"],
		vec![],
		vec!["E", "LG", "HG", "HM"]
	];
	assert_eq!(finished(&state), false);
}

#[test]
fn test_generate_floors() {
	let f1 = vec!["E", "HM", "LM"];
	let f2 = vec!["HG"];
	let pairs = generate_floors(&f1, &f2);
	assert_eq!(pairs.len(), 1);
	assert_eq!(pairs, vec![
		[vec!["LM"], vec!["E", "HG", "HM"]]
	]);

	let f1 = vec!["E", "HG", "HM"];
	let f2 = vec!["LG"];
	let pairs = generate_floors(&f1, &f2);
	assert_eq!(pairs.len(), 2);
	assert_eq!(pairs, vec![
		[vec!["HM"], vec!["E", "LG", "HG"]],
		[vec![], vec!["E", "LG", "HG", "HM"]]
	]);

	let f1 = vec!["HG", "HM"];
	let f2 = vec!["E", "LG"];
	let pairs = generate_floors(&f1, &f2);
	assert_eq!(pairs.len(), 1);
	assert_eq!(pairs,
		vec![[vec!["E", "HG", "HM", "LG"], vec![]]]);

	let f1 = vec!["HG", "HM"];
	let f2 = vec!["E", "LG", "FG", "FM"];
	let pairs = generate_floors(&f1, &f2);
	assert_eq!(pairs.len(), 1);
	assert_eq!(pairs, vec![
		[vec!["E", "HG", "HM", "LG"], vec!["FG", "FM"]]
	]);

	let f1 = vec!["HG", "HM", "LG", "FG"];
	let f2 = vec!["E", "FM", "LM", "ZM"];
	let pairs = generate_floors(&f1, &f2);
	assert_eq!(pairs.len(), 2);
	assert_eq!(pairs, vec![
		[vec!["E", "HG", "HM", "LG", "FG", "FM"], vec!["LM", "ZM"]],
		[vec!["E", "HG", "HM", "LG", "FG", "LM"], vec!["FM", "ZM"]]
	]);
}

#[test]
fn test_is_valid() {
		assert_eq!(is_valid(&vec!["HM"]), true);
		assert_eq!(is_valid(&vec!["HG"]), true);
		assert_eq!(is_valid(&vec!["HM", "LM"]), true);
		assert_eq!(is_valid(&vec!["HG", "LG"]), true);
		assert_eq!(is_valid(&vec!["HG", "HM"]), true);
		assert_eq!(is_valid(&vec!["LG", "HG", "HM"]), true);
		assert_eq!(is_valid(&vec!["HG", "LM"]), false);
		assert_eq!(is_valid(&vec!["FG", "HG", "LM"]), false);
}

#[test]
fn test_find_elevator() {
	let state = vec![
		vec!["E", "HM", "LM"],
		vec!["HG"],
		vec!["LG"]
	];
	assert_eq!(find_elevator(&state), 0);
	let state = vec![
		vec!["HM", "LM"],
		vec!["HG"],
		vec!["E", "LG"]
	];
	assert_eq!(find_elevator(&state), 2);
	let state = vec![
		vec!["HM", "LM"],
		vec!["E", "HG"],
		vec!["LG"]
	];
	assert_eq!(find_elevator(&state), 1);
}

#[test]
fn test_generate_states() {
	let state = vec![
		vec!["E", "HM", "LM"],
		vec!["HG"],
		vec!["LG"]
	];
	let states = generate_states(&state);
	assert_eq!(states.len(), 1);
	assert_eq!(states, vec![
		vec![
			vec!["LM"],
			vec!["E", "HG", "HM"],
			vec!["LG"]
		]
	]);

	let state = vec![
		vec!["E", "HM", "LM"],
		vec!["HG", "LG"],
		vec![]
	];
	let states = generate_states(&state);
	assert_eq!(states.len(), 3);
	assert_eq!(states, vec![
		vec![
			vec!["LM"],
			vec!["E", "HG", "LG", "HM"],
			vec![]
		],
		vec![
			vec!["HM"],
			vec!["E", "HG", "LG", "LM"],
			vec![]
		],
		vec![
			vec![],
			vec!["E", "HG", "LG", "HM", "LM"],
			vec![]
		]
	]);

	let state = vec![
		vec!["HG", "HM"],
		vec!["E", "LG"],
		vec!["LM"]
	];
	let states = generate_states(&state);
	assert_eq!(states.len(), 2);
	assert_eq!(states, vec![
		vec![
			vec!["HG", "HM"],
			vec![],
			vec!["E", "LM", "LG"]
		],
		vec![
			vec!["E", "HG", "HM", "LG"],
			vec![],
			vec!["LM"]
		]
	]);

	let state = vec![
		vec!["HG", "HM"],
		vec!["LG"],
		vec!["E", "LM"]
	];
	let states = generate_states(&state);
	assert_eq!(states.len(), 1);
	assert_eq!(states, vec![
		vec![
			vec!["HG", "HM"],
			vec!["E", "LG", "LM"],
			vec![]
		]
	]);

	let state = vec![
		vec!["HG", "HM"],
		vec!["LG"],
		vec!["E", "FM"]
	];
	let states = generate_states(&state);
	assert_eq!(states.len(), 0);
	assert_eq!(states, vec![] as Vec<Vec<Vec<&str>>>);
}

#[test]
fn test_part1() {
	let state = vec![
		vec![],
		vec![],
		vec![],
		vec!["E","HG","HM","LG","LM"],
	];
	assert_eq!(part1(state), 0);

	let state = vec![
		vec![],
		vec![],
		vec!["E","HM","LM"],
		vec!["HG","LG"],
	];
	assert_eq!(part1(state), 1);

	let state = vec![
		vec![],
		vec![],
		vec!["HM"],
		vec!["E","HG","LG","LM"],
	];
	assert_eq!(part1(state), 2);

	let state = vec![
		vec![],
		vec![],
		vec!["E","HG","HM","LG"],
		vec!["LM"],
	];
	assert_eq!(part1(state), 3);

	let state = vec![
		vec![],
		vec![],
		vec!["HG","LG"],
		vec!["E","HM","LM"],
	];
	assert_eq!(part1(state), 4);

	let state = vec![
		vec![],
		vec![],
		vec!["E","HG","HM","LG","LM"],
		vec![],
	];
	assert_eq!(part1(state), 5);

	let state = vec![
		vec![],
		vec!["E", "HM", "LM"],
		vec!["HG","LG"],
		vec![],
	];
	assert_eq!(part1(state), 6);

	let state = vec![
		vec!["E", "HM", "LM"],
		vec![],
		vec!["HG", "LG"],
		vec![],
	];
	assert_eq!(part1(state), 7);

	let state = vec![
		vec!["LM"],
		vec![],
		vec!["E", "HG", "HM", "LG"],
		vec![],
	];
	assert_eq!(part1(state), 9);

	let state = vec![
		vec!["LM"],
		vec!["E", "HG", "HM"],
		vec!["LG"],
		vec![],
	];
	assert_eq!(part1(state), 10);

	let state = vec![
		vec!["E", "HM", "LM"],
		vec!["HG"],
		vec!["LG"],
		vec![]
	];
	assert_eq!(part1(state), 11);
}
