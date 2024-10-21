use std::collections::HashMap;

#[cfg(test)]
mod tests;

pub fn aoc10() {
	println!("\nDay 10: Balance Bots");
	println!("━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/10/input.txt")
		.expect("Error reading file");
	let mut bots = bots_generate(&input);
	bots = bot_configure(bots, &input);

	/* Part 1 */
	println!("Part 1:\n{}", part1(bots.clone()));

	/* Part 2 */
	println!("Part 2:\n{}", part2(bots));
}

/* Part 1 */
type Chip = isize;

#[derive(Debug, Clone)]
struct Bot {
	to: [To; 2],
	values: Vec<Chip>,
}

impl Bot {
	fn new() -> Self {
		Self {
			to: [To::None, To::None],
			values: vec![],
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
enum To {
	None,
	Bot(usize),
	Output(usize),
}

fn part1(bots: HashMap<usize, Bot>) -> isize {
	bot_run(bots)
}

fn bots_generate(s: &str) -> HashMap<usize, Bot> {
	let mut bots = HashMap::new();
	for line in s.lines() {
		let mut split = line.split_whitespace();
		while let Some(word) = split.next() {
			if word == "bot" {
				let id = split
					.next()
					.unwrap_or_default()
					.parse::<usize>()
					.unwrap_or_default();
				bots.insert(id, Bot::new());
			}
		}
	}
	bots
}

fn bot_configure(mut bots: HashMap<usize, Bot>, s: &str) -> HashMap<usize, Bot> {
	for line in s.lines() {
		bots = command(bots, line);
	}
	bots
}

fn command(mut bots: HashMap<usize, Bot>, s: &str) -> HashMap<usize, Bot> {
	let mut parts = s.split_whitespace();
	if let Some(cmd) = parts.next() {
		match cmd {
			"value" => {
				let chip = parts.next().unwrap_or_default().parse::<isize>().unwrap_or_default();
				let bot = parts.nth(3).unwrap_or_default().parse::<usize>().unwrap_or_default();
				bots.insert(bot, bot_chip(bots[&bot].clone(), chip));
			},
			"bot" => {
				let bot = parts.next().unwrap_or_default().parse::<usize>().unwrap_or_default();
				let low = [
							parts.nth(3).unwrap_or_default(),
							parts.next().unwrap_or_default()
				];
				let high = [
							parts.nth(3).unwrap_or_default(),
							parts.next().unwrap_or_default()
				];
				bots.insert(bot, bot_outputs(bots[&bot].clone(), low, high));
			},
			_ => (),
		};
	}
	bots
}

fn bot_chip(mut b: Bot, chip: isize) -> Bot {
	b.values.push(chip);
	b
}

fn bot_outputs(mut b: Bot, low: [&str; 2], high: [&str; 2]) -> Bot {
	let low = match low[0] {
		"bot" => To::Bot(low[1].parse().unwrap_or_default()),
		"output" => To::Output(low[1].parse().unwrap_or_default()),
		_ => To::None,
	};
	let high = match high[0] {
		"bot" => To::Bot(high[1].parse().unwrap_or_default()),
		"output" => To::Output(high[1].parse().unwrap_or_default()),
		_ => To::None,
	};
	b.to = [low, high];
	b
}

fn bot_run(mut bots: HashMap<usize, Bot>) -> isize {
	let mut finished = false;
	while !finished {
		for (n, bot) in bots.clone() {
			if check_finish(&bot) {
				return n as isize;
			}
			if bot.values.len() == 2 {
				bots = bot_give(bots, n, bot);
			}
		}
		finished = bots.iter().all(|(_, bot)| bot.values.len() < 2);
	}
	-1
}

fn bot_give(mut bots: HashMap<usize, Bot>, n: usize, mut bot: Bot) -> HashMap<usize, Bot> {
	let low = bot.values.iter().min().unwrap_or(&0).clone();
	let high = bot.values.iter().max().unwrap_or(&0).clone();
	match bot.to[0] {
		To::Bot(n) => bots.insert(n, bot_chip(bots[&n].clone(), low)),
		To::Output(_) => None,
		To::None => None,
	};
	match bot.to[1] {
		To::Bot(n) => bots.insert(n, bot_chip(bots[&n].clone(), high)),
		To::Output(_) => None,
		To::None => None,
	};
	bot.values.clear();
	bots.insert(n, bot);
	bots
}

fn check_finish(bot: &Bot) -> bool {
	bot.values == [17, 61] || bot.values == [61, 17]
}

/* Part 2 */
fn part2(bots: HashMap<usize, Bot>) -> isize {
	bot_run2(bots)
}

fn bot_run2(mut bots: HashMap<usize, Bot>) -> isize {
	let mut outputs = [-1; 21];
	let mut finished = false;
	while !finished {
		for (n, bot) in bots.clone() {
			if bot.values.len() == 2 {
				(bots, outputs) = bot_give2(bots, n, bot, outputs);
			}
		}
		finished = bots.iter().all(|(_, bot)| bot.values.len() < 2);
	}
	return outputs.iter().take(3).product();
}

fn bot_give2(mut bots: HashMap<usize, Bot>, n: usize, mut bot: Bot, mut outputs: [Chip; 21]) -> (HashMap<usize, Bot>, [Chip; 21]) {
	let low = bot.values.iter().min().unwrap_or(&0).clone();
	let high = bot.values.iter().max().unwrap_or(&0).clone();
	match bot.to[0] {
		To::Bot(n) => {bots.insert(n, bot_chip(bots[&n].clone(), low)); ()},
		To::Output(n) => outputs[n] = low,
		To::None => (),
	};
	match bot.to[1] {
		To::Bot(n) => {bots.insert(n, bot_chip(bots[&n].clone(), high)); ()},
		To::Output(n) => outputs[n] = high,
		To::None => (),
	};
	bot.values.clear();
	bots.insert(n, bot);
	(bots, outputs)
}
