pub mod fs;

use spongedown;


///
///
/// Wrapper around the pulldown-cmark parser and renderer to render markdown

pub fn render_markdown(text: &str) -> String {
    spongedown::parse(text)
}
