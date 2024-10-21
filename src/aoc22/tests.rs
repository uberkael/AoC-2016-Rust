// cSpell:ignoreRegExp ".*"
use super::*;


#[test]
fn test_node_parse() {
	let node = Node::parse("/dev/grid/node-x0-y0     92T   68T    24T   73%")
		.expect("Error parsing");
	assert_eq!(node.x, 0);
	assert_eq!(node.y, 0);
	assert_eq!(node.size, 92);
	assert_eq!(node.used, 68);
	assert_eq!(node.avail, 24);
	let node = Node::parse("/dev/grid/node-x0-y1     90T   68T    22T   75%")
		.expect("Error parsing");
	let values = (node.x, node.y, node.size, node.used, node.avail);
	assert_eq!(values, (0, 1, 90, 68, 22));
	let node = Node::parse("/dev/grid/node-x0-y2     92T   73T    19T   79%")
		.expect("Error parsing");
	let values = (node.x, node.y, node.size, node.used, node.avail);
	assert_eq!(values, (0, 2, 92, 73, 19));
	let node = Node::parse("/dev/grid/node-x0-y3     93T   66T    27T   70%")
		.expect("Error parsing");
	let values = (node.x, node.y, node.size, node.used, node.avail);
	assert_eq!(values, (0, 3, 93, 66, 27));
	let node = Node::parse("/dev/grid/node-x0-y4     91T   69T    22T   75%")
		.expect("Error parsing");
	let values = (node.x, node.y, node.size, node.used, node.avail);
	assert_eq!(values, (0, 4, 91, 69, 22));
}

#[test]
fn test_viable() {
	let mut n = Nodes::new();
	n.add(Node::new(0, 0, 92, 68, 22));
	n.add(Node::new(0, 1, 90, 20, 80));
	assert_eq!(n.viable_count(), 2);
	let mut n = Nodes::new();
	n.add(Node::new(0, 0, 92, 68, 24));
	n.add(Node::new(0, 1, 90, 90, 20));
	assert_eq!(n.viable_count(), 0);
}
