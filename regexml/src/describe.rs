//! A read-only description of a compiled regular expression.
//!
//! regexml reads and checks a pattern (its syntax and its flags) and
//! compiles it into operations. [`Regex::node`] describes
//! those operations as they are after regexml's own rewrites and
//! optimizations, so that a program can run the same pattern with another
//! engine, or find that it cannot. Every node says how regexml matches it.
//! Where another engine cannot match a node in exactly the same way, the
//! program should leave the pattern to regexml.
//!
//! [`Regex::node`]: crate::Regex::node

/// How a [`Node::Repeat`] is matched.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RepeatKind {
    /// Greedy repetition of a node whose matches vary in length: the most
    /// repetitions first, backtracking into each repetition.
    ///
    /// When `min` is 0, regexml offers the match with no repetition at a
    /// given position only the first time the repetition starts there while
    /// one string is searched.
    Greedy,
    /// Reluctant repetition of a node whose matches vary in length.
    ///
    /// regexml takes the first match of the node in every repetition and
    /// does not offer the match with no repetition first, so this is not
    /// the usual reluctant repetition.
    Reluctant,
    /// Greedy repetition of a node whose matches all have the same length:
    /// every repetition takes the first match of the node, and fewer
    /// repetitions are tried on backtracking.
    GreedyFixed,
    /// Reluctant repetition of a node whose matches all have the same
    /// length: every repetition takes the first match of the node, and more
    /// repetitions are tried on backtracking.
    ReluctantFixed,
    /// Only the largest number of repetitions is tried, with no
    /// backtracking (a possessive repetition). regexml uses it for a
    /// literal or a class when `min` equals `max`, or when it found that
    /// the node after it in the same sequence cannot start with a character
    /// that the repeated node matches.
    Unambiguous,
}

/// A compiled operation of a regular expression.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Node {
    /// A string of characters. With the `i` flag, an input character
    /// matches a character of the string when both are equal or when their
    /// simple lowercase forms are equal.
    Literal(Vec<char>),
    /// One character from a set, given as sorted, non-overlapping inclusive
    /// code point ranges. With the `i` flag the set includes the case
    /// variants where the regular expression asks for them; the set is
    /// matched as it is.
    Class(Vec<(u32, u32)>),
    /// The nodes one after the other.
    Sequence(Vec<Node>),
    /// The first alternative that leads to a match, in order.
    Choice(Vec<Node>),
    /// A capturing group: the substring that `node` matched is group
    /// `group` (numbered from 1 in the order of the opening parentheses).
    Capture {
        /// The group number.
        group: usize,
        /// What the group matches.
        node: Box<Node>,
    },
    /// A repetition of `node`, from `min` to `max` times (`None`: no upper
    /// bound).
    Repeat {
        /// The repeated node.
        node: Box<Node>,
        /// The least number of repetitions.
        min: usize,
        /// The largest number of repetitions, if there is one.
        max: Option<usize>,
        /// How the repetition is matched.
        kind: RepeatKind,
    },
    /// A back-reference to the substring of a group (`\1`).
    BackReference(usize),
    /// `^`: the start of the string; with the `m` flag also the position
    /// after a newline (`\n`) that is not the last character of the string.
    LineStart,
    /// `$`: the end of the string; with the `m` flag also the position
    /// before a newline (`\n`).
    LineEnd,
    /// The empty string, matched anywhere.
    Empty,
    /// The end of the regular expression (the last node of the outermost
    /// sequence); it does not require the end of the string.
    End,
}

/// The flags a regular expression was compiled with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Flags {
    /// `i`: case-insensitive mode.
    pub case_insensitive: bool,
    /// `m`: multi-line mode (`^` and `$` match at the start and end of every line).
    pub multi_line: bool,
    /// `s`: dot-all mode (`.` matches every character).
    pub dot_all: bool,
    /// `x`: whitespace in the pattern is removed before it is read, except
    /// within character class expressions.
    pub ignore_whitespace: bool,
    /// `q`: the pattern is a literal string.
    pub literal: bool,
}
