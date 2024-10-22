use rayon::prelude::*;

#[cfg(test)]
mod tests;

pub fn aoc07() {
	println!("\nDay 7: Internet Protocol Version 7");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/07/input.txt").expect("Error reading file");

	println!("Part 1:\n{}", part1(&input));
	println!("Part 2:\n{}", part2(&input));
}

fn part1(input: &str) -> usize {
	input.par_lines().filter(|l| check_tls(l)).count()
}

fn check_palindrome(s: &str) -> bool {
	s.len() == 4
		&& s.as_bytes()[0] == s.as_bytes()[3]
		&& s.as_bytes()[1] == s.as_bytes()[2]
		&& s.as_bytes()[0] != s.as_bytes()[1]
}

fn check_hypernet(s: &str) -> bool {
	let mut inside_brackets = false;
	let mut start_index = 0;

	for (i, c) in s.chars().enumerate() {
		if c == '[' {
			inside_brackets = true;
			start_index = i;
		} else if c == ']' {
			inside_brackets = false;
			if check_palindrome(&s[start_index + 1..i]) {
				return true;
			}
		} else if inside_brackets && i >= start_index + 3 {
			if check_palindrome(&s[i - 3..i + 1]) {
				return true;
			}
		}
	}
	false
}

fn check_abba(s: &str) -> bool {
	let mut inside_brackets = false;
	let mut start_index = 0;

	for (i, c) in s.chars().enumerate() {
		if c == '[' {
			inside_brackets = true;
		} else if c == ']' {
			inside_brackets = false;
			start_index = i + 1;
		} else if !inside_brackets && i >= start_index + 3 {
			if check_palindrome(&s[i - 3..i + 1]) {
				return true;
			}
		}
	}
	false
}

fn check_tls(s: &str) -> bool {
	!check_hypernet(s) && check_abba(s)
}

/* Part 2 */
fn check_aba(s: &str) -> bool {
	s.len() == 3 && s.as_bytes()[0] == s.as_bytes()[2] && s.as_bytes()[0] != s.as_bytes()[1]
}

fn check_ssl(s: &str) -> bool {
	let mut abas = Vec::new();
	let mut babs = Vec::new();
	let mut inside_brackets = false;
	let mut start_index = 0;

	for (i, c) in s.chars().enumerate() {
		if c == '[' {
			inside_brackets = true;
			if i > 2 {
				abas.push(&s[start_index..i]);
			}
			start_index = i;
		} else if c == ']' {
			inside_brackets = false;
			babs.push(&s[start_index + 1..i]);
			start_index = i + 1;
		} else if !inside_brackets && i >= start_index + 2 {
			if check_aba(&s[i - 2..i + 1]) {
				let ms_rev = inverse(&s[i - 2..i + 1]);
				if babs.iter().any(|bab| bab.contains(&ms_rev)) {
					return true;
				}
			}
		}
	}

	if !inside_brackets && s.len() > start_index + 2 {
		abas.push(&s[start_index..]);
	}

	for aba in abas {
		for i in 0..aba.len() - 2 {
			if check_aba(&aba[i..i + 3]) {
				let ms_rev = inverse(&aba[i..i + 3]);
				if babs.iter().any(|bab| bab.contains(&ms_rev)) {
					return true;
				}
			}
		}
	}
	false
}

fn inverse(s: &str) -> String {
	format!(
		"{}{}{}",
		s.as_bytes()[1] as char,
		s.as_bytes()[0] as char,
		s.as_bytes()[1] as char
	)
}

fn part2(input: &str) -> usize {
	input.par_lines().filter(|l| check_ssl(l)).count()
}
