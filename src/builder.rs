use std::{path::Path, io, fs};
use css_minify::optimizations::{Minifier, Level};

use super::blog::{render_blog, home::render_home};

/// The build command used by the CLI.
pub fn build(path: &String, output: &String, styles: &String) -> io::Result<()> {
    fs::create_dir_all(Path::new(output))?;

    let posts = render_blog(Path::new(path), Path::new(output))?;
    let home = render_home(&posts);

    fs::write(Path::new(output).join("./index.html"), &home)?;

    let style_path = Path::new(path).join(styles);
    if style_path.exists() {
        // Copy all stylesheets to the `/dist` directory.
        copy_recursively(style_path, Path::new(output))?;
    }

    Ok(())
}

/// Used for copying styles to the build directory.
fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();
        let filename = entry.file_name();

        // We need to go deeper...
        if path.is_dir() {
            copy_recursively(&path, destination.as_ref().join(&filename))?;
            continue;
        }

        // Copy the file.
        let Some(ext) = path.extension() else {
            continue;
        };

        // Minify if the file is .css
        if ext == "css" {
            let file = fs::read_to_string(&path)?;
            let minified = Minifier::default().minify(&file, Level::Three).expect("failed to minify css");
            fs::write(destination.as_ref().join(&filename), &minified)?;
        } else {
            if let Ok(file) = fs::read_to_string(&path) {
                fs::write(destination.as_ref().join(&filename), &file)?;
            } else {
                let file = fs::read(&path)?;
                fs::write(destination.as_ref().join(&filename), &file)?;
            }
        };
    }
    Ok(())
}
