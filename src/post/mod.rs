use std::path::Path;
use chrono::Datelike;
use tera::{Tera, Context};
use body::parse_md;
use lazy_static::lazy_static;
use self::head::MdHeader;

mod syntect;
pub mod body;
pub mod head;

// Build the template files:
lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();

        if let Err(e) = tera.add_template_file(Path::new("./static/post.html"), Some("post")) {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        };

        tera.autoescape_on(vec![]); // Disable auto escape.
        tera
    };
}

pub struct Doc {
    pub meta: MdHeader,
    pub content: String
}

pub fn render_post(filename: &str, doc: &str) -> Doc {
    let doc = parse_md(filename, doc).unwrap();

    // Setup the post context:
    let mut context = Context::new();
    context.insert("title", &doc.meta.title);

    let date = &doc.meta.date;
    let date_str = format!("{} / {} / {}", date.day(), date.month(), date.year());
    context.insert("date", &date_str);

    let size = &(doc.content.len() as f32 * 0.001);
    let size_str = format!("{0:.2} kB", size);
    context.insert("size", &size_str);

    context.insert("description", &doc.meta.desc);
    context.insert("article", &doc.content.trim());

    // Render the post.
    let content = TEMPLATES.render("post", &context).unwrap();

    Doc { meta: doc.meta, content }
}