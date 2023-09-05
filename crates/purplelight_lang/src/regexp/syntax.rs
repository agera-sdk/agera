/*!
Description of the syntax for regular expressions.

The regular expression syntax is modeled after Perl.

# Matching one character

```plain
.             any character except new line (includes new line with s flag)
[0-9]         any ASCII digit
\d            digit (\p{Nd})
\D            not digit
\pX           Unicode character class identified by a one-letter name
\p{Greek}     Unicode character class (general category or script)
\PX           Negated Unicode character class identified by a one-letter name
\P{Greek}     negated Unicode character class (general category or script)
```

# Character classes

```plain
[xyz]         A character class matching either x, y or z (union).
[^xyz]        A character class matching any character except x, y and z.
[a-z]         A character class matching any character in range a-z.
[[:alpha:]]   ASCII character class ([A-Za-z])
[[:^alpha:]]  Negated ASCII character class ([^A-Za-z])
[x[^xyz]]     Nested/grouping character class (matching any character except y and z)
[a-y&&xyz]    Intersection (matching x or y)
[0-9&&[^4]]   Subtraction using intersection and negation (matching 0-9 except 4)
[0-9--4]      Direct subtraction (matching 0-9 except 4)
[a-g~~b-h]    Symmetric difference (matching `a` and `h` only)
[\[\]]        Escaping in character classes (matching [ or ])
[a&&b]        An empty character class matching nothing
```

Any named character class may appear inside a bracketed `[...]`
character class. For example, `[\p{Greek}[:digit:]]` matches
any Greek or ASCII digit. `[\p{Greek}&&\pL]` matches Greek letters.

Precedence in character classes, from most binding to least:

1. Ranges: `[a-cd]` == `[[a-c]d]`
2. Union: `[ab&&bc]` == `[[ab]&&[bc]]`
3. Intersection, difference, symmetric difference.
All three have equivalent precedence, and are evaluated in
left-to-right order. For example, `[\pL--\p{Greek}&&\p{Uppercase}]` ==
`[[\pL--\p{Greek}]&&\p{Uppercase}]`.
4. Negation: `[^a-z&&b]` == `[^[a-z&&b]]`.

# Composites

```plain
xy    concatenation (x followed by y)
x|y   alternation (x or y, prefer x)
```

This example shows how an alternation works, and what it means to
prefer a branch in the alternation over subsequent branches.

```
# use purplelight_lang::regexp::regexp;

let haystack = "samwise";
// If 'samwise' comes first in our alternation, then it is
// preferred as a match, even if the regex engine could
// technically detect that 'sam' led to a match earlier.
let re = regexp!(r"samwise|sam");
assert_eq!("samwise", re.find(haystack).unwrap().as_str());
// But if 'sam' comes first, then it will match instead.
// In this case, it is impossible for 'samwise' to match
// because 'sam' is a prefix of it.
let re = regexp!(r"sam|samwise");
assert_eq!("sam", re.find(haystack).unwrap().as_str());
```

# Repetitions

```plain
x*        zero or more of x (greedy)
x+        one or more of x (greedy)
x?        zero or one of x (greedy)
x*?       zero or more of x (ungreedy/lazy)
x+?       one or more of x (ungreedy/lazy)
x??       zero or one of x (ungreedy/lazy)
x{n,m}    at least n x and at most m x (greedy)
x{n,}     at least n x (greedy)
x{n}      exactly n x
x{n,m}?   at least n x and at most m x (ungreedy/lazy)
x{n,}?    at least n x (ungreedy/lazy)
x{n}?     exactly n x
```

# Empty matches

```plain
^     the beginning of a haystack (or start-of-line with multi-line mode)
$     the end of a haystack (or end-of-line with multi-line mode)
\A    only the beginning of a haystack (even with multi-line mode enabled)
\z    only the end of a haystack (even with multi-line mode enabled)
\b    a Unicode word boundary (\w on one side and \W, \A, or \z on other)
\B    not a Unicode word boundary
```

The empty regex is valid and matches the empty string.
For example, the empty regex matches `abc` at positions
`0`, `1`, `2` and `3`. When using the top-level `Regexp` on `&str` haystacks,
an empty match that splits a codepoint is guaranteed to never be returned.
However, such matches are permitted when using a [bytes Regexp][super::bytes::BytesRegexp].
For example:

```
# use purplelight_lang::regexp::regexp;
let re = regexp!(r"");
let ranges: Vec<_> = re.find_iter("💩").map(|m| m.range()).collect();
assert_eq!(ranges, vec![0..0, 4..4]);

use purplelight_lang::regexp::bytes::*;
let re = BytesRegexp::new(r"").unwrap();
let ranges: Vec<_> = re.find_iter("💩".as_bytes()).map(|m| m.range()).collect();
assert_eq!(ranges, vec![0..0, 1..1, 2..2, 3..3, 4..4]);
```

Note that an empty regex is distinct from a regex that can never match.
For example, the regex `[a&&b]` is a character class that represents the
intersection of `a` and `b`. That intersection is empty, which means the
character class is empty. Since nothing is in the empty set, `[a&&b]`
matches nothing, not even the empty string.

# Grouping and flags

```plain
(exp)          numbered capture group (indexed by opening parenthesis)
(?P<name>exp)  named (also numbered) capture group (names must be alpha-numeric)
(?<name>exp)   named (also numbered) capture group (names must be alpha-numeric)
(?:exp)        non-capturing group
(?flags)       set flags within current group
(?flags:exp)   set flags for exp (non-capturing)
```

Capture group names must be any sequence of alpha-numeric Unicode
codepoints, in addition to `.`, `_`, `[` and `]`. Names must start with
either an `_` or an alphabetic codepoint. Alphabetic codepoints correspond
to the `Alphabetic` Unicode property, while numeric codepoints correspond to
the union of the `Decimal_Number`, `Letter_Number` and `Other_Number` general categories.

Flags are each a single character. For example, `(?x)` sets the flag `x`
and `(?-x)` clears the flag `x`. Multiple flags can be set or cleared at
the same time: `(?xy)` sets both the `x` and `y` flags and `(?x-y)` sets
the `x` flag and clears the `y` flag.

All flags are by default disabled unless stated otherwise. They are:

```plain
i     case-insensitive: letters match both upper and lower case
m     multi-line mode: ^ and $ match begin/end of line
s     allow . to match \n
R     enables CRLF mode: when multi-line mode is enabled, \r\n is used
U     swap the meaning of x* and x*?
u     Unicode support (enabled by default)
x     verbose mode, ignores whitespace and allow line comments (starting with `#`)
```

Note that in verbose mode, whitespace is ignored everywhere, including
within character classes. To insert whitespace, use its escaped form or a
hex literal. For example, `\`  or `\x20` for an ASCII space.

Flags can be toggled within a pattern. Here’s an example that matches
case-insensitively for the first part but case-sensitively for the second part:

```
# use purplelight_lang::regexp::regexp;
let re = regexp!(r"(?i)a+(?-i)b+");
let m = re.find("AaAaAbbBBBb").unwrap();
assert_eq!(m.as_str(), "AaAaAbb");
```

Notice that the `a+` matches either `a` or `A`, but the `b+` only matches `b`.

Multi-line mode means `^` and `$` no longer match just at the
beginning/end of the input, but also at the beginning/end of lines:

```
# use purplelight_lang::regexp::regexp;
let re = regexp!(r"(?m)^line \d+");
let m = re.find("line one\nline 2\n").unwrap();
assert_eq!(m.as_str(), "line 2");
```

Note that `^` matches after new lines, even at the end of input:

```
# use purplelight_lang::regexp::regexp;
let re = regexp!(r"(?m)^");
let m = re.find_iter("test\n").last().unwrap();
assert_eq!((m.start(), m.end()), (5, 5));
```

When both CRLF mode and multi-line mode are enabled, then `^` and `$`
will match either `\r` and `\n`, but never in the middle of a `\r\n`:

```
# use purplelight_lang::regexp::regexp;
let re = regexp!(r"(?mR)^foo$");
let m = re.find("\r\nfoo\r\n").unwrap();
assert_eq!(m.as_str(), "foo");
```

Unicode mode can also be selectively disabled,
although only when the result _would not_ match invalid UTF-8.
One good example of this is using an ASCII word boundary instead of a
Unicode word boundary, which might make some regex searches run faster:

```
# use purplelight_lang::regexp::regexp;
let re = regexp!(r"(?-u:\b).+(?-u:\b)");
let m = re.find("$$abc$$").unwrap();
assert_eq!(m.as_str(), "abc");
```

# Escape sequences

Note that this includes all possible escape sequences, even ones that are documented elsewhere.

```plain
\*          literal *, applies to all ASCII except [0-9A-Za-z<>]
\a          bell (\x07)
\f          form feed (\x0C)
\t          horizontal tab
\n          new line
\r          carriage return
\v          vertical tab (\x0B)
\A          matches at the beginning of a haystack
\z          matches at the end of a haystack
\b          word boundary assertion
\B          negated word boundary assertion
\123        octal character code, up to three digits (when enabled)
\x7F        hex character code (exactly two digits)
\x{10FFFF}  any hex character code corresponding to a Unicode code point
\u007F      hex character code (exactly four digits)
\u{7F}      any hex character code corresponding to a Unicode code point
\U0000007F  hex character code (exactly eight digits)
\U{7F}      any hex character code corresponding to a Unicode code point
\p{Letter}  Unicode character class
\P{Letter}  negated Unicode character class
\d, \s, \w  Perl character class
\D, \S, \W  negated Perl character class
```

# Perl character classes (Unicode friendly)

These classes are based on the definitions provided in
[UTS#18](https://www.unicode.org/reports/tr18/#Compatibility_Properties):

```plain
\d     digit (\p{Nd})
\D     not digit
\s     whitespace (\p{White_Space})
\S     not whitespace
\w     word character (\p{Alphabetic} + \p{M} + \d + \p{Pc} + \p{Join_Control})
\W     not word character
```

# ASCII character classes

These classes are based on the definitions provided in
[UTS#18](https://www.unicode.org/reports/tr18/#Compatibility_Properties):

```plain
[[:alnum:]]    alphanumeric ([0-9A-Za-z])
[[:alpha:]]    alphabetic ([A-Za-z])
[[:ascii:]]    ASCII ([\x00-\x7F])
[[:blank:]]    blank ([\t ])
[[:cntrl:]]    control ([\x00-\x1F\x7F])
[[:digit:]]    digits ([0-9])
[[:graph:]]    graphical ([!-~])
[[:lower:]]    lower case ([a-z])
[[:print:]]    printable ([ -~])
[[:punct:]]    punctuation ([!-/:-@\[-`{-~])
[[:space:]]    whitespace ([\t\n\v\f\r ])
[[:upper:]]    upper case ([A-Z])
[[:word:]]     word characters ([0-9A-Za-z_])
[[:xdigit:]]   hex digit ([0-9A-Fa-f])
```
*/