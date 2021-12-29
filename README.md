# strsplit

implementation of strsplit from [rust lifetime annotations video](https://www.youtube.com/watch?v=4DqP57BHaXI). i paused video pretty early and made a more robust implementation

### running

```sh
$ cargo test
```

### usage

```rs
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

// delimiter not found
let haystack = "foo bar";
let letters = StrSplit::new(haystack, "baz").collect::<Vec<_>>();
assert_eq!(letters, vec!["foo bar"]);

// empty delimiter returns each character as str
let haystack = "abcd";
let letters = StrSplit::new(haystack, "").collect::<Vec<_>>();
assert_eq!(letters, vec!["a", "b", "c", "d"]);
```
