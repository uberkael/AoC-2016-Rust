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

	/* Part 2 */
	let triangles = parse_input2(&input);
	let possible = triangles.iter().filter(|t| check_triangle(**t)).count();
	println!("Part 2:\n{possible}");

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


/* Part 2 */
fn parse_input2(input: &str) -> Vec<Triangle> {
	let mut result = Vec::new();
	for three_lines in input.lines().collect::<Vec<&str>>().chunks(3) {
		result.append(&mut parse_lines(three_lines));
	}
	result
}

fn parse_lines(lines: &[&str]) -> Vec<Triangle> {
	let mut sides : Vec<usize> = Vec::new();
	for line in lines {
		let mut line_sides : Vec<usize> = line
			.split_whitespace()
			.map(|x| x.parse::<usize>()
			.unwrap())
			.collect();
		sides.append(&mut line_sides);
	}
	let t1 = [sides[0], sides[3], sides[6]];
	let t2 = [sides[1], sides[4], sides[7]];
	let t3 = [sides[2], sides[5], sides[8]];
	vec![t1, t2, t3]
}
