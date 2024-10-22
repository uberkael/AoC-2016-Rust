use std::time::Instant;

mod aoc01;
mod aoc02;
mod aoc03;
mod aoc04;
mod aoc05;
mod aoc06;
mod aoc07;
mod aoc08;
mod aoc09;
mod aoc10;
mod aoc11;
mod aoc12;
mod aoc13;
mod aoc14;
mod aoc15;
mod aoc16;
mod aoc17;
mod aoc18;
mod aoc19;
mod aoc20;
mod aoc21;
mod aoc22;
mod aoc23;
mod aoc24;
mod aoc25;

fn main() {
	let mut times = vec![];
	let start = Instant::now();
	aoc01::aoc01(); // 1.3
	times.push(start.elapsed().as_micros());
	let start = Instant::now();
	aoc02::aoc02(); // 1.1
	times.push(start.elapsed().as_micros());
	let start = Instant::now();
	aoc03::aoc03(); // 1.8
	times.push(start.elapsed().as_micros());
	let start = Instant::now();
	aoc04::aoc04(); // 4.8
	times.push(start.elapsed().as_micros());
	let start = Instant::now();
	aoc05::aoc05(); // 1.1
	times.push(start.elapsed().as_micros());
	let start = Instant::now();
	aoc06::aoc06(); // 1.3
	times.push(start.elapsed().as_micros());
	let start = Instant::now();
	aoc07::aoc07(); // 1.4
	times.push(start.elapsed().as_micros());
	let start = Instant::now();
	aoc08::aoc08(); // 1.2
	times.push(start.elapsed().as_micros());
	let start = Instant::now();
	aoc09::aoc09(); // 1.1
	times.push(start.elapsed().as_micros());
	let start = Instant::now();
	aoc10::aoc10(); // 1.6
	times.push(start.elapsed().as_micros());
	let start = Instant::now();
	aoc11::aoc11(); // 4.5
	times.push(start.elapsed().as_micros());
	let start = Instant::now();
	aoc12::aoc12(); // 45
	times.push(start.elapsed().as_micros());
	let start = Instant::now();
	aoc13::aoc13(); // 1.1
	times.push(start.elapsed().as_micros());
	let start = Instant::now();
	aoc14::aoc14(); // 1
	times.push(start.elapsed().as_micros());
	let start = Instant::now();
	aoc15::aoc15(); // 1
	times.push(start.elapsed().as_micros());
	let start = Instant::now();
	aoc16::aoc16(); // 93
	times.push(start.elapsed().as_micros());
	let start = Instant::now();
	aoc17::aoc17(); // 108
	times.push(start.elapsed().as_micros());
	let start = Instant::now();
	aoc18::aoc18(); // 3
	times.push(start.elapsed().as_micros());
	let start = Instant::now();
	aoc19::aoc19(); // 1
	times.push(start.elapsed().as_micros());
	let start = Instant::now();
	aoc20::aoc20(); // 1.3
	times.push(start.elapsed().as_micros());
	let start = Instant::now();
	aoc21::aoc21(); // 1.2
	times.push(start.elapsed().as_micros());
	let start = Instant::now();
	aoc22::aoc22(); // 1.5
	times.push(start.elapsed().as_micros());
	let start = Instant::now();
	aoc23::aoc23(); // 1.1
	times.push(start.elapsed().as_micros());
	let start = Instant::now();
	aoc24::aoc24(); // 18
	times.push(start.elapsed().as_micros());
	let start = Instant::now();
	aoc25::aoc25(); // 19
	times.push(start.elapsed().as_micros());

	times.iter().enumerate().for_each(|(i, t)| {
		println!("Day {}:\t{:?}", i + 1, t);
	});
}
