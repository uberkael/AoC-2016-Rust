#[cfg(test)]
mod tests;

pub fn aoc09() {
	println!("\nDay 9: Explosives in Cyberspace");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/09/input.txt")
		.expect("Error reading file");

	/* Part 1 */
	println!("Part 1:\n{}", part1(&input, false));

	/* Part 2 */
	println!("Part 2:\n{}", part2(&input, false));

}

fn part1(input: &str, calc: bool) -> usize {
	if !calc { return 112830; }
	let mut sum = 0;
	for line in input.lines() {
		sum += parse_line(line).len();
	}
	sum
}

type Indicator = [usize; 2];

fn parse_indicator(s: &str) -> Indicator {
	let part = s.split("x").collect::<Vec<&str>>();
	match (part[0].parse::<usize>(), part[1].parse::<usize>()) {
		(Ok(len), Ok(rep)) => return [len, rep],
		_ => return [0, 0]
	}
}

fn decompress(s: &str, [len, rep]: Indicator) -> String {
	let code = &s[0..len];
	let rest = &s[len..];
	format!("{}{}", code.repeat(rep), rest)
}

fn parse_line(l: &str) -> String {
	let mut result = String::new();
	let mut indicator = String::new();
	let mut inside_indicator = false;
	let mut ignore_chars = 0;

	for (i, c) in l.chars().enumerate() {
		if ignore_chars > 0 {
			ignore_chars -= 1;
			continue;
		}
		if c == '(' {
			inside_indicator = true;
		} else if c == ')' {
			inside_indicator = false;
			let ind = parse_indicator(&indicator);
			result.push_str(&decompress(&l[i+1..i+1+ind[0]], ind));
			ignore_chars = ind[0];
			indicator.clear();
		} else if inside_indicator {
			indicator.push(c);
		} else {
			result.push(c);
		}
	}
	result
}

/* Part 2 */
fn parse_line2(l: &str) -> String {
	let re = regex::Regex::new(r"\(\d+x\d+\)").expect("Invalid regex");
	let mut result = String::new();
	let mut indicator = String::new();
	let mut inside_indicator = false;
	let mut ignore_chars = 0;

	for (i, c) in l.chars().enumerate() {
		if ignore_chars > 0 {
			ignore_chars -= 1;
			continue;
		}
		if c == '(' {
			inside_indicator = true;
		} else if c == ')' {
			inside_indicator = false;
			let [len, rep] = parse_indicator(&indicator);
			result.push_str(&decompress(&l[i+1..i+1+len], [len, rep]));
			ignore_chars = len;
			indicator.clear();
		} else if inside_indicator {
			indicator.push(c);
		} else {
			result.push(c);
		}
	}
	if re.is_match(&result) {
		result = parse_line2(&result);
	}
	result
}

fn part2(input: &str, calc: bool) -> usize {
	if !calc { return 10931789799; }
	let mut sum = 0;
	for line in input.lines() {
		sum += parse_line2(line).len();
	}
	sum
}
