use regexml::Regex;

// A reluctant repetition of an operation whose matches vary in length used to
// loop forever when the operation did not match before the least number of
// repetitions was reached, and to repeat its last position when it stopped
// matching later.

#[test]
fn test_reluctant_repeat_stops_when_the_operation_fails() {
    let regex = Regex::xpath("(?:ab|c)+?", "").unwrap();
    assert!(!regex.is_match("x"));
    assert!(regex.is_match("xab"));
    let regex = Regex::xpath("(?:a|bc){2,}?", "").unwrap();
    assert!(!regex.is_match("a"));
    assert!(regex.is_match("abc"));
}

#[test]
fn test_reluctant_repeat_matches() {
    let regex = Regex::xpath("x(?:ab|c)+?", "").unwrap();
    assert_eq!(regex.replace_all("xabab xc", "-").unwrap(), "-ab -");
    let regex = Regex::xpath("(?:ab|c){2,3}?d", "").unwrap();
    assert_eq!(regex.replace_all("abcd cd abababd", "-").unwrap(), "- cd -");
}
