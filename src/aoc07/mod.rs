#[cfg(test)]
mod tests;

use regex::Regex;
use fancy_regex::Regex as FRegex;

pub fn aoc07() {
	println!("\nDay 7: Internet Protocol Version 7");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/07/input.txt").unwrap();

	/* Parte 1 */
	println!("{}", part1(&input));

}

fn part1(input: &str) -> usize {
	input.lines().filter(|l| check_tls(l)).count()
}

fn check_hypernet(s: &str) -> bool {
	let re_hypernet = FRegex::new(r".*?(.)(?!\1)(.)\2\1.*").expect("Invalid regex");
	let re_extract = Regex::new(r"\[(.*?)\]").expect("Invalid regex");
	for m in re_extract.find_iter(s) {
		let ss = m.as_str();
		if re_hypernet.is_match(ss).unwrap_or_default() {
			return true;
		}
	}
	false
}

fn check_abba(s: &str) -> bool {
	let re = FRegex::new(r"(.)(?!\1)(.)\2\1").expect("Invalid regex");
	let re_cut = Regex::new(r"\[.*?\]").expect("Invalid regex");

	let parts: Vec<&str> = re_cut.split(s).collect();
	for part in parts {
		if re.is_match(part).unwrap_or_default() {
			return true;
		}
	}
	false
}

fn check_tls(s: &str) -> bool {
	if check_hypernet(s) {
		return false;
	}
	check_abba(s)
}
