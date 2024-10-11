use super::*;

#[test]
fn test_ranges() {
	assert_eq!(ranges("5-8\n0-2\n4-7".to_string()).len(), 4294967295);
}

#[test]
fn test_min() {
	let ips = ranges("5-8\n0-2\n4-7".to_string());
	assert_eq!(ip_min(&ips), 3);
}

#[test]
fn test_min2() {
	let ips = ranges("0-10000".to_string());
	assert_eq!(ip_min(&ips), 10001);
}
