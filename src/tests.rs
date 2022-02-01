use super::*;

#[test]
fn test_name() {
    let pre = KatexProcessor;
    let preprocessor: &dyn Preprocessor = &pre;
    assert_eq!(preprocessor.name(), "katex")
}

#[test]
fn test_support_html() {
    let preprocessor = KatexProcessor;
    assert!(preprocessor.supports_renderer("html"));
    assert!(!preprocessor.supports_renderer("other_renderer"))
}

#[test]
fn test_rendering_without_math() {
    let preprocessor = KatexProcessor;
    let raw_content = r"Some text, and more text.";
    let expected_output = String::from("Some text, and more text.");
    let rendered_content = preprocessor.process_chapter(&raw_content);
    debug_assert_eq!(expected_output, rendered_content);
}

#[test]
fn test_rendering_with_math() {
    let preprocessor = KatexProcessor;
    let raw_content = r"A simple fomula, $\sum_{n=1}^\infty \frac{1}{n^2} = \frac{\pi^2}{6}$.";
    let expected_output = String::from(
        r"A simple fomula, $\\sum\_{n=1}^\\infty \\frac{1}{n^2} = \\frac{\\pi^2}{6}$.",
    );
    let rendered_content = preprocessor.process_chapter(&raw_content);
    debug_assert_eq!(expected_output, rendered_content);
}

#[test]
fn test_rendering_underscore() {
    let preprocessor = KatexProcessor;
    let raw_content = r"A simple `f_f_f`, f_f_f, f`f$f_$f_` fomula, $\sum_{n=1}^\infty\\$.";
    let expected_output =
        String::from(r"A simple `f_f_f`, f_f_f, f`f$f_$f_` fomula, $\\sum\_{n=1}^\\infty\\\\$.");
    let rendered_content = preprocessor.process_chapter(&raw_content);
    debug_assert_eq!(expected_output, rendered_content);
}

#[test]
fn test_rendering_braces() {
    let preprocessor = KatexProcessor;
    let raw_content = r"define $\{a_i\} = \{ a \mid a * b * c \}$.";
    let expected_output = String::from(r"define $\\{a\_i\\} = \\{ a \\mid a \* b \* c \\}$.");
    let rendered_content = preprocessor.process_chapter(&raw_content);
    debug_assert_eq!(expected_output, rendered_content);
}
