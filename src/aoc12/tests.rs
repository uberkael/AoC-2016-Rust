// cSpell:ignore njnz
use super::*;

#[test]
fn test_parse() {
	assert_eq!(instruction("cpy 41 a"), Instruction::Cpy(Arg::Val(41), Arg::Reg(0)));
	assert_eq!(instruction("inc a"),    Instruction::Inc(Arg::Reg(0)));
	assert_eq!(instruction("inc a"),    Instruction::Inc(Arg::Reg(0)));
	assert_eq!(instruction("dec a"),    Instruction::Dec(Arg::Reg(0)));
	assert_eq!(instruction("jnz a 2"),  Instruction::Jnz(Arg::Reg(0), Arg::Val(2)));
	assert_eq!(instruction("dec a"),    Instruction::Dec(Arg::Reg(0)));
}

#[test]
fn test_reader() {
	let input = "cpy 41 a\ninc a\ninc a\ndec a\njnz a 2\ndec a";
	let instructions = reader(input);
	assert_eq!(instructions, vec![
		Instruction::Cpy(Arg::Val(41), Arg::Reg(0)),
		Instruction::Inc(Arg::Reg(0)),
		Instruction::Inc(Arg::Reg(0)),
		Instruction::Dec(Arg::Reg(0)),
		Instruction::Jnz(Arg::Reg(0), Arg::Val(2)),
		Instruction::Dec(Arg::Reg(0)),
	]);
}
