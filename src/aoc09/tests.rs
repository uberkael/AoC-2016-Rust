// cSpell:ignoreRegExp ".+?"
#![allow(deprecated)]
use super::*;

#[test]
fn test_parse_indicator() {
	assert_eq!(parse_indicator("10x2"), [10, 2]);
	assert_eq!(parse_indicator("1x2"), [1, 2]);
	assert_eq!(parse_indicator("2x1"), [2, 1]);
}

#[test]
fn test_decompress_simple() {
	assert_eq!(decompress_simple("BC", [1, 5]), "BBBBBC");
	assert_eq!(decompress_simple("XYZ", [3, 3]), "XYZXYZXYZ");
	assert_eq!(decompress_simple("BCD", [2, 2]), "BCBCD");
	assert_eq!(decompress_simple("EFG", [2, 2]), "EFEFG");
	assert_eq!(decompress_simple("(1x3)A", [6, 1]), "(1x3)A");
	assert_eq!(decompress_simple("(3x3)ABCY", [8, 2]), "(3x3)ABC(3x3)ABCY");
}

#[test]
fn test_decompress() {
	assert_eq!(decompress(b"BC", false), 2);
	assert_eq!(decompress(b"XYZ", false), 3);
	assert_eq!(decompress(b"BCD", false), 3);
	assert_eq!(decompress(b"EFG", false), 3);
	assert_eq!(decompress(b"(1x3)A", false), 3);
	assert_eq!(decompress(b"(3x3)ABCY", false), 10);
}

#[test]
fn test_decompress_recursive() {
	assert_eq!(decompress(b"(3x3)XYZ", true), 9);
	assert_eq!(decompress(b"X(8x2)(3x3)ABCY", true), 20); // Calculado recursivamente
	assert_eq!(decompress(b"(27x12)(20x12)(13x14)(7x10)(1x12)A", true), 241920);
	assert_eq!(decompress(b"(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN", true), 445);
}

#[test]
fn test_parse_line() {
	assert_eq!(parse_line("ADVENT"), "ADVENT");
	assert_eq!(parse_line("A(1x5)BC"), "ABBBBBC");
	assert_eq!(parse_line("(3x3)XYZ"), "XYZXYZXYZ");
	assert_eq!(parse_line("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG");
	assert_eq!(parse_line("(6x1)(1x3)A"), "(1x3)A");
	assert_eq!(parse_line("X(8x2)(3x3)ABCY"), "X(3x3)ABC(3x3)ABCY");
}

#[test]
fn test_len() {
	assert_eq!(parse_line("ADVENT").len(), 6);
	assert_eq!(parse_line("A(1x5)BC").len(), 7);
	assert_eq!(parse_line("(3x3)XYZ").len(), 9);
	assert_eq!(parse_line("A(2x2)BCD(2x2)EFG").len(), 11);
	assert_eq!(parse_line("(6x1)(1x3)A").len(), 6);
	assert_eq!(parse_line("X(8x2)(3x3)ABCY").len(), 18);
}

/* Part 2 */

#[test]
fn test_parse_line2() {
	assert_eq!(parse_line2("(3x3)XYZ"), "XYZXYZXYZ");
	assert_eq!(parse_line2("X(8x2)(3x3)ABCY"), "XABCABCABCABCABCABCY");
	assert_eq!(parse_line2("(27x12)(20x12)(13x14)(7x10)(1x12)A"), "A".repeat(241920));
	assert_eq!(parse_line2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN").len(), 445);
}

// Deprecated: These functions were used in older test versions

#[deprecated(note = "Use decompress function directly for length verification")]
fn parse_indicator(s: &str) -> [usize; 2] {
	let bytes = s.as_bytes();
	let mut i = 0;
	let mut amount = 0;
	while i < bytes.len() && bytes[i].is_ascii_digit() {
		amount = amount * 10 + (bytes[i] - b'0') as usize;
		i += 1;
	}
	i += 1; // skip 'x'
	let mut repeat = 0;
	while i < bytes.len() && bytes[i].is_ascii_digit() {
		repeat = repeat * 10 + (bytes[i] - b'0') as usize;
		i += 1;
	}
	[amount, repeat]
}

#[deprecated(note = "Use decompress function directly for length verification")]
fn decompress_simple(s: &str, [amount, repeat]: [usize; 2]) -> String {
	let prefix = &s[..amount];
	prefix.repeat(repeat) + &s[amount..]
}

#[deprecated(note = "Use decompress function directly for length verification")]
fn decompress_str(slice: &[u8], recurse: bool) -> String {
	let mut result = String::new();
	let mut index = 0;
	while index < slice.len() {
		if slice[index] == b'(' {
			let (amount, repeat, marker_end) = super::parse_marker(&slice[index..]);
			let segment_start = index + marker_end;
			let segment_end = segment_start + amount;
			let seg = if recurse {
				decompress_str(&slice[segment_start..segment_end], true)
			} else {
				String::from_utf8_lossy(&slice[segment_start..segment_end]).to_string()
			};
			for _ in 0..repeat {
				result.push_str(&seg);
			}
			index = segment_end;
		} else {
			result.push(slice[index] as char);
			index += 1;
		}
	}
	result
}

#[deprecated(note = "Use decompress function directly for length verification")]
fn parse_line(s: &str) -> String {
	decompress_str(s.as_bytes(), false)
}

#[deprecated(note = "Use decompress function directly for length verification")]
fn parse_line2(s: &str) -> String {
	decompress_str(s.as_bytes(), true)
}
