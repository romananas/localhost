use std::process::Command;

pub enum Method {
    GET,
    POST,
    DELETE,
}

#[derive(Debug,PartialEq, Eq)]
pub enum Lang {
    Python,
    Php,
    Unknown
}

pub fn get_lang(file: String) -> Lang {
    let ext = match file.split(".").last() {
        Some(v) => v,
        None => return Lang::Unknown,
    };
    match ext {
        "py" => Lang::Python,
        "php" => Lang::Php,
        _ => Lang::Unknown
    }
}

pub fn exec(path: String,m: Method,c: String,lang: Option<Lang>) -> String {
    let lang = match lang {
        Some(v) => v,
        None =>{
            get_lang(path.clone())
        }
    };
    let output = match lang {
        Lang::Python => Command::new("python3").arg(path).output().expect("failed to execute process"),
        _ => return String::from("unkown"),
    };
    let v = String::from_utf8(output.stdout).unwrap();
    println!("le script a dit: \"{}\"",v);
    v
}