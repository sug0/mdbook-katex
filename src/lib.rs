use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::errors::Result;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::renderer::{RenderContext, Renderer};

// ensures that both the preprocessor and renderers are enabled
// in the `book.toml`; the renderer forces mdbook to separate all
// renderers into their respective directories, ensuring that the
// html renderer will always be at `{out_dir}/html`
fn enforce_config(cfg: &mdbook::Config) {
    if cfg.get("preprocessor.katex").is_none() {
        panic!("Missing `[preprocessor.katex]` directive in `book.toml`!");
    }
}

pub struct KatexProcessor;

// dummy renderer to ensure rendered output is always located
// in the `book/html/` directory
impl Renderer for KatexProcessor {
    fn name(&self) -> &str {
        "katex"
    }

    fn render(&self, ctx: &RenderContext) -> Result<()> {
        enforce_config(&ctx.config);
        Ok(())
    }
}

// preprocessor to inject rendered katex blocks and stylesheet
impl Preprocessor for KatexProcessor {
    fn name(&self) -> &str {
        "katex"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        // enforce config requirements
        enforce_config(&ctx.config);
        // parse TOML config
        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                chapter.content = self.process_chapter(&chapter.content)
            }
        });
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html" || renderer == "katex"
    }
}

impl KatexProcessor {
    // render Katex equations in HTML, and add the Katex CSS
    fn process_chapter(&self, raw_content: &str) -> String {
        return Self::render(&raw_content);
    }

    fn render(string: &str) -> String {
        let mut result = String::new();

        let mut past_char = '\0';
        let mut in_codeblocks = false;
        let mut in_math = false;
        for c in string.chars() {
            // println!("{}", temp_string);
            if '`' == c {
                if '`' == past_char {
                    // do nothing: skip muti-backtick
                } else {
                    in_codeblocks = !in_codeblocks;
                }
            } else if in_codeblocks {
                // do nothing: skip codeblocks
            } else if '$' == c {
                if '$' != past_char {
                    in_math = !in_math;
                }
            } else if in_math {
                if '_' == c {
                    result.push('\\');
                } else if '*' == c {
                    result.push('\\');
                } else if '\\' == c {
                    result.push('\\');
                }
            }
            result.push(c);
            past_char = c;
        }
        return result;
    }
}

#[cfg(test)]
mod tests;
