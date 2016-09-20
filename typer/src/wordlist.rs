use std::io::BufReader;
use std::fs::File;
use std::str;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;

use view::*;

pub struct Word {
    word: String,
    state: Colour,
    location: Point,
}

impl Word {
    fn new(word: String) -> Word {
        Word { word: word, state: Colour::Nothing, location: Point::new(0,0) }
    }
}

// Load the wordlist from a file, and randomize it
pub fn load_wordlist (filename: &str) -> Vec<Word> {
    
    let path = Path::new(filename);
    let display = path.display();

    let file = match File::open(&path) {
        
        Err(e) => panic!("Couldn't open {}: {}", display, e.description()),
        Ok(file) => file,
    };

    let fh = BufReader::new(file);

    let mut vec = Vec::new();
    for line in fh.lines().filter_map(|result| result.ok()) {
        vec.push(Word::new(line));
    }
    
    return vec;
}

// Write the wordlist to the proper view
pub fn write_wordlist (mut word_view: Block, words: &mut Vec<Word>) {
    
    let mut done = false;
    let mut index = 0;

    while !done {
        
        //UNSAFE
        let wordref = &mut words[index]; 
        let text = wordref.word.clone();
        
        let (location, result) = word_view.write_block(text);
                
        if result == false { 
            done = true; 
        } else {
            wordref.location = location;
            index += 1;
        }

        // Add a space after the word, find better solution for this
        word_view.write_block(" ".to_string());
    }
}


