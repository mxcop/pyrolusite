use std::path::Path;
use walk::walk_posts;
use crate::post::{head::MdHeader, render_post};

pub mod home;
mod walk;

/// Render the blog.
pub fn render_blog(dir: &Path, out: &Path) -> Vec<MdHeader> {
    let mut posts: Vec<MdHeader> = Vec::new();

    walk_posts(&mut posts, dir, out, &|path| {
        let doc = std::fs::read_to_string(path).unwrap();
        let filename = format!("{}.html", path.file_stem().expect("failed to read filename").to_string_lossy());
        render_post(&filename, &doc)
    });

    posts
}