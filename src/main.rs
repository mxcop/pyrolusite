use std::path::Path;
use render::render_post;
use doc::parse_md;

mod head;
mod doc;
mod render;
mod syntect;

const DOC: &'static str = include_str!("../md/test.md");

fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::create_dir_all(&destination)?;
    for entry in std::fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() {
    let doc = parse_md(DOC).unwrap();

    // Create the build directory:
    if Path::new("./dist/").is_dir() == false {
        std::fs::create_dir("./dist/")
            .expect("failed to create `./dist`");
    }

    let post = render_post(&doc);

    std::fs::write(Path::new("./dist/test.html"), &post).unwrap();

    // Copy all stylesheets to the `/dist` directory.
    copy_recursively(Path::new("./styles/"), Path::new("./dist/")).unwrap();
}
