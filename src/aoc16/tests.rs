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
	assert_eq!(duplicate(0b1, 1), (0b100, 3));
	assert_eq!(duplicate(0b0, 1), (0b001, 3));
	assert_eq!(duplicate(0b11111, 5), (0b11111000000, 11));
	assert_eq!(duplicate(0b111100001010, 12), (0b1111000010100101011110000, 25));
	assert_eq!(duplicate(0b10000, 5), (0b10000011110, 11));
	assert_eq!(duplicate(0b10000011110, 11), (0b10000011110010000111110, 23));
}

#[test]
fn test_checksum() {
	assert_eq!(checksum(0b110010110100, 12), (0b110101, 6));
}

#[test]
fn test_valid_checksum() {
	assert_eq!(valid_checksum(0b110010110100, 12), 0b100);
	assert_eq!(valid_checksum(0b10000011110010000111, 20), 0b01100);
}

#[test]
fn test_process() {
	assert_eq!(process(0b10000, 5, 20), 0b01100);
}
