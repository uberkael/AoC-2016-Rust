// cSpell:ignoreRegExp ".*"
use super::*;

#[test]
fn test_toggle() {
	let inst = instruction("tgl a");
	assert_eq!(inst, Instruction::Tgl(Arg::Reg(0)));
}

#[test]
fn test_execute_toggle() {
	let mut instructions = vec![
		Instruction::Inc(Arg::Reg(0)),
		Instruction::Tgl(Arg::Reg(0)),
		Instruction::Inc(Arg::Reg(0)),
	];
	let registers = [0; 4];
	let ip = 1;
	let new_ip = execute_toggle(ip, &registers, &mut instructions);
	assert_eq!(new_ip, 2);
	assert_eq!(instructions, vec![
		Instruction::Inc(Arg::Reg(0)),
		Instruction::Inc(Arg::Reg(0)),
		Instruction::Inc(Arg::Reg(0)),
	]);
}
