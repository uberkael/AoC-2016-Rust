#![allow(dead_code)]

#[cfg(test)]
mod tests;

pub fn aoc19() {
    println!("\nDay 19: An Elephant Named Joseph");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

		let input: usize = std::fs::read_to_string("input/19/input.txt").unwrap().trim().parse().unwrap();

		println!("Part 1:\n{}", part1(input));
		println!("Part 1:\n{}", part1(5));
		println!("Part 1:\n{}", part1(10));
}

#[derive(Debug, Clone, Copy)]
struct Elf {
	id: usize,
	presents: usize,
}

fn generate_elves(count: usize) -> Vec<Elf> {
	let mut elves = vec![];
	for i in 0..count {
		elves.push(Elf { id: i + 1, presents: 1 });
	}
	elves
}

fn round(mut party: Vec<Elf>) -> Vec<Elf> {
	if party.len() < 10 {
		println!("{:?}", party);
	}
	if party.len() == 1 {
		return party;
	}
	if party.len() == 2 {
		party[0].presents += party[1].presents;
		return vec![party[0]];
	}
	let mut new_party = vec![];
	new_party.push(party[party.len() - 1]);
	for i in 0..party.len() - 1 {
		if party[i].presents == 0 {
			continue;
		}
		party[i].presents += party[i + 1].presents;
		party[i + 1].presents = 0;
		new_party.push(party[i]);
	}
	new_party
}

fn play(party: Vec<Elf>) -> usize {
	let mut party = party;
	loop {
		party = round(party);
		if party.len() == 1 {
			// println!("{:?}", party);
			return party[0].id;
		}
	}
}

fn part1(input: usize) -> usize {
	let party = generate_elves(input);
	play(party)
}
