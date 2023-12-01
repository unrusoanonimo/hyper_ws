pub fn get_extension(file: &str) -> Option<&str> {
    let mut chars = file.chars();

    let mut poss = 1;

    while chars.next_back()? != '.' {
        poss += 1;
    }
    Some(&file[file.len() - poss..])
}