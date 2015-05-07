use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::iter::Iterator;
use std::string::String;
use std::collections::HashMap;

mod db {
    pub struct TxtTable {
        name: String,
        contents: HashMap<String, String>,
    }

    impl TxtTable {
        fn read_file(file_name: &str) -> String {
            let path = Path::new(file_name);
            let display = path.display();
            // Open in read-only mode, returns `io::Result<File>`
            let mut file = match File::open(&path) {
                Err(why) => panic!("Failed to open file {}. Reason: {}", display, Error::description(&why)),
                Ok(line) => line,
            };
            let mut file_contents = String::new();
            file.read_to_string(&mut file_contents).unwrap();
            return file_contents;
        }

        fn is_comment(line: &str) -> bool {
            return line.starts_with("//");
        }

        fn split_line(line: &str) -> Vec<&str> {
            return line.splitn(2, ':').map(|x| x.trim_matches(' ')).collect();
        }
        pub fn new(table_name: &str) -> TxtTable {
            let mut contents = HashMap::new();
            let file_contents: String = TxtTable::read_file(table_name);
            let mut lines_in_file = file_contents.lines().collect::<Vec<&str>>();
            for line_iter in lines_in_file.iter() {
                let line = line_iter.trim_matches(' ');
                if TxtTable::is_comment(line) {
                    continue;
                }
                let tokens = TxtTable::split_line(line);
                contents.insert(tokens[0].to_string(), tokens[1].to_string());
            }

            return TxtTable {name: table_name.to_string(), contents: contents};
        }
    }
}