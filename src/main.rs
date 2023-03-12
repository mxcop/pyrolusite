use std::path::Path;
use blog::{render_blog, home::render_home};

mod post;
mod blog;

/// Used for copying styles to the build directory.
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
    let posts = render_blog(Path::new("./md/"));

    let home = render_home(&posts);

    std::fs::write(Path::new("./dist/index.html"), &home).expect("failed to save home page");

    // Copy all stylesheets to the `/dist` directory.
    copy_recursively(Path::new("./styles/"), Path::new("./dist/")).unwrap();
}
