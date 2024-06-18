#[cfg(test)]
mod tests;

use regex::Regex;

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

fn check_palindrome(s: &str) -> bool {
	if s.len() != 4 { return false; }
	s[0..1] == s[3..4] && s[1..2] == s[2..3] && s[0..1] != s[1..2]
}

fn check_hypernet(s: &str) -> bool {
	let re_extract = Regex::new(r"\[(.*?)\]").expect("Invalid regex");
	for m in re_extract.find_iter(s) {
		let ss = m.as_str();
		let ss = &ss[1..ss.len()-1];
		for i in 0..ss.len()-3 {
			if check_palindrome(&ss[i..i+4]) {
				return true;
			}
		}
	}
	false
}

fn check_abba(s: &str) -> bool {
	let re_cut = Regex::new(r"\[.*?\]").expect("Invalid regex");
	let parts: Vec<&str> = re_cut.split(s).collect();
	for part in parts {
		for i in 0..part.len()-3 {
			if check_palindrome(&part[i..i+4]) {
				return true;
			}
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
