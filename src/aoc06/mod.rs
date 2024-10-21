#[cfg(test)]
mod tests;

use std::collections::HashMap;

pub fn aoc06() {
	println!("\nDay 6: Signals and Noise");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/06/input.txt")
		.expect("Error reading file");

	/* Part 1 */
	println!("Part 1:\n{}", part1(&input));

	/* Part 2 */
	println!("Part 2:\n{}", part2(&input));
}

fn part1(input: &str) -> String {
	let data = input.lines().collect::<Vec<_>>();
	let dictionaries = dictionaries(&data);
	dict_max_char(dictionaries)
}

type Counter = HashMap<char, u32>;

fn dictionaries(data: &Vec<&str>) -> Vec<Counter> {
	let mut counters = generate_dictionaries(data[0].len());
	for line in data {
		for (i, c) in line.chars().enumerate() {
			*counters[i].entry(c).or_insert(0) += 1;
		}
	}
	counters
}

fn generate_dictionaries(n: usize) -> Vec<Counter> {
	(0..n).map(|_| Counter::new()).collect()
}

fn max_char(counter: &Counter) -> char {
	*counter.iter().max_by_key(|&(_, count)| count).expect("Empty counter").0
}

fn dict_max_char(dictionaries: Vec<Counter>) -> String {
	dictionaries
		.iter()
		.map(|counter| max_char(counter))
		.collect()
}

/* Part 2 */

fn min_char(counter: &Counter) -> char {
	*counter.iter().min_by_key(|&(_, count)| count).expect("Empty counter").0
}

fn dict_min_char(dictionaries: Vec<Counter>) -> String {
	dictionaries
		.iter()
		.map(|counter| min_char(counter))
		.collect()
}

fn part2(input: &str) -> String {
	let data = input.lines().collect::<Vec<_>>();
	let dictionaries = dictionaries(&data);
	dict_min_char(dictionaries)
}
