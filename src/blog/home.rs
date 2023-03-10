use std::{path::Path, str::from_utf8};
use tera::{Tera, Context};
use lazy_static::lazy_static;
use crate::post::head::MdHeader;

// Build the template files:
lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();

        if let Err(e) = tera.add_template_file(Path::new("./static/home.html"), Some("home")) {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        };

        tera.autoescape_on(vec![]); // Disable auto escape.
        tera
    };
}

pub fn render_home(posts: &Vec<MdHeader>) -> String {
    // Setup the post context:
    let mut context = Context::new();
    context.insert("posts", posts);

    // Render the home page.
    let page = TEMPLATES.render("home", &context).expect("failed to render home");

    from_utf8(&minify_html::minify(
        page.as_bytes(),
        &minify_html::Cfg::default(),
    ))
    .expect("failed to minify home page").to_string()
}