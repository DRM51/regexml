use regexml::{Node, Regex, RepeatKind};

fn lit(s: &str) -> Node {
    Node::Literal(s.chars().collect())
}

fn class(ranges: &[(char, char)]) -> Node {
    Node::Class(ranges.iter().map(|(a, b)| (*a as u32, *b as u32)).collect())
}

fn repeat(node: Node, min: usize, max: Option<usize>, kind: RepeatKind) -> Node {
    Node::Repeat {
        node: Box::new(node),
        min,
        max,
        kind,
    }
}

fn capture(group: usize, node: Node) -> Node {
    Node::Capture {
        group,
        node: Box::new(node),
    }
}

#[test]
fn test_node_literal_and_class() {
    let regex = Regex::xpath("ab[c-e]", "").unwrap();
    assert_eq!(
        regex.node(),
        Node::Sequence(vec![lit("ab"), class(&[('c', 'e')]), Node::End])
    );
}

#[test]
fn test_node_dot() {
    let regex = Regex::xpath(".", "").unwrap();
    assert_eq!(
        regex.node(),
        Node::Sequence(vec![
            Node::Class(vec![(0, 9), (11, 12), (14, 0x10FFFF)]),
            Node::End
        ])
    );
    let regex = Regex::xpath(".", "s").unwrap();
    assert_eq!(
        regex.node(),
        Node::Sequence(vec![Node::Class(vec![(0, 0x10FFFF)]), Node::End])
    );
}

#[test]
fn test_node_case_closure_in_class() {
    // the class holds the case variants: K, k and KELVIN SIGN
    let regex = Regex::xpath("[k]", "i").unwrap();
    assert_eq!(
        regex.node(),
        Node::Sequence(vec![
            class(&[('K', 'K'), ('k', 'k'), ('\u{212A}', '\u{212A}')]),
            Node::End
        ])
    );
    // a literal keeps its character; the flag says how it matches
    let regex = Regex::xpath("k", "i").unwrap();
    assert_eq!(regex.node(), Node::Sequence(vec![lit("k"), Node::End]));
    assert!(regex.flags().case_insensitive);
}

#[test]
fn test_node_repeat_kinds() {
    // followed by a character it cannot match: no backtracking needed
    let regex = Regex::xpath("a*b", "").unwrap();
    assert_eq!(
        regex.node(),
        Node::Sequence(vec![
            repeat(lit("a"), 0, None, RepeatKind::Unambiguous),
            lit("b"),
            Node::End
        ])
    );
    let regex = Regex::xpath("a*a", "").unwrap();
    assert_eq!(
        regex.node(),
        Node::Sequence(vec![
            repeat(lit("a"), 0, None, RepeatKind::GreedyFixed),
            lit("a"),
            Node::End
        ])
    );
    let regex = Regex::xpath("a{2,3}?", "").unwrap();
    assert_eq!(
        regex.node(),
        Node::Sequence(vec![
            repeat(lit("a"), 2, Some(3), RepeatKind::ReluctantFixed),
            Node::End
        ])
    );
    let regex = Regex::xpath("(?:ab|c)+", "").unwrap();
    assert_eq!(
        regex.node(),
        Node::Sequence(vec![
            repeat(
                Node::Choice(vec![lit("ab"), lit("c")]),
                1,
                None,
                RepeatKind::Greedy
            ),
            Node::End
        ])
    );
    let regex = Regex::xpath("(?:ab|c)+?", "").unwrap();
    assert_eq!(
        regex.node(),
        Node::Sequence(vec![
            repeat(
                Node::Choice(vec![lit("ab"), lit("c")]),
                1,
                None,
                RepeatKind::Reluctant
            ),
            Node::End
        ])
    );
}

#[test]
fn test_node_groups() {
    let regex = Regex::xpath("(a)(?:b)(c)", "").unwrap();
    assert_eq!(regex.capture_count(), 2);
    assert_eq!(
        regex.node(),
        Node::Sequence(vec![
            capture(1, lit("a")),
            lit("b"),
            capture(2, lit("c")),
            Node::End
        ])
    );
    // a group repeated zero times is dropped, but still counted
    let regex = Regex::xpath("(a){0}(b)", "").unwrap();
    assert_eq!(regex.capture_count(), 2);
    assert_eq!(
        regex.node(),
        Node::Sequence(vec![Node::Empty, capture(2, lit("b")), Node::End])
    );
}

#[test]
fn test_node_anchors_and_back_reference() {
    let regex = Regex::xpath("^(a)\\1$", "").unwrap();
    assert_eq!(
        regex.node(),
        Node::Sequence(vec![
            Node::LineStart,
            capture(1, lit("a")),
            Node::BackReference(1),
            Node::LineEnd,
            Node::End
        ])
    );
}

#[test]
fn test_node_flags_and_empty_string() {
    let regex = Regex::xpath("a b", "x").unwrap();
    let flags = regex.flags();
    assert!(flags.ignore_whitespace);
    assert!(!flags.case_insensitive && !flags.multi_line && !flags.dot_all && !flags.literal);
    assert_eq!(regex.node(), Node::Sequence(vec![lit("ab"), Node::End]));
    assert!(!regex.matches_empty_string());
    assert!(Regex::xpath("a*", "").unwrap().matches_empty_string());
    let regex = Regex::xpath("(a)", "q").unwrap();
    assert!(regex.flags().literal);
    assert_eq!(regex.capture_count(), 0);
    assert_eq!(regex.node(), Node::Sequence(vec![lit("(a)"), Node::End]));
}
