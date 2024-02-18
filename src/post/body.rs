use super::{head::MdHeader, syntect::SyntectAdapter};
use comrak::{ComrakOptions, ComrakPlugins, markdown_to_html_with_plugins};
use once_cell::sync::Lazy;
use regex::Regex;

pub static REGEX_SELECT_BODY: Lazy<Regex> = Lazy::new(|| Regex::new(r"[\s]-{3,}([\s\S]*)").unwrap());

/// Markdown document.
#[derive(Debug)]
pub struct MdDoc {
    /// Document meta data.
    pub meta: MdHeader,
    /// Parsed markdown document. (HTML)
    pub content: String
}

impl MdDoc {
    /// Parse a markdown file to meta data and html.
    pub fn from_file(filename: &str, doc: &str) -> Option<MdDoc> {
        // Parse the document header.
        let Some(meta) = MdHeader::from_file(filename, doc) else {
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
}
