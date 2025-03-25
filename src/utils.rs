pub fn remove_extension(filename: &str) -> &str {
    filename.rsplit_once('.').map(|(name, _)| name).unwrap_or(filename)
}