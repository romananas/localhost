use std::{fmt::Display, process::Command};
use crate::utils;
use crate::options;

pub enum ErrorStatus {
    Execution,
    Program,
    ByteConversion
}

pub struct Error {
    status: ErrorStatus,
    message: String,
}

impl Error {
    fn new(status: ErrorStatus,message: String) -> Self {
        Self { status, message}
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self.status {
            ErrorStatus::Execution => "can't execute the program",
            ErrorStatus::ByteConversion => "the program has been succesfully executed but can't convert the result into a string",
            ErrorStatus::Program => "the program returned an error",
        };
        write!(f,"{} : {}",status,self.message)
    }
}

pub fn exec(path: String,command: String,args: String) -> Result<String,Error> {
    let output = match Command::new(command).arg(path).arg(args).output() {
        Ok(output) => output,
        Err(e) => return Err(Error::new(ErrorStatus::Execution, format!("{}",e))),
    };
    if output.status.success() {
        match String::from_utf8(output.stdout) {
            Ok(s) => return Ok(s),
            Err(e) => return Err(Error::new(ErrorStatus::ByteConversion, format!("{}",e))),
        };
    } else {
        match String::from_utf8(output.stderr) {
            Ok(s) => return Err(Error::new(ErrorStatus::Program, s)),
            Err(e) => return Err(Error::new(ErrorStatus::ByteConversion, format!("{}",e))),
        };
    }
}

pub fn launch(path: &str,opts: &options::Opts,data: String) -> Result<String,Error> {
     // Exécution CGI si applicable
     let mut content = String::new();
     if let Some(ext) = utils::get_file_extention(path) {
        if let Some(cmd) = opts.cgi_binds.get(ext) {
            content = exec(
                path.trim_start_matches('/').to_string(),
                cmd.clone(),
                data,
            )?;
        }
    }
    return Ok(content);
}