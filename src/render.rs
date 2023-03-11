use std::path::Path;
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
    context.insert("date", &doc.meta.date.to_string());
    context.insert("description", &doc.meta.desc);
    context.insert("article", &doc.content.trim());

    // Render the post.
    tera.render("post", &context).unwrap()
}