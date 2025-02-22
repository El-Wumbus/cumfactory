pub mod uri; // TODO: rework a bit

/// Walk a directory, performing some action on every file and directory within.
pub fn walk<F: FnMut(&std::path::Path, bool) -> std::io::Result<bool>>(
    p: impl AsRef<std::path::Path>,
    callback: &mut F,
) -> Result<(), std::io::Error> {
    let dir = p.as_ref();
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if callback(&path, true)? {
                    walk(path, callback)?;
                }
            } else {
                callback(&path, false)?;
            }
        }
    } else {
        // We don't want to ignore the first item if it's a file
        callback(dir, false)?;
    }
    Ok(())
}
