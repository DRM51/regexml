use regexml::{AnalyzeEntry, MatchEntry, Regex};

// [m] is a match of m, (n) a non-match of n, as in the probes of XSLT's
// xsl:analyze-string; the expected strings are Saxon-HE's.
fn render(regex: &Regex, input: &str) -> String {
    let mut out = String::new();
    for entry in regex.analyze_xslt(input) {
        match entry {
            AnalyzeEntry::Match(entries) => {
                out.push('[');
                push_entries(&mut out, &entries);
                out.push(']');
            }
            AnalyzeEntry::NonMatch(s) => {
                out.push('(');
                out.push_str(&s);
                out.push(')');
            }
        }
    }
    out
}

fn push_entries(out: &mut String, entries: &[MatchEntry]) {
    for entry in entries {
        match entry {
            MatchEntry::String(s) => out.push_str(s),
            MatchEntry::Group { value, .. } => push_entries(out, value),
        }
    }
}

#[test]
fn test_analyze_xslt_zero_length_matches() {
    let regex = Regex::xpath("x*", "").unwrap();
    assert!(regex.analyze("axb").is_err());
    assert_eq!(render(&regex, "axb"), "[](a)[x][](b)[]");
}

#[test]
fn test_analyze_xslt_empty_fields() {
    let regex = Regex::xpath("([^,]*)", "").unwrap();
    assert_eq!(render(&regex, "a,,b"), "[a][](,)[](,)[b][]");
}

#[test]
fn test_analyze_xslt_same_as_analyze() {
    let regex = Regex::xpath("[0-9]+", "").unwrap();
    let a: Vec<_> = regex.analyze("a12b345").unwrap().collect();
    let b: Vec<_> = regex.analyze_xslt("a12b345").collect();
    assert_eq!(a, b);
    assert_eq!(render(&regex, "a12b345"), "(a)[12](b)[345]");
}

#[test]
fn test_analyze_xslt_no_match() {
    let regex = Regex::xpath("x", "").unwrap();
    assert_eq!(render(&regex, ""), "");
    assert_eq!(render(&regex, "ab"), "(ab)");
}
