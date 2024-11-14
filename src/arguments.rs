use std::env;
use std::fmt::Display;
use std::path::PathBuf;

pub struct Arguments {
    pub path: PathBuf,
    pub name_filter: Option<String>,
}

impl Arguments {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn parse(&mut self) {
        let args: Vec<String> = env::args().collect();

        self.path = PathBuf::from(args[1].clone());

        let argc = args.len();
        let mut i = 2;
        while i != argc {
            let curr_arg = &args[i];
            if curr_arg == "-name" {
                self.name_filter = args.get(i + 1).cloned();
                i += 1;
            }
            i += 1;
        };
    }
}

impl Display for Arguments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Path: {:?}\nName filter: {:?}\n", self.path.to_str(), self.name_filter)
    }
}

impl Default for Arguments {
    fn default() -> Self {
        Arguments {
            path: PathBuf::new(),
            name_filter: None,
        }
    }
}
