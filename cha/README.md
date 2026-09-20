# Cha

The language - Starting with a bootstrapped C++ compiler.

This is intedned to be a living document. I am really bad at keeping documentation and code in sync
but I will work to keep things up to date here.

## Development Process

I am completely new to this, I have never really tried anything non-trivial related to compilers or
interpreters, so I expect to make a lot of mistakes and hopefully learn a lot. I am also using this
as a safe space away from AI. Output is not the goal, the process is. I want to make sure that I
dedicate time to keeping my programming skills up so I don't lose them.

I plan on developing thin vertical slices, focusing on incremental functionality while still keeping
quality up and minimizing both throw-away work and technical debt. High quality code and design is
the highest priority, including over feature delivery.

There will be three motivators for adding new features:
 1. Personal Interest
 2. Self Hosting
 3. Operating System in the language

If something doesn't check these boxes, I probably won't do it.

## Language Philosophy

The previous section probably gives a decent idea of the style of language I want to write. Some
characteristics I want to aim for:
 - **Memory safety** - I am tired of running into bugs in code that was partially migrated from C to
   C++, I want the compiler to catch memory errors and use after move bugs.
 - **No garbage collection** - Need to have predictable performance, not acceptable for an OS
 - **Async Support** - It seems backwards to me that hardware is inherently asynchronous and we put a
   synchronous abstraction on top of it, then need to wedge in an async layer on top of it in
   existing languages.
 - **Compile Time Programming** - Needs to be fully built into the language. This is incredibly
   helpful for communicating intent both to other programmers and the compiler.

This is not a comprehensive list, it is really just the things I had at top of mind. It is more of a
list of complaints about languages I have worked with than anything else.

I plan on taking heavy inspiration from Rust, Zig, and C++. However, I have very little experience
with either Rust or Zig, this project should also help me learn more about those languages.

## Design

### Grammar

How the language looks isn't super important to me, but I need to start with something. I'll probably
make most of these decisions based on the requirements imposed by the technical internals of the
compiler. This means I'll probably end up with something mostly un-original.

I will specify this in a `.peg` file, similar to what is in the
[Zig documentation](https://ziglang.org/documentation/master/#Grammar).

Here is my starting point:
```peg
# --- Structure ------------------------

Root <- skip Decl* EOF

Decl <- FnDecl

FnDecl <- KEYWORD_fn IDENTIFIER LPAREN ParamList RPAREN ARROW TypeExpr Block

ParamList <-

TypeExpr <- IDENTIFIER

# --- Statements & Expressions ---------

Block <- LBRACE Expr? RBRACE

Expr <- PrimaryExpr

PrimaryExpr <- INTEGER
             / Block

# --- Tokens ---------------------------

INTEGER <- dec_int skip
dec_int <- [0-9] ('_'? [0-9])*

IDENTIFIER <- !keyword [A-Za-z_] [A-Za-z0-9_]* skip

KEYWORD_fn <- 'fn' end_of_word
keyword <- KEYWORD_fn

LPAREN <- '('
RPAREN <- ')'
LBRACE <- '{'
RBRACE <- '}'
ARROW <- '->' skip

# --- Whitespace & commnets ------------

end_of_word <- ![A-Za-z0-9_] skip
line_comment <- '//' [^\n]*
skip <- ([ \t\r\n] / line_comment)*
EOF <- !.
```

This is just enough to parse the most trivial possible program:
```cha
fn main() -> i64 { 0 }
```