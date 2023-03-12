use std::{fs, path::Path};

use crate::post::{head::MdHeader, Doc};

/// Recursively walk through all pages in the pages directory.
pub fn walk_posts(posts: &mut Vec<MdHeader>, dir: &Path, cb: &dyn Fn(&Path) -> Doc) {
    if dir.is_dir() {
        let Ok(dir) = fs::read_dir(dir) else {
            panic!("couldn't read dir");
        };

        for entry in dir {
            let Ok(entry) = entry else {
                panic!("couldn't read file");
            };
            let path = entry.path();
            if path.is_dir() {
                walk_posts(posts, &path, cb);
            } else {
                // Parse the post.
                let doc = cb(&entry.path().as_path());
                posts.push(doc.meta);

                // Save the post.
                let filename = path.file_stem().expect("couldn't read file name");
                fs::write(
                    Path::new("./dist/").join(format!("{}.html", filename.to_string_lossy())),
                    doc.content,
                )
                .expect("couldn't save document");
            }
        }
    }
}
