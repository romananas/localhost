pub fn remove_extension(file_path: &str) -> &str {
    // filename.rsplit_once('.').map(|(name, _)| name).unwrap_or(filename)
    match file_path.strip_suffix(".html") {
        Some(v) => v,
        None => file_path,
    }
}

/// return the file extension of a choosen file if the file does not have an extension the function return None
pub fn get_file_extention(file_path: &str) -> Option<&str> {
    let ext = file_path.split(".").collect::<Vec<&str>>();
    if ext.len() < 2 {
        return None;
    } else {
        return ext.last().copied();
    }
}

pub fn sanatize_path(file_path: &str) -> &str {
    let file_path = match file_path.strip_prefix("/") {
        Some(s) => s,
        None => file_path,
    };
    file_path
}