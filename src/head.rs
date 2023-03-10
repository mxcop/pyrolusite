use chrono::NaiveDate;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX_SELECT_HEADER: Regex = Regex::new(r"([\s\S]*?)[\s]-{3,}").unwrap();
}

/// Markdown document header.
#[derive(Debug)]
pub struct MdHeader {
    /// Title of the document.
    pub title: String,
    /// Date when the doc was published.
    pub date: NaiveDate,

    /// Description of the document.
    pub desc: String,
}

/// Parse the header of a markdown document.
pub fn parse_md_header(doc: &str) -> Option<MdHeader> {
    // Capture the document header:
    let Some(captures) = REGEX_SELECT_HEADER.captures(doc) else {
        return None;
    };

    // Get the lines from the header.
    let mut header = captures[1].lines();

    // Extract the header components:
    let title = header.next().unwrap_or("Not Found").to_string();
    let date = header.next().unwrap_or("Unknown");
    let desc = header
        .filter(|&line| !line.is_empty())
        .collect::<Vec<&str>>()
        .join("\n\r");

    // Parse the date into a naive date.
    let date = NaiveDate::parse_from_str(date, "%d/%m/%Y").unwrap_or(NaiveDate::MIN);

    Some(MdHeader { title, date, desc })
}
