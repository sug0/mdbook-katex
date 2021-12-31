use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use serde_derive::{Deserialize, Serialize};
use toml;

use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::errors::Result;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::renderer::{RenderContext, Renderer};

#[derive(Deserialize, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct KatexConfig {
    // options for the katex-rust crate
    pub leqno: bool,
    pub fleqn: bool,
    pub throw_on_error: bool,
    pub error_color: String,
    pub min_rule_thickness: f64,
    pub max_size: f64,
    pub max_expand: i32,
    pub trust: bool,
    // other options
    pub static_css: bool,
    pub macros: Option<String>,
}

impl Default for KatexConfig {
    fn default() -> KatexConfig {
        KatexConfig {
            // default options for the katex-rust crate
            // uses defaults specified in: https://katex.org/docs/options.html
            leqno: false,
            fleqn: false,
            throw_on_error: true,
            error_color: String::from("#cc0000"),
            min_rule_thickness: -1.0,
            max_size: f64::INFINITY,
            max_expand: 1000,
            trust: false,
            // other options
            static_css: false,
            macros: None,
        }
    }
}

// ensures that both the preprocessor and renderers are enabled
// in the `book.toml`; the renderer forces mdbook to separate all
// renderers into their respective directories, ensuring that the
// html renderer will always be at `{out_dir}/html`
fn enforce_config(cfg: &mdbook::Config) {
    if cfg.get("preprocessor.katex").is_none() {
        panic!("Missing `[preprocessor.katex]` directive in `book.toml`!");
    }
    if cfg.get("output.katex").is_none() {
        panic!("Missing `[output.katex]` directive in `book.toml`!");
    }
    if cfg.get("output.html").is_none() {
        panic!("The katex preprocessor is only compatible with the html renderer!");
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
            } else if '_' == c {
                result.push('\\');
            }
            result.push(c);
            past_char = c;
        }
        return result;
    }
}

pub fn get_macro_path(root: &PathBuf, macros_path: &Option<String>) -> Option<PathBuf> {
    match macros_path {
        Some(path) => Some(root.join(PathBuf::from(path))),
        _ => None,
    }
}

pub fn get_config(book_cfg: &mdbook::Config) -> Result<KatexConfig, toml::de::Error> {
    let cfg = match book_cfg.get("preprocessor.katex") {
        Some(raw) => raw.clone().try_into(),
        None => Ok(KatexConfig::default()),
    };
    cfg.or_else(|_| Ok(KatexConfig::default()))
}

pub fn load_as_string(path: &Path) -> String {
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut string = String::new();
    match file.read_to_string(&mut string) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => (),
    };
    string
}

#[cfg(test)]
mod tests;
