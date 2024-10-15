// #![allow(dead_code)]

use std::collections::HashMap;

#[cfg(test)]
mod tests;

pub fn aoc23() {
	println!("\nDay 23: Safe Cracking");
	println!("━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/23/input.txt").unwrap();

	let instructions = reader(&input);
	println!("Part 1:\n{}", part1(instructions.clone(), 7));
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Arg {
	Reg(char),
	Val(isize),
}

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
	Cpy(Arg, Arg),
	Inc(Arg),
	Dec(Arg),
	Jnz(Arg, Arg),
	Tgl(Arg),
}

trait ParseShortcut {
	fn c(&self) -> char;
	fn a(&self) -> Arg;
}

impl ParseShortcut for &str {
	fn c(&self) -> char {
		self.chars().next().unwrap()
	}
	fn a(&self) -> Arg {
		if let Ok(val) = self.parse() {
			Arg::Val(val)
		} else {
			Arg::Reg(self.c())
		}
	}
}

fn instruction(s: &str) -> Instruction {
	let parts: Vec<&str> = s.split_whitespace().collect();
	match parts[0] {
		"cpy" => Instruction::Cpy(parts[1].a(), parts[2].a()),
		"inc" => Instruction::Inc(parts[1].a()),
		"dec" => Instruction::Dec(parts[1].a()),
		"jnz" => Instruction::Jnz(parts[1].a(), parts[2].a()),
		"tgl" => Instruction::Tgl(parts[1].a()),
		_ => panic!("Invalid instruction"),
	}
}

fn execute(inst: &Instruction, registers: &mut HashMap<char, isize>) -> isize {
	match inst {
		Instruction::Cpy(src, dst) => {
			if let Arg::Reg(dst_reg) = dst {
				let value = match src {
					Arg::Reg(r) => *registers.get(r).unwrap_or(&0),
					Arg::Val(v) => *v,
				};
				registers.insert(*dst_reg, value);
			}
			1
		}
		Instruction::Inc(arg) => {
			if let Arg::Reg(r) = arg {
				*registers.entry(*r).or_insert(0) += 1;
			}
			1
		}
		Instruction::Dec(arg) => {
			if let Arg::Reg(r) = arg {
				*registers.entry(*r).or_insert(0) -= 1;
			}
			1
		}
		Instruction::Jnz(cond, offset) => {
			let value = match cond {
				Arg::Reg(r) => *registers.get(r).unwrap_or(&0),
				Arg::Val(v) => *v,
			};
			let jump = match offset {
				Arg::Reg(r) => *registers.get(r).unwrap_or(&0),
				Arg::Val(v) => *v,
			};
			if value != 0 {
				jump
			} else {
				1
			}
		}
		Instruction::Tgl(_) => panic!("Tgl debe manejarse fuera de execute"), // No debería llegar aquí
	}
}

fn reader(input: &str) -> Vec<Instruction> {
	input.lines().map(|line| instruction(line)).collect()
}

fn toggle(inst: &Instruction) -> Instruction {
	match inst {
		Instruction::Inc(arg)  => Instruction::Dec(*arg),
		Instruction::Dec(arg)  => Instruction::Inc(*arg),
		Instruction::Tgl(arg)  => Instruction::Inc(*arg),
		Instruction::Jnz(a, b) => Instruction::Cpy(*a, *b),
		Instruction::Cpy(a, b) => Instruction::Jnz(*a, *b),
	}
}

fn execute_toggle(offset: isize, instructions: &mut Vec<Instruction>) -> isize {
	if offset >= 0 && (offset as usize) < instructions.len() {
		instructions[offset as usize] = toggle(&instructions[offset as usize]);
	}
	1
}

fn part1(mut instructions: Vec<Instruction>, eggs: isize) -> usize {
	let mut registers: HashMap<char, isize> = HashMap::new();
	registers.insert('a', eggs);
	let mut ip: isize = 0;
	while (ip >= 0) && ((ip as usize) < instructions.len()) {
		let instruction = &instructions[ip as usize];
		if let Instruction::Tgl(Arg::Reg(r)) = instruction {
			let r_val = *registers.get(r).unwrap_or(&0);
			ip += execute_toggle(ip + r_val, &mut instructions);
			continue;
		}
		ip += execute(instruction, &mut registers);
	}
	*registers.get(&'a').unwrap_or(&0) as usize
}
