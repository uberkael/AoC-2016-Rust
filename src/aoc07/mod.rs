#![allow(unused)]

#[cfg(test)]
mod tests;

use regex::Regex;

pub fn aoc07() {
	println!("\nDay 7: Internet Protocol Version 7");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/07/input.txt").unwrap();

	/* Parte 1 */
	println!("Part 1:\n{}", part1(&input, false));

	/* Parte 2 */
	println!("Part 2:\n{}", part2(&input, false));

}

fn part1(input: &str, calc: bool) -> usize {
	if !calc { return 118; }
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

/* Part 2 */

fn check_aba(s: &str) -> bool {
	if s.len() != 3 { return false; }
	s[0..1] == s[2..3] && s[0..1] != s[1..2]
}

fn check_ssl(s: &str) -> bool {
	let re_cut = Regex::new(r"\[.*?\]").expect("Invalid regex");
	let abas: Vec<&str> = re_cut.split(s).collect();
	let babs: Vec<&str> = re_cut
		.find_iter(s)
		.map(|m| { let ms = m.as_str(); &ms[1..ms.len()-1] })
		.collect();

	for aba in abas {
		for i in 0..aba.len()-2 {
			if check_aba(&aba[i..i+3]) {
				let ms_rev = inverse(&aba[i..i+3]);
				if babs.iter().any(|bab| bab.contains(&ms_rev)) {
					return true;
				}
			}
		}
	}
	false
}

fn inverse(s: &str) -> String {
	format!("{}{}{}", &s[1..2], &s[0..1], &s[1..2])
}

fn part2(input: &str, calc: bool) -> usize {
	if !calc { return 260; }
	input.lines().filter(|l| check_ssl(l)).count()
}
