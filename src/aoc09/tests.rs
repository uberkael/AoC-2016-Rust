// cSpell:ignoreRegExp ".+?"
use super::*;

#[test]
fn test_parse_indicator() {
	assert_eq!(parse_indicator("10x2"), [10, 2]);
	assert_eq!(parse_indicator("1x2"), [1, 2]);
	assert_eq!(parse_indicator("2x1"), [2, 1]);
}

#[test]
fn test_decompress() {
	assert_eq!(decompress("BC", [1, 5]), "BBBBBC");
	assert_eq!(decompress("XYZ", [3, 3]), "XYZXYZXYZ");
	assert_eq!(decompress("BCD", [2, 2]), "BCBCD");
	assert_eq!(decompress("EFG", [2, 2]), "EFEFG");
	assert_eq!(decompress("(1x3)A", [6, 1]), "(1x3)A");
	assert_eq!(decompress("(3x3)ABCY", [8, 2]), "(3x3)ABC(3x3)ABCY");
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
