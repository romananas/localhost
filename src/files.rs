use std::fs::read_dir;
use std::path::Path;

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
