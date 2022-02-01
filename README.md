# mdbook-katex

A preprocessor for [mdBook](https://github.com/rust-lang/mdBook), base on [lzanini/mdbook-katex](https://github.com/lzanini/mdbook-katex).

To LaTeX render correctly, it does simple text replacement, such as `_` with `\_`, or `*` with `\*`.

Another version without mdbook is [here](https://github.com/rogeryoungh/preprocessor-md-tex), which I use in hugo.

## Usage

First, install `mdbook-katex`

```bash
$ cargo install --git "https://github.com/rogeryoungh/mdbook-katex"
```


Add the following lines to your book.toml file

```toml
[preprocessor.katex]
```
