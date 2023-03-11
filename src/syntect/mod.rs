use comrak::adapters::SyntaxHighlighterAdapter;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

pub fn build_opening_tag(tag: &str, attributes: &HashMap<String, String>) -> String {
    let mut tag_parts = vec![format!("<{}", tag)];

    for (attr, val) in attributes {
        tag_parts.push(format!(" {}=\"{}\"", attr, val));
    }

    tag_parts.push(String::from(">"));

    tag_parts.join("")
}

pub fn extract_attributes_from_tag(html_tag: &str) -> HashMap<String, String> {
    let re = regex::Regex::new("([a-zA-Z_:][-a-zA-Z0-9_:.]+)=([\"'])(.*?)([\"'])").unwrap();

    let mut attributes: HashMap<String, String> = HashMap::new();

    for caps in re.captures_iter(html_tag) {
        attributes.insert(String::from(&caps[1]), String::from(&caps[3]));
    }

    attributes
}

#[derive(Debug)]
/// Syntect syntax highlighter plugin.
pub struct SyntectAdapter {
    theme: Theme,
    syntax_set: SyntaxSet,
}

impl SyntectAdapter {
    /// Construct a new `SyntectAdapter` object and set the syntax highlighting theme.
    pub fn new() -> Self {
        SyntectAdapter {
            theme: ThemeSet::get_theme(Path::new("./src/syntect/earthsong.tmTheme")).unwrap(),
            syntax_set: SyntaxSet::load_defaults_newlines()
        }
    }

    fn gen_empty_block(&self) -> String {
        let syntax = self.syntax_set.find_syntax_by_name("Plain Text").unwrap();
        match highlighted_html_for_string(
            "",
            &self.syntax_set,
            syntax,
            &self.theme,
        ) {
            Ok(empty_block) => empty_block,
            Err(_) => "".into(),
        }
    }

    fn remove_pre_tag(&self, highlighted_code: String) -> String {
        let re: Regex = Regex::new("<pre[\\s]+.*?>").unwrap();

        re.replace_all(highlighted_code.as_str(), "")
            .to_string()
            .replace("</pre>", "")
    }
}

impl SyntaxHighlighterAdapter for SyntectAdapter {
    fn highlight(&self, lang: Option<&str>, code: &str) -> String {
        let fallback_syntax = "Plain Text";

        let lang: &str = match lang {
            None => fallback_syntax,
            Some(l) => {
                if l.is_empty() {
                    fallback_syntax
                } else {
                    l
                }
            }
        };

        let syntax = self
            .syntax_set
            .find_syntax_by_token(lang)
            .unwrap_or_else(|| {
                self.syntax_set
                    .find_syntax_by_first_line(code)
                    .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text())
            });

        match highlighted_html_for_string(
            code,
            &self.syntax_set,
            syntax,
            &self.theme,
        ) {
            Ok(highlighted_code) => self.remove_pre_tag(highlighted_code),
            Err(_) => code.into(),
        }
    }

    fn build_pre_tag(&self, attributes: &HashMap<String, String>) -> String {
        let mut syntect_attributes = extract_attributes_from_tag(self.gen_empty_block().as_str());

        for (comrak_attr, val) in attributes {
            let mut combined_attr: String = val.clone();

            if syntect_attributes.contains_key(comrak_attr.as_str()) {
                combined_attr = format!(
                    "{} {}",
                    syntect_attributes.remove(comrak_attr).unwrap(),
                    val
                );
            }

            syntect_attributes.insert(comrak_attr.clone(), combined_attr);
        }

        build_opening_tag("pre", &syntect_attributes)
    }

    fn build_code_tag(&self, attributes: &HashMap<String, String>) -> String {
        build_opening_tag("code", attributes)
    }
}
