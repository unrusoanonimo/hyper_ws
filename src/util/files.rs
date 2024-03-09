use std::path::Path;

pub fn get_extension(file: &str) -> Option<&str> {
    let mut chars = file.chars();

    let mut poss = 1;

    while chars.next_back()? != '.' {
        poss += 1;
    }
    Some(&file[file.len() - poss..])
}
pub fn is_filename(s: &str) -> bool {
    !["/", r"\", ":", ".."]
        .into_iter()
        .any(|sub| s.contains(sub))
}
pub fn path_in_dir(dir: &Path, file: &Path) -> bool {
    let d = if let Ok(d) = dir.canonicalize() {
        d
    } else {
        return false;
    };

    let f = if let Ok(f) = file.canonicalize() {
        f
    } else {
        return false;
    };
    f.starts_with(d)
}
