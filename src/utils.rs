pub fn remove_extension(file_path: &str) -> &str {
    // filename.rsplit_once('.').map(|(name, _)| name).unwrap_or(filename)
    match file_path.strip_suffix(".html") {
        Some(v) => v,
        None => file_path,
    }
}

/// return the file extension of a choosen file if the file does not have an extension the function return None
pub fn get_file_extention(file_path: &str) -> Option<&str> {
    let tmp = file_path.split(".").collect::<Vec<&str>>();
    if tmp.len() < 2 {
        return None;
    } else {
        let post_point = match tmp.last().copied() {
            Some(v) => v,
            None => return None,
        };
        let ext = match post_point.split_once("?") {
            Some(v) => Some(v.0),
            None => Some(post_point),
        };
        return ext;
    }
}
/// get a file path as index.php?name=timmy and retrun an option of a (path,request) tuple
pub fn split_get_request(path: &str) -> Option<(&str,&str)>{
    let parts = path.split(".").collect::<Vec<&str>>();
    if parts.len() < 2 {
        return None;
    }
    let ext_and_req = match parts.last().copied() {
        Some(v) => v,
        None => return None,
    };
    let req_msg = match ext_and_req.split("?").last() {
        Some(v) => v,
        None => return None,
    };
    let trimmed_path = match path.strip_suffix(format!("?{}",req_msg).as_str()) {
        Some(v) => v,
        None => return None,
    };
    return Some((trimmed_path,req_msg));
}

pub fn sanatize_path(file_path: &str) -> &str {
    let file_path = match file_path.strip_prefix("/") {
        Some(s) => s,
        None => file_path,
    };
    file_path
}