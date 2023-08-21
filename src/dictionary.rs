use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Dictionary {
    word_list: HashSet<String>,
}

impl Dictionary {
    pub fn new(file_name: &str) -> Dictionary {
        Dictionary {
            word_list: Self::parse_file(file_name),
        }
    }

    fn parse_file(file_name: &str) -> HashSet<String> {
        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file);

        reader
            .lines()
            .map_while(Result::ok)
            .filter(|line| !line.is_empty())
            .collect()
    }

    // TODO: Handle lower case checks
    pub fn is_legal_word(&self, word: &str) -> bool {
        self.word_list.contains(word)
    }
}
