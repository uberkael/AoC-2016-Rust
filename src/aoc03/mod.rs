#[cfg(test)]
mod tests;

pub fn aoc03() {
	println!("\nDay 3: Squares With Three Sides");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/03/input.txt").unwrap();

	/* Part 1 */
	let triangles = parse_input(&input);
	let possible = triangles.iter().filter(|t| check_triangle(**t)).count();
	println!("Part 1:\n{possible}");

}

type Triangle = [usize; 3];

fn parse_input(input: &str) -> Vec<Triangle> {
	input.lines().map(|x| parse_triangle(x)).collect()
}

fn parse_triangle(s: &str) -> Triangle {
	let mut sides = s.split_whitespace().map(|x| x.parse::<usize>().unwrap());
	[sides.next().unwrap(), sides.next().unwrap(), sides.next().unwrap()]
}

fn check_triangle(t: Triangle) -> bool {
	let max = *t.iter().max().unwrap();
	let sum: usize = t.iter().sum();
	sum - max > max
}
