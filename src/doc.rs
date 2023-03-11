use comrak::{ComrakOptions, ComrakPlugins, markdown_to_html_with_plugins};
use crate::{head::{MdHeader, parse_md_header}, syntect::SyntectAdapter};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX_SELECT_BODY: Regex = Regex::new(r"[\s]-{3,}([\s\S]*)").unwrap();
}

/// Markdown document.
#[derive(Debug)]
pub struct MdDoc {
    /// Document meta data.
    pub meta: MdHeader,
    /// Parsed markdown document. (HTML)
    pub content: String
}

/// Parse a markdown document to meta data and html.
pub fn parse_md(doc: &str) -> Option<MdDoc> {
    // Parse the document header.
    let Some(meta) = parse_md_header(doc) else {
        return None;
    };

    // Capture the document body:
    let Some(captures) = REGEX_SELECT_BODY.captures(doc) else {
        return None;
    };

    // Get the lines from the header.
    let body = captures[1].trim();

    // Setup the syntect adapter:
    let adapter = SyntectAdapter::new();
    let mut options = ComrakOptions::default();
    options.extension.tasklist = true;
    let mut plugins = ComrakPlugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    // Parse the markdown body.
    let formatted = markdown_to_html_with_plugins(body, &options, &plugins);

    Some(MdDoc { meta, content: formatted })
}