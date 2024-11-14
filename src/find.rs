use std::{fs::{self, DirEntry}, path::Path};

use crate::arguments::Arguments;

pub struct Find<'a> {
    args: &'a Arguments,
    found_entries: Vec<DirEntry>,
}

impl<'a> Find<'a> {
    pub fn new(_args: &'a Arguments) -> Self {
        Find {
            args: _args,
            found_entries: Vec::new(),
        }
    }

    fn recursive_through_dir(&mut self, dir: &Path) {
        if dir.is_dir() {
            match fs::read_dir(dir) {
                Ok(read_dir) => {
                    for entry in read_dir {
                        let dir_entry = match entry {
                            Ok(e) => e,
                            Err(_) => continue,
                        };
                        if dir_entry.file_type().unwrap().is_dir() {
                            self.recursive_through_dir(dir_entry.path().as_path());
                        }
                        // filter by name
                        if self.args.name_filter.is_some() && dir_entry.file_name().into_string() == Ok(self.args.name_filter.clone().unwrap()) {
                            self.found_entries.push(dir_entry);
                        }
                    }
                },
                Err(e) => eprintln!("{}: {e}", dir.to_str().unwrap_or(""))
            }
        }
    }

    pub fn exec(&mut self) -> Result<(), std::io::Error> {
        let dir = self.args.path.as_path();

        self.recursive_through_dir(dir);
        Ok(())
    }

    pub fn display_found_entries(&self) {
        for entry in &self.found_entries {
            println!("{}", entry.path().to_str().unwrap_or(""));
        }
    }
}