// cSpell:ignoreRegExp ".+?"
use super::*;

#[test]
fn test1_room() {
	let input = "aaaaa-bbb-z-y-x-123[abxyz]";
	let room = Room::new(input);
	assert_eq!(room.name, "aaaaa-bbb-z-y-x");
	assert_eq!(room.sector, 123);
	assert_eq!(room.checksum, "abxyz");
}

#[test]
 fn test1_checksum() {
	let input = "aaaaa-bbb-z-y-x-123[abxyz]";
	let room = Room::new(input);
	let checksum = room.generate_checksum();
	assert_eq!(checksum, "abxyz");
}

#[test]
fn test1_1() {
	let input = "aaaaa-bbb-z-y-x-123[abxyz]";
	let room = Room::new(input);
	assert_eq!(room.generate_checksum(), "abxyz");
	assert!(room.is_real());
}

#[test]
fn test1_2() {
	let input = "a-b-c-d-e-f-g-h-987[abcde]";
	let room = Room::new(input);
	assert_eq!(room.generate_checksum(), "abcde");
	assert!(room.is_real());
}

#[test]
fn test1_3() {
	let input = "not-a-real-room-404[oarel]";
	let room = Room::new(input);
	assert_eq!(room.generate_checksum(), "oarel");
	assert!(room.is_real());
}

#[test]
fn test1_4() {
	let input = "totally-real-room-200[decoy]";
	let room = Room::new(input);
	let checksum = room.generate_checksum();
	assert_ne!(checksum, "decoy");
	assert!(!room.is_real());
}

/* Part 2 */

#[test]
fn test2_cypher() {
	assert_eq!(cipher('a', 1), 'b');
	assert_eq!(cipher('z', 1), 'a');
	assert_eq!(cipher('a', 26), 'a');
	assert_eq!(cipher('a', 27), 'b');
	assert_eq!(cipher('-', 1), ' ');
}

#[test]
fn test2_cypher2() {
	let input = "qzmt-zixmtkozy-ivhz";
	let name = input.chars().map(|c| cipher(c, 343)).collect::<String>();
	assert_eq!(name, "very encrypted name");
}

#[test]
fn test2_room() {
	let input = "qzmt-zixmtkozy-ivhz-343[abcde]";
	let room = Room::new(input);
	assert_eq!(room.decrypt(), "very encrypted name");
}
