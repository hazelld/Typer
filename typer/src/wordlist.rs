extern crate rand;

use std::io::BufReader;
use std::fs::File;
use std::str;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;
use self::rand::Rng;
use self::rand::thread_rng;

use view::*;

pub struct Word {
    pub word: String,
    pub state: Colour,
    pub location: Point,
}

impl Word {
    pub fn new(word: String, state: Colour, location: Point) -> Word {
        Word { word: word, state: state, location: location}
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
        vec.push(Word::new(line, Colour::Nothing, Point::new(0,0)));
    }
    
    //Randomize the vector and return
    thread_rng().shuffle(vec.as_mut_slice());
    vec
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


