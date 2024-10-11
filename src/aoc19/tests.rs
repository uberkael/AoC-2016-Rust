use super::*;

#[test]
fn test_josephus() {
	assert_eq!(josephus(5), 3);
	assert_eq!(josephus(10), 5);
}
