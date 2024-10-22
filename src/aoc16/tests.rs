use super::*;

#[test]
fn test_process_single_zero() {
	// For a single '0' input, the block has 0 ones (even) so the checksum bit is '1'.
	let result = process("0", 1);
	assert_eq!(result, "1", "Checksum for input '0' (len 1) should be '1'");
}

#[test]
fn test_process_small_example() {
	// For the AoC Day 16 sample, starting with "10000" and filling to a length of 20
	// the expected checksum is "01100".
	let input = "10000";
	let result = process(input, 20);
	assert_eq!(result, "01100", "Checksum for input '10000' (len 20) should be '01100'");
}

#[test]
fn test_process_single_one() {
	// For a single '1' input, the block has 1 one (odd) so the checksum bit is '0'.
	let result = process("1", 1);
	assert_eq!(result, "0", "Checksum for input '1' (len 1) should be '0'");
}

#[test]
fn test_count_ones_at_level_zero() {
	// Test the count_ones function at level 0, where it should equal the prefix sum.
	let base_str = "10110";
	let base: Vec<bool> = base_str.trim().bytes().map(|b| b == b'1').collect();
	let n = base.len();
	let mut prefix = vec![0; n + 1];
	for i in 0..n {
		prefix[i + 1] = prefix[i] + (base[i] as usize);
	}

	// levels[0] is just the length of the base.
	let levels = vec![n];
	let count = count_ones(&levels, &base, &prefix, 0, n);
	assert_eq!(count, prefix[n], "At level 0, count_ones should equal the prefix sum");
}

#[test]
fn test_process() {
	assert_eq!(process("10000", 20), "01100");
	assert_eq!(process("10000011110010000111", 272), "10100010101101011");
}
