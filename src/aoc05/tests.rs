use super::*;

#[test]
fn test1_md5() {
	let input = b"abc3231929";
	let digest = md5::compute(input);

	assert_eq!("00000155f8105dff7f56ee10fa9b9abd", format!("{:x}", digest));
}

#[test]
fn test1_check_zeros() {
	assert!(check_zeros("00000f9a2c309875e05c5a5d09f1b8c4"));
	assert!(!check_zeros("0000f9a2c309875e05c5a5d09f1b8c4"));
}

#[test]
fn test1_get_char() {
	assert_eq!('1', get_char("00000155f8105dff7f56ee10fa9b9abd"));
}

#[test]
fn test1_1() {
	let hash = format!("{:x}", md5::compute(b"abc3231929"));

	assert_eq!("00000155f8105dff7f56ee10fa9b9abd", hash);
	assert_eq!('1', get_char(&hash));
}

#[test]
fn test1_2() {
	let hash = format!("{:x}", md5::compute(b"abc5017308"));

	assert_eq!("000008f82c5b3924a1ecbebf60344e00", hash);
	assert_eq!('8', get_char(&hash));
}

#[test]
fn test1_3() {
	let hash = format!("{:x}", md5::compute(b"abc5278568"));

	assert_eq!("00000f9a2c309875e05c5a5d09f1b8c4", hash);
	assert_eq!('f', get_char(&hash));
}

#[test]
fn test1_generator() {
	let input = generate_number("abc").take(5).collect::<Vec<_>>();
	assert_eq!(vec!["abc0", "abc1", "abc2", "abc3", "abc4"], input);

	let input = generate_number("abc").nth(5000).expect("No value");
	assert_eq!("abc5000", input);
}

#[ignore]
#[test]
fn test1_generator_hash() {
	let input = generate_number("abc")
	.map(|s| format!("{:x}", md5::compute(s.as_bytes())))
	.filter(|s| check_zeros(s))
	.take(8)
	.collect::<Vec<_>>();

	assert_eq!(8, input.len());

	assert_eq!(vec![
		"00000155f8105dff7f56ee10fa9b9abd",
		"000008f82c5b3924a1ecbebf60344e00",
		"00000f9a2c309875e05c5a5d09f1b8c4",
		"000004e597bd77c5cd2133e9d885fe7e",
		"0000073848c9ff7a27ca2e942ac10a4c",
		"00000a9c311683dbbf122e9611a1c2d4",
		"000003c75169d14fdb31ec1593915cff",
		"0000000ea49fd3fc1b2f10e02d98ee96"
	], input);

}

#[ignore]
#[test]
fn part1() {
	let input = generate_number("abc")
	.map(|s| format!("{:x}", md5::compute(s.as_bytes())))
	.filter(|s| check_zeros(s))
	.take(8)
	.collect::<Vec<_>>();

	let password: String = input.iter().map(|s| get_char(s)).collect();
	assert_eq!("18f47a30", password);
}

/* Part 2 */
#[test]
fn part2_get_data() {
	assert_eq!((0,  '0'), get_data("000000000"));
	assert_eq!((1,  '5'), get_data("00000155f8105dff7f56ee10fa9b9abd"));
	assert_eq!((8,  'f'), get_data("000008f82c5b3924a1ecbebf60344e00"));
	assert_eq!((15, '9'), get_data("00000f9a2c309875e05c5a5d09f1b8c4"));
	assert_eq!((4,  'e'), get_data("000004e597bd77c5cd2133e9d885fe7e"));
	assert_eq!((7,  '3'), get_data("0000073848c9ff7a27ca2e942ac10a4c"));
	assert_eq!((10, '9'), get_data("00000a9c311683dbbf122e9611a1c2d4"));
	assert_eq!((3,  'c'), get_data("000003c75169d14fdb31ec1593915cff"));
	assert_eq!((0,  '0'), get_data("0000000ea49fd3fc1b2f10e02d98ee96"));
}

#[test]
fn part2_assign_char() {
	let mut password = ['_'; 8];
	assign_char(&mut password, (0, '0'));
	assert_eq!(['0', '_', '_', '_', '_', '_', '_', '_'], password);

	assign_char(&mut password, (1, '1'));
	assert_eq!(['0', '1', '_', '_', '_', '_', '_', '_'], password);

	assign_char(&mut password, (8, '8'));
	assert_eq!(['0', '1', '_', '_', '_', '_', '_', '_'], password);

	assign_char(&mut password, (7, '7'));
	assert_eq!(['0', '1', '_', '_', '_', '_', '_', '7'], password);
}

#[test]
fn part2_check_finished() {
	let password = ['0', '1', '2', '3', '4', '5', '6', '7'];
	assert!(check_finished(&password));

	let password = ['0', '1', '2', '3', '4', '5', '6', '_'];
	assert!(!check_finished(&password));
}
