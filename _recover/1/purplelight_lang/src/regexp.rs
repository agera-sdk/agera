/*!
Regular expressions.

Regular expressions are patterns used to match character
combinations in strings. The syntax is modeled after Perl.

# Syntax

The syntax is modeled after Perl. [Consult here for more information on the syntax.][syntax]
This is mostly copied from the documentation from the `regex` crate from
the crates.io registry.

# Creating a regular expression

There are two ways of constructing a regular expression object:

- Using a `regexp!` literal, which consists of a pattern and optional flags, as follows:
```
# use agera_lang::regexp::*;
let my_regexp = regexp!(r"pattern");
let my_regexp = regexp!(r"pattern");
```
  `regexp!` literals compile the regular expression only once.
- Or calling the `Regexp::new` constructor:
```
# use agera_lang::regexp::*;
let my_regexp = Regexp::new(r"pattern").unwrap();
```

Flags, such as `i`, can be passed as suffix when using the `regexp!` literal:

```
# use agera_lang::regexp::*;
let _ = regexp!(r"pattern"i);
```

# Creating a static regular expression

Sometimes you may wish to not repeat a certain regular expression literal.
In that case you can use the `lazy_regexp!` literal and annotate it with
`LazyRegexp` to define a global regular expression:

```
# use agera_lang::regexp::*;
static GLOBAL_REGEX: LazyRegexp = lazy_regexp!(r"pattern");
```

# Replacement

Most commonly, macros such as `regexp_replace_all!` can be used to replace occurrences:

```
# use agera_lang::regexp::*;
let text = "Foo fuu";
let text = regexp_replace_all!(
    r#"\bf(?P<suffix>\w+)"#i,
    text,
    |_, suffix: &str| format!("F<{}>", suffix),
);
assert_eq!(text, "F<oo> F<uu>");
```

Currently, the capture groups in the callback given to macros such as these
must be typed as above, often with just `&str`, otherwise the macro
may report wrong diagnostics.
*/

pub mod syntax;

pub use lazy_regex::{
    regex as regexp,
    lazy_regex as lazy_regexp,
    regex::{
        Regex as Regexp,
        Match as RegexpMatch,
        Error as RegexpError,
        Captures as RegexpCaptures,
        CaptureMatches as RegexpCaptureMatches,
        CaptureNames as RegexpCaptureNames,
        CaptureLocations as RegexpCaptureLocations,
        SubCaptureMatches as RegexpSubCaptureMatches,
    },
    regex::Replacer as RegexpReplacer,

    regex_captures as regexp_captures,
    regex_find as regexp_find,
    regex_is_match as regexp_is_match,
    regex_replace as regexp_replace,
    regex_replace_all as regexp_replace_all,
};

pub type LazyRegexp = lazy_regex::Lazy<Regexp>;

/// Work with regular expressions on slices of bytes.
pub mod bytes {
    pub use lazy_regex::regex::bytes::{
        Regex as BytesRegexp,
        Match as BytesRegexpMatch,
        Captures as BytesRegexpCaptures,
        CaptureMatches as BytesRegexpCaptureMatches,
        CaptureNames as BytesRegexpCaptureNames,
        CaptureLocations as BytesRegexpCaptureLocations,
        SubCaptureMatches as BytesRegexpSubCaptureMatches,
    };
}