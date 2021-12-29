#![warn(missing_debug_implementations, rust_2018_idioms)]

#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: &'a str,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: haystack,
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remainder.is_empty() {
            None
        } else if self.delimiter.is_empty() {
            let first = &self.remainder[..1];
            self.remainder = &self.remainder[1..];
            Some(first)
        } else if let Some(next_delim) = self.remainder.find(self.delimiter) {
            let before_delim = &self.remainder[..next_delim];
            self.remainder = &self.remainder[next_delim + self.delimiter.len()..];
            Some(before_delim)
        } else {
            let last = self.remainder;
            self.remainder = "";
            Some(last)
        }
    }
}

#[test]
fn it_works() {
    // base case
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ").collect::<Vec<_>>();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);

    // long delimiter
    let haystack = "barfoobaz";
    let letters = StrSplit::new(haystack, "foo").collect::<Vec<_>>();
    assert_eq!(letters, vec!["bar", "baz"]);

    // empty haystack
    let haystack = "";
    let letters = StrSplit::new(haystack, " ").collect::<Vec<_>>();
    assert_eq!(letters, vec![] as Vec<&str>);

    // empty delimiter returns each character as str
    let haystack = "abcd";
    let letters = StrSplit::new(haystack, "").collect::<Vec<_>>();
    assert_eq!(letters, vec!["a", "b", "c", "d"] as Vec<&str>);
}