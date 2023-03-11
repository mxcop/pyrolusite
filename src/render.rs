use std::path::Path;
use chrono::Datelike;
use tera::{Tera, Context};
use crate::doc::MdDoc;

pub fn render_post(doc: &MdDoc) -> String {
    // Load the template file:
    let mut tera = Tera::default();
    tera.add_template_file(Path::new("./static/tmpl.html"), Some("post")).unwrap();
    tera.autoescape_on(vec![]); // Disable auto escape.

    // Setup the post context:
    let mut context = Context::new();
    context.insert("title", &doc.meta.title);

    let date = &doc.meta.date;
    let date_str = format!("{}/{}/{}", date.day(), date.month(), date.year());
    context.insert("date", &date_str);

    let size = &(doc.content.len() as f32 * 0.001);
    let size_str = format!("{0:.2}kB", size);
    context.insert("size", &size_str);

    context.insert("description", &doc.meta.desc);
    context.insert("article", &doc.content.trim());

    // Render the post.
    tera.render("post", &context).unwrap()
}