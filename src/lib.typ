#let highlighted-code = ()
#{
  let file = sys.inputs.at("__lirstings", default: none)
  if file != none {
    highlighted-code = json(file)
  }
}

#let _show-highlights(highlights) = {
  for (index, line) in highlights.enumerate() {
    let line-contents = line.map(((part, style)) => part).join()
    let highlighted-line = {
      for (part, style) in line {
        if style == none {
          text(part)
          continue
        }
        let tmp = text(
          fill: rgb(style.color.red, style.color.green, style.color.blue),
          style: if style.italic { "italic" } else { "normal" },
          weight: if style.bold { "bold" } else { "regular" },
          part
        )
        if style.bg != none {
          tmp = highlight(tmp, fill: rgb(style.bg.red, style.bg.green, style.bg.blue))
        }
        if style.strikethrough {
          tmp = strike(tmp)
        }
        if style.underline {
          tmp = underline(tmp)
        }
        tmp
      }
      // don't add a line break after the last line, needed for inline code
      if index != highlights.len() - 1 {
        linebreak()
      }
    }
    raw.line(index + 1, highlights.len(), line-contents, highlighted-line)
  }
}

#let __lirstings = counter("__lirstings")
#let lirsting(it, theme: "one::light") = {
  if it.has("label") and it.label == label("__lirstings-ignore") { return it }

  [#metadata((lang: it.lang, text: it.text, theme: theme))<__lirstings>]
  context {
    let highlights = highlighted-code.at(__lirstings.get().at(0), default: none)
    if it.block {
      block({
        set align(it.align)
        if highlights != none {
          _show-highlights(highlights)
        } else {
          it.lines.join(linebreak())
        }
      })
    } else if highlights != none {
      _show-highlights(highlights)
    } else {
      it.lines.join()
    }
  }
  __lirstings.step()
}
