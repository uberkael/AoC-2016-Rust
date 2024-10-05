use std::collections::HashMap;

pub fn aoc12() {
	println!("\nDay 12: Leonardo's Monorail");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/12/input.txt").unwrap();

	println!("Part 1:\n{}", part1(&input));

}

#[derive(Debug, PartialEq)]
enum Arg {
	Reg(char),
	Val(isize),
}

#[derive(Debug, PartialEq)]
enum Instruction {
	Cpy(Arg, char),
	Inc(char),
	Dec(char),
	Jnz(Arg, isize),
}

trait ParseShortcut {
	fn c(&self) -> char;
	fn p(&self) -> isize;
	fn a(&self) -> Arg;
}

impl ParseShortcut for &str {
	fn c(&self) -> char { self.chars().next().unwrap() }
	fn p(&self) -> isize { self.parse().unwrap_or_default() }
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
		"cpy" => Instruction::Cpy(parts[1].a(), parts[2].c()),
		"inc" => Instruction::Inc(parts[1].c()),
		"dec" => Instruction::Dec(parts[1].c()),
		"jnz" => Instruction::Jnz(parts[1].a(), parts[2].p()),
		_ => panic!("Invalid instruction"),
	}
}

fn execute(inst: &Instruction, registers: &mut HashMap<char, isize>) -> isize {
	match inst {
		Instruction::Cpy(src, dst) => {
			let value = match src {
				Arg::Reg(r) => *registers.get(r).unwrap_or(&0),
				Arg::Val(v) => *v,
			};
			registers.insert(*dst, value);
			1
		}
		Instruction::Inc(r) => {
			*registers.entry(*r).or_insert(0) += 1;
			1
		}
		Instruction::Dec(r) => {
			*registers.entry(*r).or_insert(0) -= 1;
			1
		}
		Instruction::Jnz(cond, offset) => {
			let value = match cond {
				Arg::Reg(r) => *registers.get(r).unwrap_or(&0),
				Arg::Val(v) => *v,
			};
			if value != 0 {
				*offset
			} else {
				1
			}
		}
	}
}

fn reader(input: &str) -> Vec<Instruction> {
	input.lines().map(|line| instruction(line)).collect()
}

fn part1(input: &str) -> usize {
	let instructions = reader(input);
	let mut registers: HashMap<char, isize> = HashMap::new();
	let mut ip: isize = 0;
	while (ip >= 0) && ((ip as usize) < instructions.len()) {
		let instruction = &instructions[ip as usize];
		ip += execute(instruction, &mut registers);
	}
	*registers.get(&'a').unwrap_or(&0) as usize
}
