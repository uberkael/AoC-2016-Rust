use super::*;

#[test]
fn test_generate_hashes() {
	let hashes = generate_hashes::<5>("abc");
	assert_eq!(hashes.len(), 5);
	assert_eq!(hashes[0], "577571be4de9dcce85a041ba0410f29f");
	assert_eq!(hashes[1], "23734cd52ad4a4fb877d8a1e26e5df5f");
	assert_eq!(hashes[2], "63872b5565b2179bd72ea9c339192543");
	assert_eq!(hashes[3], "8a8b3aea9e3ca257a31cf91db6d6ba12");
	assert_eq!(hashes[4], "1ab3168356216e0c9d97d7bca8ff190b");
}

#[test]
fn test_triple() {
	let hashes = generate_hashes::<20>("abc");
	assert_eq!(hashes[18], "0034e0923cc38887a57bd7b1d4f953df");
	if let Some(c) = check_triple(&hashes[18]) {
		assert_eq!(c, 56);
	} else {
		assert!(false);
	}
}

#[test]
fn test_quintuple() {
	let hashes = generate_hashes::<900>("abc");
	assert_eq!(hashes[39], "347dac6ee8eeea4652c7476d0f97bee5");
	assert_eq!(hashes[816], "3aeeeee1367614f3061d165a5fe3cac3");
	println!("{:?}", 101 as char);
	assert_eq!(check_quintuples(&hashes[816]), [101]);
	assert_eq!(check_quintuples(&hashes[816])[0] as char == 'e', true);
}

#[test]
fn test_find_key() {
	let hashes = generate_hashes::<1000>("abc");
	assert_eq!(find_keys(&hashes, 1)[0], 39);
}

#[test]
fn test_find_keys() {
	let hashes = generate_hashes::<1000>("abc");
	let keys = find_keys(&hashes, 64);
	assert_eq!(keys.len(), 8);
	assert_eq!(keys[0], 39);
	assert_eq!(keys[1], 92);
	assert_eq!(keys[2], 110);
	assert_eq!(keys[3], 184);
	assert_eq!(keys[4], 314);
	assert_eq!(keys[5], 459);
	assert_eq!(keys[6], 461);
	assert_eq!(keys[7], 771);
}

#[test]
fn test_key_stretching() {
	let hashes = key_stretching::<1>("abc");
	assert_eq!(hashes.len(), 1);
	assert_eq!(hashes[0], "a107ff634856bb300138cac6568c0f24");
}
