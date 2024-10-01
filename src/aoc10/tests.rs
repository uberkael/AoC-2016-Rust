use super::*;

#[test]
fn test_bot_generate_1() {
	let b = bots_generate("bot 2");
	assert_eq!(b.len(), 1);
}

#[test]
fn test_bot_generate_line() {
	let b = bots_generate("bot 2 gives low to bot 1 and high to bot 0");
	assert_eq!(b.len(), 3);
}

#[test]
fn test_bot_generate() {
	let b = bots_generate("value 5 goes to bot 2
	bot 2 gives low to bot 1 and high to bot 0
	value 3 goes to bot 1
	bot 1 gives low to output 1 and high to bot 0
	bot 0 gives low to output 2 and high to output 0
	value 2 goes to bot 2");
	assert_eq!(b.len(), 3);
}

#[test]
fn test_bot_outputs() {
	let b = Bot::new();
	let b = bot_outputs(b, ["output", "2"], ["output", "0"]);
	assert_eq!(b.to, [To::Output(2), To::Output(0)]);
	let b = bot_outputs(b, ["output", "1"], ["bot", "0"]);
	assert_eq!(b.to, [To::Output(1), To::Bot(0)]);
}

#[test]
fn test_bot_chip() {
	let b = Bot::new();
	let b = bot_chip(b, 5);
	let b = bot_chip(b, 3);
	assert_eq!(b.values, [5, 3]);
}

#[test]
fn test_command() {
	let bots = bots_generate("value 5 goes to bot 2
	bot 2 gives low to bot 1 and high to bot 0
	value 3 goes to bot 1
	bot 1 gives low to output 1 and high to bot 0
	bot 0 gives low to output 2 and high to output 0
	value 2 goes to bot 2");
	let b = command(bots, "value 5 goes to bot 2");
	assert_eq!(b[&2].values, [5]);
	let b = command(b, "value 6 goes to bot 2");
	assert_eq!(b[&2].values, [5, 6]);
	let b = command(b, "bot 2 gives low to bot 1 and high to bot 0");
	assert_eq!(b[&2].to, [To::Bot(1), To::Bot(0)]);
	let b = command(b, "bot 1 gives low to output 2 and high to bot 1");
	assert_eq!(b[&1].to, [To::Output(2), To::Bot(1)]);
}

#[test]
fn test_bot_configure() {
	let input = "value 5 goes to bot 2
	bot 2 gives low to bot 1 and high to bot 0
	value 3 goes to bot 1
	bot 1 gives low to output 1 and high to bot 0
	bot 0 gives low to output 2 and high to output 0
	value 2 goes to bot 2";
	let mut bots = bots_generate(input);
	bots = bot_configure(bots, input);
	assert_eq!(bots[&0].values, []);
	assert_eq!(bots[&1].values, [3]);
	assert_eq!(bots[&2].values, [5,  2]);
	assert_eq!(bots[&0].to, [To::Output(2), To::Output(0)]);
	assert_eq!(bots[&1].to, [To::Output(1), To::Bot(0)]);
	assert_eq!(bots[&2].to, [To::Bot(1),    To::Bot(0)]);
}
