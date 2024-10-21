#[cfg(test)]
mod tests;

pub fn aoc23() {
	println!("\nDay 23: Safe Cracking");
	println!("━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/23/input.txt")
		.expect("Error reading the file");
	let instructions = reader(&input);

	println!("Part 1:\n{}", compute(instructions.clone(), 7));
	println!("Part 2:\n{}", compute(instructions, 12));
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Arg {
	Reg(usize),
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
		self.chars().next().expect("Empty string")
	}
	fn a(&self) -> Arg {
		if let Ok(val) = self.parse() {
			Arg::Val(val)
		} else {
			// a -> 0, b -> 1, c -> 2, d -> 3 ...
			Arg::Reg((self.c() as u8 - b'a') as usize)
		}
	}
}

fn reader(input: &str) -> Vec<Instruction> {
	input.lines().map(|line| instruction(line)).collect()
}

fn get_value(arg: &Arg, registers: &[isize; 4]) -> isize {
	match arg {
		Arg::Val(v) => *v,
		Arg::Reg(r) => registers[*r],
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

fn execute(ip: usize, inst: &Instruction, registers: &mut [isize; 4]) -> usize {
	match inst {
		Instruction::Cpy(src, dst) => {
			if let Arg::Reg(dst_reg) = dst {
				let value = get_value(src, registers);
				registers[*dst_reg] = value;
			}
			ip + 1
		}
		Instruction::Inc(arg) => {
			if let Arg::Reg(r) = arg {
				registers[*r] += 1;
			}
			ip + 1
		}
		Instruction::Dec(arg) => {
			if let Arg::Reg(r) = arg {
				registers[*r] -= 1;
			}
			ip + 1
		}
		Instruction::Jnz(cond, offset) => {
			let value = get_value(cond, registers);
			let jump = get_value(offset, registers);
			if value != 0 {
				if ip as isize + jump < 0 {
					0
				} else {
					(ip as isize + jump) as usize
				}
			} else {
				ip + 1
			}
		}
		Instruction::Tgl(_) => {
			panic!("Toggle not here");
		}
	}
}

fn execute_toggle(ip: usize, registers: &[isize; 4], instructions: &mut Vec<Instruction>) -> usize {
	if let Instruction::Tgl(arg) = &instructions[ip] {
		let offset = get_value(arg, registers);
		let target = (ip as isize + offset) as usize;
		if target < instructions.len() {
			instructions[target] = toggle(&instructions[target]);
		}
	}
	ip + 1
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

/// inc X
/// dec Y
/// jnz Y -2
/// X += Y; Y = 0;
fn try_sum(
    ip: usize,
    instructions: &Vec<Instruction>,
    registers: &mut [isize; 4],
) -> Option<usize> {
    if (ip + 2) >= instructions.len() { return None; }
    let Instruction::Inc(Arg::Reg(target)) = instructions[ip] else { return None; };
    let Instruction::Dec(Arg::Reg(src)) = instructions[ip + 1] else { return None; };
    let Instruction::Jnz(Arg::Reg(check), Arg::Val(offset)) = instructions[ip + 2] else { return None; };
    if check != src || offset != -2 { return None; }
    let val = registers[src];
    registers[target] += val;
    registers[src] = 0;
    Some(3)
}


/// cpy X Y
/// inc A
/// dec Y
/// jnz Y -2
/// dec B
/// jnz B -5
/// a += X * B; Y = 0; B = 0;
fn try_multiplication(
	ip: usize,
	instructions: &Vec<Instruction>,
	registers: &mut [isize; 4],
) -> Option<usize> {
	if (ip + 5) >= instructions.len() {
		return None;
	}
	let Instruction::Cpy(src, Arg::Reg(reg_y)) = instructions[ip] else { return None; };
	let Instruction::Inc(Arg::Reg(reg_a)) = instructions[ip + 1] else { return None; };
	let Instruction::Dec(Arg::Reg(reg_y2)) = instructions[ip + 2] else { return None; };
	if reg_y != reg_y2 { return None; }
	let Instruction::Jnz(Arg::Reg(reg_y3), Arg::Val(off1)) = instructions[ip + 3] else { return None; };
	if reg_y != reg_y3 || off1 != -2 { return None; }
	let Instruction::Dec(Arg::Reg(reg_b)) = instructions[ip + 4] else { return None; };
	let Instruction::Jnz(Arg::Reg(reg_b2), Arg::Val(off2)) = instructions[ip + 5] else { return None; };
	if reg_b != reg_b2 || off2 != -5 { return None; }
	let factor = get_value(&src, registers);
	let veces = registers[reg_b];
	registers[reg_a] += factor * veces;
	registers[reg_y] = 0;
	registers[reg_b] = 0;
	Some(6)
}

fn compute(mut instructions: Vec<Instruction>, eggs: isize) -> usize {
	let mut registers: [isize; 4] = [0; 4]; // Fixed array
	registers[0] = eggs; // 'a' is 0
	let mut ip = 0;
	while ip < instructions.len() {
		if let Some(jump) = try_multiplication(ip, &instructions, &mut registers) {
			ip += jump;
			continue;
		}
		if let Some(jump) = try_sum(ip, &instructions, &mut registers) {
			ip += jump;
			continue;
		}
		if let Instruction::Tgl(Arg::Reg(_)) = instructions[ip as usize] {
			ip = execute_toggle(ip, &registers, &mut instructions);
			continue;
		}
		ip = execute(ip, &instructions[ip], &mut registers);
	}
	registers[0] as usize
}
