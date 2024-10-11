use super::*;

#[test]
fn test_tile() {
	let parents = [true, true, false];
	let tile = Tile::new(parents);
	assert_eq!(tile.trap, true);
}

#[test]
fn test_generate_row() {
	let row = Row::new(".^^^^".to_string());
	let row = row.generate_row();
	assert_eq!(row.to_string(), "^^..^");


	let row = Row::new("..^^.".to_string());
	let row = row.generate_row();
	assert_eq!(row.to_string(), ".^^^^");
	let row = row.generate_row();
	assert_eq!(row.to_string(), "^^..^");

	let row = Row::new(".^^.^.^^^^".to_string());
	let row = row.generate_row();
	assert_eq!(row.to_string(), "^^^...^..^");
	let row = row.generate_row();
	assert_eq!(row.to_string(), "^.^^.^.^^.");
	let row = row.generate_row();
	assert_eq!(row.to_string(), "..^^...^^^");
	let row = row.generate_row();
	assert_eq!(row.to_string(), ".^^^^.^^.^");
	let row = row.generate_row();
	assert_eq!(row.to_string(), "^^..^.^^..");
	let row = row.generate_row();
	assert_eq!(row.to_string(), "^^^^..^^^.");
	let row = row.generate_row();
	assert_eq!(row.to_string(), "^..^^^^.^^");
	let row = row.generate_row();
	assert_eq!(row.to_string(), ".^^^..^.^^");
	let row = row.generate_row();
	assert_eq!(row.to_string(), "^^.^^^..^^");
}

#[test]
fn test_generate_map() {
	let row = Row::new(".^^.^.^^^^".to_string());
	let map = Map::new_from_row(10, row);
	assert_eq!(map.to_string(),
	".^^.^.^^^^\n\
	^^^...^..^\n\
	^.^^.^.^^.\n\
	..^^...^^^\n\
	.^^^^.^^.^\n\
	^^..^.^^..\n\
	^^^^..^^^.\n\
	^..^^^^.^^\n\
	.^^^..^.^^\n\
	^^.^^^..^^");
	assert_eq!(map.count_safe(), 38);
}
