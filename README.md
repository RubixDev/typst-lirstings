# `typst-lirstings`

Tree-sitter syntax highlighting for code blocks in Typst.

This aims to be a more powerful alternative to
[`syntastica-typst`](https://github.com/RubixDev/syntastica-typst) by giving up
the convenience of being directly integrated in Typst.
[`syntastica-typst`](https://github.com/RubixDev/syntastica-typst) uses a
WebAssembly plugin, which allows it to be used with a normal Typst compiler and
even in the Web App, whereas `typst-lirstings` provides a CLI that wraps calls
to the Typst CLI to query the document and run the highlighting separately. This
way it can run much faster and isn't constrained by the Wasm sandbox, but it
also limits the accessibility of where this can be used.

## Usage

Run

```bash
typst-lirstings print-typ > lirstings.typ
```

in your project once to create the `lirstings.typ` file which contains the Typst
interface of this tool. Then add the following to your document:

```typ
#import "/lirstings.typ": lirsting
#show raw: lirsting
```

This should have no effect by default, but will enable the usage of the CLI to
compile your document with enhanced syntax highlighting. To do that, run

```bash
typst-lirstings compile <file.typ>
```

The compile command reflects the interface of the typical `typst compile`
command.

### Using a different Theme

`typst-lirstings` comes bundled with
[a bunch of different themes](https://github.com/RubixDev/syntastica/blob/v0.5.0/syntastica-themes/theme_list.md).
To select which one to use, simply set the `theme` argument of the `lirsting`
function. For example:

```typ
#show raw: lirsting.with(theme: "catppuccin::latte")
```

### Adding custom Parsers

`typst-lirstings` also supports dynamically loading additional tree-sitter
parsers the same way the tree-sitter CLI does. Simply put the parser directories
in a `parsers` folder at your project root.
