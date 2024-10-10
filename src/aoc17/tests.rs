use super::*;

#[test]
fn test_room_doors() {
	let passcode = b"hijkl";
	let room = Room::new(passcode.to_vec(), (0, 0));
	let directions = room.doors;
	assert_eq!(directions, [true, true, true, false]);
}

#[test]
fn test_room_check_limit() {
	let passcode = b"hijkl".to_vec();
	let room = Room::new(passcode, (0, 0));
	assert_eq!(room.check_limit(&Direction::Up), false);
	assert_eq!(room.check_limit(&Direction::Down), true);
	assert_eq!(room.check_limit(&Direction::Left), false);
	assert_eq!(room.check_limit(&Direction::Right), true);
}

#[test]
fn test_room_next() {
	let passcode = b"hijkl".to_vec();
	let room = Room::new(passcode, (0, 0));
	let up = room.next(Direction::Up);
	let down = room.next(Direction::Down).unwrap();
	let left = room.next(Direction::Left);
	let right = room.next(Direction::Right).unwrap();
	assert_eq!(up, None);
	assert_eq!(down.position, (0, 1));
	assert_eq!(down, Room::new(b"hijklD".to_vec(), (0, 1)));
	assert_eq!(left, None);
	assert_eq!(right.position, (1, 0));
	assert_eq!(right, Room::new(b"hijklR".to_vec(), (1, 0)));
}

#[test]
fn test_bfs() {
	let room = Room::new(b"ihgpwlah".to_vec(), (0, 0));
	let result = bfs(room);
	assert_eq!(result, "DDRRRD");
	let room = Room::new(b"kglvqrro".to_vec(), (0, 0));
	let result = bfs(room);
	assert_eq!(result, "DDUDRLRRUDRD");
	let room = Room::new(b"ulqzkmiv".to_vec(), (0, 0));
	let result = bfs(room);
	assert_eq!(result, "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
}
