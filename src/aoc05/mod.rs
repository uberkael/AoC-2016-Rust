#[cfg(test)]
mod tests;

pub fn aoc05() {
	println!("\nDay 5: How About a Nice Game of Chess?");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

	/* Part 1 */

	let input = generate_number("ojvtpuvg")
		.map(|s| format!("{:x}", md5::compute(s.as_bytes())))
		.filter(|s| check_zeros(s))
		.take(8)
		.collect::<Vec<_>>();

	let password: String = input.iter().map(|s| get_char(s)).collect();
	println!("Part:1\n{password}");
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
