use std::{fs, path::Path, io};

use crate::post::{head::MdHeader, Doc};

/// Recursively walk through all posts in the markdown directory.
pub fn walk_posts(posts: &mut Vec<MdHeader>, dir: &Path, out: &Path, cb: &dyn Fn(&Path) -> Doc) -> io::Result<()> {
    if dir.is_dir() == false {
        return Ok(());
    }

    let dir = fs::read_dir(dir)?;

    for entry in dir {
        let path = entry?.path();

        // Keep walking if directory...
        if path.is_dir() {
            walk_posts(posts, &path, out, cb)?;
            continue;
        }

        // Parse if it is a post.
        let doc = cb(&path.as_path());
        posts.push(doc.meta);

        // Save the post.
        let Some(filename) = path.file_stem() else {
            continue;
        };
        fs::write(
            out.join(format!("./{}.html", filename.to_string_lossy())),
            doc.content,
        )?;
    }

    Ok(())
}
