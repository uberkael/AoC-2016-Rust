use super::*;

#[test]
fn test_invert_reverse() {
	assert_eq!(invert_reverse(0b1, 1), 0b0);
	assert_eq!(invert_reverse(0b0, 1), 0b1);
	assert_eq!(invert_reverse(0b11111, 5), 0b00000);
	assert_eq!(invert_reverse(0b111100001010, 12), 0b101011110000);
	assert_eq!(invert_reverse(0b10000, 5), 0b11110);
	assert_eq!(invert_reverse(0b10000011110, 11), 0b10000111110);
}

#[test]
fn test_duplicate() {
	assert_eq!(duplicate(&vec![1]), vec![1, 0, 0]);
	assert_eq!(duplicate(&vec![0]), vec![0, 0, 1]);
	assert_eq!(duplicate(&vec![1, 1, 1, 1, 1]), vec![1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0]);
	assert_eq!(duplicate(&vec![1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0]), vec![1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0]);
	assert_eq!(duplicate(&vec![1, 0, 0, 0, 0]), vec![1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0]);
	assert_eq!(duplicate(&vec![1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0]), vec![1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0]);
}

#[test]
fn test_checksum() {
	assert_eq!(checksum(&vec![1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0]), vec![1, 1, 0, 1, 0, 1]);
}

#[test]
fn test_valid_checksum() {
	assert_eq!(valid_checksum(vec![1,1,0,0,1,0,1,1,0,1,0,0]), vec![1, 0, 0]);
	assert_eq!(valid_checksum(vec![1,0,0,0,0,0,1,1,1,1,0,0,1,0,0,0,0,1,1,1]), vec![0, 1, 1, 0, 0]);
}

#[test]
fn test_process() {
	assert_eq!(process("10000", 20), vec![0, 1, 1, 0, 0]);
}
