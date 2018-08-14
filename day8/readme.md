# Day 8

This problem involved some simple parsing. However, I wanted to
challenge myself and use a combinator parser library to try and
learn some new tools. The following solution is quite verbose as
it does a lot more than it really has to if a simple regex based
parsing scheme had been implemented.

The solution makes use of [combine][combine], a trait-based
combinator parser that doesn't make any use of macros in order
to write the parsing code (unlike nom).

  [combine]: https://github.com/Marwes/combine

