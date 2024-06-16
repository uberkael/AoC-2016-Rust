#[cfg(test)]
mod tests;

pub fn aoc05() {
	println!("\nDay 5: How About a Nice Game of Chess?");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

	/* Part 1 */
	let password: String = part1();
	println!("Part:1\n{password}");

	/* Part 2 */
	let password: String = part2();
	println!("Part:2\n{password}");
}

fn part1() -> String {
	let data = generate_number("ojvtpuvg")
		.map(|s| format!("{:x}", md5::compute(s.as_bytes())))
		.filter(|s| check_zeros(s))
		.take(8)
		.collect::<Vec<_>>();

	let password: String = data.iter().map(|s| get_char(s)).collect();
	password
}

fn get_char(hash: &str) -> char {
	let mut chars = hash.chars();
	if let Some(c) = chars.nth(5) {
		return c;
	}
	0 as char
}

fn check_zeros(s: &str) -> bool {
	s.starts_with("00000")
}

fn generate_number(id: &str) -> impl Iterator<Item = String> + '_ {
	(0..).map(move |num| format!("{}{}", id, num))
}

/* Part 2 */

fn get_data(s: &str) -> (usize, char) {
	let chars = s.chars().collect::<Vec<_>>();
	let pos = chars[5].to_digit(16).unwrap_or_default() as usize;
	let c = chars[6];
	return (pos, c);
}

fn assign_char(password: &mut [char; 8], (pos, c): (usize, char)) {
	if pos < 8 && password[pos] == '_' {
		password[pos] = c;
	}
}

fn check_finished(password: &[char; 8]) -> bool {
	password.iter().all(|&c| c != '_')
}

fn part2() -> String {
	let mut password = ['_'; 8];
	for d in generate_number("ojvtpuvg")
		.map(|s| format!("{:x}", md5::compute(s.as_bytes())))
		.filter(|s| check_zeros(s))
		.map(|s| get_data(&s))
	{
		assign_char(&mut password, d);
		if check_finished(&password) {
			break;
		}
	}
	let password: String = password.iter().collect();
	password
}
