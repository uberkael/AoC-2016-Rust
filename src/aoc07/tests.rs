// cSpell:ignoreRegExp ".+?"
use super::*;

#[test]
fn test1_palindrome() {
	assert!(check_palindrome("abba"));
	assert!(check_palindrome("xyyx"));
	assert!(!check_palindrome("aaaa"));
	assert!(!check_palindrome("ioxx"));
}

#[test]
fn test1_hypernet() {
	assert!(check_hypernet("aaaa[abba]qrst"));
	assert!(check_hypernet("aaaa[abcdeabbaa]qrst"));
	assert!(check_hypernet("aaaa[aaaa]qrst[abba]"));
	assert!(check_hypernet("jpgkrbovswsjnbcj[bjybzimnzszdkqj]mvwmgdithrxecpselt[kghbhnctqnhhxxhdro]drttsuyvtuuygdtfaf[zohajxyfyjnqrigq]gadarjzfafqxsjxk"));
	assert!(!check_hypernet("aaaa[abcde]qrst"));
	assert!(!check_hypernet("aaaa[aaaa]qrst"));
	assert!(!check_hypernet("wysextplwqpvipxdv[srzvtwbfzqtspxnethm]syqbzgtboxxzpwr[kljvjjkjyojzrstfgrw]obdhcczonzvbfby[svotajtpttohxsh]cooktbyumlpxostt"));
}

#[test]
fn test1_abba() {
	assert!(check_abba("abba[mnop]qrst"));
	assert!(check_abba("abcd[bddb]xyyx"));
	assert!(!check_abba("aaaa[qwer]tyui"));
	assert!(check_abba("ioxxoj[asdfgh]zxcvbn"));
}

#[test]
fn test1_tls() {
	assert!(check_tls("abba[mnop]qrst"));
	assert!(!check_tls("abcd[bddb]xyyx"));
	assert!(!check_tls("aaaa[qwer]tyui"));
	assert!(check_tls("ioxxoj[asdfgh]zxcvbn"));
}

/* Part 2 */

#[test]
fn test2_aba() {
	assert!(check_aba("aba"));
	assert!(check_aba("xyx"));
	assert!(!check_aba("aaa"));
}

#[test]
fn test2_ssl() {
	assert!(check_ssl("aba[bab]xyz"));
	assert!(!check_ssl("xyx[xyx]xyx"));
	assert!(check_ssl("aaa[kek]eke"));
	assert!(check_ssl("zazbz[bzb]cdb"));
}
