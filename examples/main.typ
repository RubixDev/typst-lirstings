#import "../src/lib.typ": lirsting

#show raw: lirsting

Test ```rust fn inline() {}```, no lang `inline`.

```rust
fn main() {}
```

lol

```js
function main() {}
```

```rust
use regex::Regex;
use lazy_regex::{regex, regex_is_match};

fn fib(n: usize) -> usize {
    if n < 2 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    Regex::new(r"[a-fA-F0-9_]\s(.*)$");
    let a = regex!(r"[a-fA-F0-9_]\s(.*)$");
    if regex_is_match!(/* comment */ r"[a-fA-F0-9_]\s(.*)$"i, r"raw text \s[a-f]") {
        return;
    }
}
```

#[
  #show raw: lirsting.with(theme: "gruvbox::light")

  ```go
  import "fmt"

  // comment
  func Main() {
      fmt.Println("Hello, World!")
  }
  ```
]

Inline code is also supported: #raw("fn main() {}", lang: "rust").
This may be useful to reference types like #raw("i32", lang: "rust") or functions like #raw("foo()", lang: "rust") or even things like regular expressions '#raw("[a-fA-F0-9_]\s(.*)$", lang: "regex")' in text.

Languages that `syntastica` doesn't support will continue to be highlighted by Typst's native highlighting logic (using `syntect`)

```typ
= Chapter 1
#let hi = "Hello World"
```

```py
def fib(n):
    if n < 0:
        return None
    if n == 0 or n == 1:
        return n
    return fib(n-1) + fib(n-2)
```

#[
  #show raw.where(block: false): it => box(
    it,
    fill: luma(240),
    inset: (x: 3pt, y: 0pt),
    outset: (y: 3pt),
    radius: 2pt,
  )
  #show raw.where(block: true): it => block(
    it,
    fill: luma(240),
    inset: 10pt,
    radius: 4pt,
  )

  You can also combine `lirstings` with other show rules.
  Here is the RegEx #raw("[a-fA-F0-9_]\s(.*)$", lang: "regex", block: false) again.

  ```s
  .intel_syntax
  .global _start

  .section .text

  _start:
      call        main..main
      mov         %rdi, 0
      call        exit

  main..main:
      push        %rbp
      mov         %rbp, %rsp
      sub         %rsp, 32
      mov         qword ptr [%rbp-8], 3
      lea         %rax, qword ptr [%rbp-8]
      mov         qword ptr [%rbp-16], %rax
      lea         %rax, qword ptr [%rbp-16]
      mov         qword ptr [%rbp-24], %rax
      mov         %rax, qword ptr [%rbp-24]
      mov         %rax, qword ptr [%rax]
      mov         qword ptr [%rbp-32], %rax
      mov         %rdi, qword ptr [%rbp-24]
      mov         %rdi, qword ptr [%rdi]
      mov         %rdi, qword ptr [%rdi]
      mov         %rsi, qword ptr [%rbp-24]
      mov         %rsi, qword ptr [%rsi]
      mov         %rsi, qword ptr [%rsi]
      call        __rush_internal_pow_int
      mov         %rdi, %rax
      mov         %rax, qword ptr [%rbp-32]
      mov         qword ptr [%rax], %rdi
      mov         %rdi, qword ptr [%rbp-24]
      mov         %rdi, qword ptr [%rdi]
      mov         %rdi, qword ptr [%rdi]
      call        exit
  main..main.return:
      leave
      ret
  ```
]

#[
  #set text(font: "New Computer Modern")
  #show raw: set text(font: "JetBrains Mono NL")
  #show figure.caption: it => {
    [*#it.supplement~2.7* -- #it.body]
  }

  #let fancylist(
    body,
    title: none,
    title-gap: .5em,
    frame-color: luma(160),
    frame-thickness: .5pt,
    line-ranges: none,
  ) = {
    show raw.where(block: true): it => block(inset: (x: 0pt, y: 5pt), {
      set line(length: 100%, stroke: frame-thickness + frame-color)
      if title != none {
        context {
          let title-text = text(frame-color, .8em, title)
          let width = measure(title-text).width
          set line(length: 50% - width / 2 - title-gap)

          box(line())
          h(title-gap)
          title-text
          h(title-gap)
          box(line())
        }
      } else {
        line()
      }
      it
      line()
    })
    show raw.line: it => {
      let line-ranges = if line-ranges == none { none } else {
        line-ranges.map(pair => {
            let start = pair.at(0)
            let end = pair.at(1, default: none)
            range(start, if end != none { end + 1 } else { start + it.count }) + (none,)
          })
          .flatten()
      }
      box(place(
        right,
        text(.8em, frame-color, {
          if line-ranges == none {
            [#it.number]
          } else {
            [#line-ranges.at(it.number - 1)]
          }
          h(1.5em)
        }),
        dy: -.6em,
      ))
      it
    }
    body
  }

  #figure(
  caption: [Pratt-parser: Implementation for grouped expressions.],
    align(start, fancylist(title: "crates/rush-parser/src/parser.rs", line-ranges: ((733, 743), (749,)))[
      ```rust
      fn grouped_expr(&mut self) -> Result<'src, Spanned<'src, Box<Expression<'src>>>> {
          let start_loc = self.curr_tok.span.start;
          // skip the opening parenthesis
          self.next()?;

          let expr = self.expression(0)?;
          self.expect_recoverable(
              TokenKind::RParen,
              "missing closing parenthesis",
              self.curr_tok.span,
          )?;
          // ...
      }
      ```
    ])
  ) <test>

  See @test.
]
