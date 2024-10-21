#[cfg(test)]
mod tests;

pub fn aoc20() {
	println!("\nDay 20: Firewall Rules");
	println!("━━━━━━━━━━━━━━━━━━━━━━");

	let input = std::fs::read_to_string("input/20/input.txt")
		.expect("Error reading the file");

	let ranges = ranges(input.trim().to_string());
	let merged_ranges = merge(ranges);

	println!("Part 1:\n{}", part1(&merged_ranges));
	println!("Part 2:\n{}", part2(&merged_ranges));
}

fn ranges(input: String) -> Vec<(u32, u32)> {
	input.lines()
		.map(|line| {
			let parts: Vec<&str> = line.split('-').collect();
			let start = parts[0].parse::<u32>().expect("Error parsing input");
			let end = parts[1].parse::<u32>().expect("Error parsing input");
			(start, end)
		}).collect()
}

fn merge(mut ranges: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
	let mut merged = Vec::new();
	ranges.sort_by_key(|&(start, _)| start);
	let mut current_range = ranges[0];
	for &(start, end) in &ranges[1..] {
		if start <= current_range.1 + 1 {
			current_range.1 = current_range.1.max(end);
		} else {
			merged.push(current_range);
			current_range = (start, end);
		}
	}
	merged.push(current_range);
	merged
}


fn ip_min(ranges: &Vec<(u32, u32)>) -> u32 {
	if ranges[0].0 > 0 {
		return 0;
	}
	for i in 1..ranges.len() {
		let (i_start, _) = ranges[i];
		let (_, prev_end) = ranges[i - 1];
		if i_start > prev_end + 1 {
			return prev_end + 1;
		}
	}
	ranges.last().expect("Empty ranges").1 + 1
}


fn part1(ranges: &Vec<(u32, u32)>) -> u32 {
	ip_min(ranges)
}

fn suma(ranges: &Vec<(u32, u32)>) -> u32 {
	let mut allowed_ips = 0;
	let mut prev_end = 0;
	for &(start, end) in ranges {
		if start > prev_end {
			allowed_ips += start - prev_end - 1;
		}
		prev_end = prev_end.max(end);
	}
	allowed_ips += u32::MAX - prev_end;
	allowed_ips
}

fn part2(ranges: &Vec<(u32, u32)>) -> u32 {
	suma(ranges)
}
