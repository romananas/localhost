use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{Read, Write, Seek, SeekFrom},
};
use super::parse::*;

pub struct Hosts {
    file: File,
    content: String,
}

impl Hosts {
    pub fn add(hosts: HashMap<Domain, Address>) -> Result<Self, Box<dyn std::error::Error>> {
        // Ouvre en lecture + écriture
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/etc/hosts")?;

        // Lis tout le fichier proprement
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        // Construit les nouvelles lignes à ajouter
        let mut to_write = String::new();
        for (domain, address) in hosts {
            if is_taken(domain.clone()).unwrap_or(false) {
                continue;
            }
            to_write += &format!("{} {}\n", address, domain);
        }

        // Écrit à la fin du fichier
        file.seek(SeekFrom::End(0))?;
        file.write_all(to_write.as_bytes())?;

        Ok(Hosts { file, content })
    }

    pub fn remove(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Revenir au début et vider le fichier
        self.file.set_len(0)?;
        self.file.seek(SeekFrom::Start(0))?;
        self.file.write_all(self.content.as_bytes())?;
        Ok(())
    }
}
