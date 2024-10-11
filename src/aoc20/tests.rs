use super::*;

#[test]
fn test_ranges() {
	let ranges = ranges("5-8\n0-2\n4-7".to_string());
	assert_eq!(ranges.len(), 3);
	assert_eq!(ranges[0], (5, 8));
	assert_eq!(ranges[1], (0, 2));
	assert_eq!(ranges[2], (4, 7));

	let ranges = merge(ranges);
	assert_eq!(ranges.len(), 2);
}

#[test]
fn test_min() {
	let ranges = ranges("5-8\n0-2\n4-7".to_string());
	let merged = merge(ranges);
	assert_eq!(ip_min(&merged), 3);
}

#[test]
fn test_min2() {
	let ips = ranges("0-10000".to_string());
	assert_eq!(ip_min(&ips), 10001);
}
