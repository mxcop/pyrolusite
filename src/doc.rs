use pulldown_cmark::{Parser, html::push_html};
use crate::head::{MdHeader, parse_md_header};
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

    // Parse the markdown to html:
    let parser = Parser::new(body);
    let mut html_body = String::new();
    push_html(&mut html_body, parser);

    Some(MdDoc { meta, content: html_body })
}