#![allow(unused)]

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::path::Path;
use std::env;

struct Journal {
    title: String,
    entries: Vec<String>,
}

impl Journal {
    pub fn new(title: String) -> Journal {
        Journal {
            title,
            entries : Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: &str) {
        self.entries.push(entry.to_string());
    }
}

struct PersistanceManager {}
impl PersistanceManager {
    pub fn save(&self, j: &Journal, filename: &str) {
        let cwd = env::current_dir().expect("can't get cwd.");
        let fname = format!("{}/{}", cwd.display(), filename);
        
        let mut f = File::create(fname).expect("can't create file");

        for e in &j.entries {
            writeln!(f, "{}", e);
        }
    }
}

fn main() {

    let mut j = Journal::new(String::from("Dear diary"));
    j.add_entry("i ate a bug");
    j.add_entry("i cried today");
    j.add_entry("it worked");

    let pm = PersistanceManager{};
    pm.save(&j, "file.txt");

}
