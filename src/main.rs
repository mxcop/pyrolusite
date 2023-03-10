use std::path::Path;
use crate::doc::parse_md;

mod head;
mod doc;

const DOC: &'static str = include_str!("../md/test.md");

fn main() {
    let doc = parse_md(DOC).unwrap();

    // Create the build directory:
    if Path::new("./dist/").is_dir() == false {
        std::fs::create_dir("./dist/")
            .expect("failed to create `./dist`");
    }

    std::fs::write(Path::new("./dist/test.html"), doc.content).unwrap();
}
