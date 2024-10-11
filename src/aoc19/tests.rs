use super::*;

#[test]
fn test_josephus() {
	assert_eq!(josephus(5), 3);
	assert_eq!(josephus(10), 5);
}

#[test]
fn test_josephus_opposite() {
	assert_eq!(josephus_opposite(5), 2);
	assert_eq!(josephus_opposite(10), 1);
}
