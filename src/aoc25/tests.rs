// cSpell:ignore njnz
use super::*;

#[test]
fn test_parse_out() {
	let inst = instruction("out a");
	assert_eq!(inst, Instruction::Out(Arg::Reg(0)));
	let inst = instruction("out 0");
	assert_eq!(inst, Instruction::Out(Arg::Val(0)));
	let inst = instruction("out 1");
	assert_eq!(inst, Instruction::Out(Arg::Val(1)));
}

#[test]
fn test_execute_out() {
	let instructions = vec![
		Instruction::Out(Arg::Val(1)),
		Instruction::Out(Arg::Val(0)),
		Instruction::Out(Arg::Val(1)),
	];
	let output = compute(instructions.clone(), 0);
	assert_eq!(output, vec![1, 0, 1]);
}
