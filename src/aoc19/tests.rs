use super::*;

#[test]
fn test_generate_elves() {
	let party = generate_elves(5);
	assert_eq!(party.len(), 5);
	assert_eq!(party[0].id, 1);
	assert_eq!(party[0].presents, 1);
	assert_eq!(party[4].id, 5);
	assert_eq!(party[4].presents, 1);
}

#[test]
fn test_round() {
	let party = generate_elves(5);
	let party = round(party);
	assert_eq!(party.len(), 3);
	assert_eq!(party[0].id, 5);
	assert_eq!(party[0].presents, 1);
	assert_eq!(party[1].id, 1);
	assert_eq!(party[1].presents, 2);
	assert_eq!(party[2].id, 3);
	assert_eq!(party[2].presents, 2);

	let party = round(party);
	assert_eq!(party.len(), 2);
	assert_eq!(party[0].id, 3);
	assert_eq!(party[0].presents, 2);
	assert_eq!(party[1].id, 5);
	assert_eq!(party[1].presents, 3);

	let party = round(party);
	assert_eq!(party.len(), 1);
	assert_eq!(party[0].id, 3);
	assert_eq!(party[0].presents, 5);
}

#[test]
fn test_play() {
	let party = generate_elves(5);
	let winner = play(party);
	assert_eq!(winner, 3);
}
