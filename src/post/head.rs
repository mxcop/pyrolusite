use chrono::NaiveDate;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use serde::ser::SerializeStruct;

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
    /// URL of the document.
    pub url: String,

    /// Description of the document.
    pub desc: String,
}

impl Serialize for MdHeader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut state = serializer.serialize_struct("MdHeader", 4)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("date", &self.date.to_string())?;
        state.serialize_field("url", &self.url)?;
        state.serialize_field("desc", &self.desc)?;
        state.end()
    }
}

/// Parse the header of a markdown document.
pub fn parse_md_header(filename: &str, doc: &str) -> Option<MdHeader> {
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
    let url = filename.to_string();

    Some(MdHeader { title, date, url, desc })
}
