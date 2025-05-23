use std::collections::HashMap;
use std::fs::read_dir;
use std::path::Path;
use std::fs::File;
use std::io::Write;

use super::utils;



pub fn parse_dir(dir: &str) -> Vec<String> {
    let mut list = Vec::new();
    visit_dirs(Path::new(dir), &mut list);
    // dbg!("{}",list.clone());
    list
}

fn visit_dirs(dir: &Path, list: &mut Vec<String>) {
    if let Ok(entries) = read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() { // Ajoute uniquement les fichiers
                list.push(path.display().to_string());
            } else if path.is_dir() {
                visit_dirs(&path, list); // Appel récursif pour les sous-dossiers
            }
        }
    }
}



pub fn write_file(dir: String,fname: String,data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = match File::create(format!("{}{}",dir,fname)) {
        Ok(file) => file,
        Err(e) => return Err(Box::new(e)),
    };
    match file.write(&data) {
        Ok(_) => {},
        Err(e) => return Err(Box::new(e)),
    };
    Ok(())
}

pub fn parse_files(index: String) -> HashMap<String,String>{
    let mut links: HashMap<String,String> = HashMap::new();
    // sanaitizing files paths for later use
    let files_paths = parse_dir(".");
    let paths: Vec<_> = files_paths
        .iter()
        .filter_map(|fp| fp.strip_prefix(".").map(|s| s.to_string()))
        .collect();
    let files_paths = files_paths.iter().map(|f| f.strip_prefix("./").unwrap_or(f).to_string()).collect::<Vec<String>>();

    for (path,file_path) in paths.iter().zip(files_paths) {
        if links.values().any(|v| v.contains(&file_path)) {
            continue;
        }
        if file_path.contains(index.as_str()) {
            links.insert("/".to_string(), file_path);
            continue;
        }
        let tmp = utils::remove_extension(&path);
        links.insert(tmp.to_string().clone(), file_path.clone());
    }
    return links;
}