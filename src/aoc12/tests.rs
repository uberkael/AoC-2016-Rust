use super::*;

#[test]
fn test_parse() {
	assert_eq!(instruction("cpy 41 a"), Instruction::Cpy(41, 'a'));
	assert_eq!(instruction("inc a"),    Instruction::Inc('a'));
	assert_eq!(instruction("inc a"),    Instruction::Inc('a'));
	assert_eq!(instruction("dec a"),    Instruction::Dec('a'));
	assert_eq!(instruction("jnz a 2"),  Instruction::Jnz('a', 2));
	assert_eq!(instruction("dec a"),    Instruction::Dec('a'));
}

#[test]
fn test_reader() {
	let input = "cpy 41 a\ninc a\ninc a\ndec a\njnz a 2\ndec a";
	let instructions = reader(input);
	assert_eq!(instructions, vec![
		Instruction::Cpy(41, 'a'),
		Instruction::Inc('a'),
		Instruction::Inc('a'),
		Instruction::Dec('a'),
		Instruction::Jnz('a', 2),
		Instruction::Dec('a'),
	]);
}
