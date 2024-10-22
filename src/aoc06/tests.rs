// cSpell:ignoreRegExp \w+\\n\w+|"\w+"
use super::*;

#[test]
fn test1_input() {
	let input = "eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv
nssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar";

	let data = input.lines().collect::<Vec<_>>();
	let dictionaries = dictionaries(&data);

	assert_eq!(6, data[0].len());
	assert_eq!(data[0].len(), dictionaries.len());
	assert_eq!('e', max_char(&dictionaries[0]));
	assert_eq!("easter", dict_max_char(dictionaries));
}

/* Part 2 */
#[test]
fn test2_input() {
	let input = "eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv
nssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar";

	let data = input.lines().collect::<Vec<_>>();
	let dictionaries = dictionaries(&data);

	assert_eq!('a', min_char(&dictionaries[0]));
	assert_eq!("advent", dict_min_char(dictionaries));

}
