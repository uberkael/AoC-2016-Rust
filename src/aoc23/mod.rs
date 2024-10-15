use std::collections::HashMap;

#[cfg(test)]
mod tests;

pub fn aoc23() {
	println!("\nDay 23: Safe Cracking");
	println!("━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/23/input.txt").unwrap();
	let instructions = reader(&input);
	println!("Part 1:\n{}", part1(instructions.clone(), 7));
	println!("Part 2:\n{}", part1(instructions.clone(), 12));
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
		_ => panic!("Instrucción inválida"),
	}
}

fn execute(inst: &Instruction, registers: &mut HashMap<char, isize>) -> isize {
	match inst {
		Instruction::Cpy(src, dst) => {
			if let Arg::Reg(dst_reg) = dst {
				let value = get_value(src, registers);
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
			let value = get_value(cond, registers);
			let jump = get_value(offset, registers);
			if value != 0 {
				jump
			} else {
				1
			}
		}
		Instruction::Tgl(_) => {
			panic!("Tgl se maneja externamente");
		}
	}
}

fn reader(input: &str) -> Vec<Instruction> {
	input.lines().map(|line| instruction(line)).collect()
}

fn toggle(inst: &Instruction) -> Instruction {
	match inst {
		Instruction::Inc(arg) => Instruction::Dec(*arg),
		Instruction::Dec(arg) => Instruction::Inc(*arg),
		Instruction::Tgl(arg) => Instruction::Inc(*arg),
		Instruction::Jnz(a, b) => Instruction::Cpy(*a, *b),
		Instruction::Cpy(a, b) => Instruction::Jnz(*a, *b),
	}
}

fn get_value(arg: &Arg, registers: &HashMap<char, isize>) -> isize {
	match arg {
		Arg::Val(v) => *v,
		Arg::Reg(r) => *registers.get(r).unwrap_or(&0),
	}
}

fn execute_toggle(
	ip: isize,
	registers: &HashMap<char, isize>,
	instructions: &mut Vec<Instruction>,
) -> isize {
	if let Instruction::Tgl(arg) = &instructions[ip as usize] {
		let offset = get_value(arg, registers);
		let target = ip + offset;
		if target >= 0 && (target as usize) < instructions.len() {
			instructions[target as usize] = toggle(&instructions[target as usize]);
		}
	}
	1
}

/// inc X
/// dec Y
/// jnz Y -2
/// X += Y; Y = 0;
fn try_sum(
	ip: isize,
	instructions: &Vec<Instruction>,
	registers: &mut HashMap<char, isize>,
) -> Option<isize> {
	if (ip + 2) as usize >= instructions.len() {
		return None;
	}
	if let Instruction::Inc(Arg::Reg(target)) = instructions[ip as usize] {
		if let Instruction::Dec(Arg::Reg(src)) = instructions[(ip + 1) as usize] {
			if let Instruction::Jnz(Arg::Reg(check), Arg::Val(offset)) =
				instructions[(ip + 2) as usize]
			{
				if check == src && offset == -2 {
					// Realizamos la suma optimizada.
					let val = *registers.get(&src).unwrap_or(&0);
					*registers.entry(target).or_insert(0) += val;
					registers.insert(src, 0);
					return Some(3);
				}
			}
		}
	}
	None
}

/// cpy X Y
/// inc A
/// dec Y
/// jnz Y -2
/// dec B
/// jnz B -5
/// a += X * B; Y = 0; B = 0;
fn try_multiplication(
	ip: isize,
	instructions: &Vec<Instruction>,
	registers: &mut HashMap<char, isize>,
) -> Option<isize> {
	if (ip + 5) as usize >= instructions.len() {
		return None;
	}
	if let Instruction::Cpy(src, Arg::Reg(reg_y)) = instructions[ip as usize] {
		if let Instruction::Inc(Arg::Reg(reg_a)) = instructions[(ip + 1) as usize] {
			if let Instruction::Dec(Arg::Reg(reg_y2)) = instructions[(ip + 2) as usize] {
				if reg_y != reg_y2 {
					return None;
				}
				if let Instruction::Jnz(Arg::Reg(reg_y3), Arg::Val(off1)) =
					instructions[(ip + 3) as usize]
				{
					if reg_y != reg_y3 || off1 != -2 {
						return None;
					}
					if let Instruction::Dec(Arg::Reg(reg_b)) = instructions[(ip + 4) as usize] {
						if let Instruction::Jnz(Arg::Reg(reg_b2), Arg::Val(off2)) =
							instructions[(ip + 5) as usize]
						{
							if reg_b != reg_b2 || off2 != -5 {
								return None;
							}
							let factor = get_value(&src, registers);
							let veces = *registers.get(&reg_b).unwrap_or(&0);
							*registers.entry(reg_a).or_insert(0) += factor * veces;
							registers.insert(reg_y, 0);
							registers.insert(reg_b, 0);
							return Some(6);
						}
					}
				}
			}
		}
	}
	None
}

fn part1(mut instructions: Vec<Instruction>, eggs: isize) -> usize {
	let mut registers: HashMap<char, isize> = HashMap::new();
	registers.insert('a', eggs);
	let mut ip: isize = 0;
	while ip >= 0 && (ip as usize) < instructions.len() {
		if let Some(jump) = try_multiplication(ip, &instructions, &mut registers) {
			ip += jump;
			continue;
		}
		if let Some(jump) = try_sum(ip, &instructions, &mut registers) {
			ip += jump;
			continue;
		}
		if let Instruction::Tgl(Arg::Reg(_)) = instructions[ip as usize] {
			ip += execute_toggle(ip, &registers, &mut instructions);
			continue;
		}
		ip += execute(&instructions[ip as usize], &mut registers);
	}
	*registers.get(&'a').unwrap_or(&0) as usize
}
