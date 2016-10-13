
use view::*;
use wordlist::*;

struct Handler {
    ustr: String,
    cur_word: &mut Word,
    update: bool,
    match_flag: bool,
}

// Returns true if the string and word match
pub fn compare ( user_string: String, word: Word ) -> bool {
    return user_string == word.word;
}

// Remove the last character from the string and return it, for when user
// uses backspace
pub fn remove_last_char (user_string: String) -> String {
    let mut s = user_string;
    s.pop();
    s
}

// Add character to the string 
pub fn add_char_end (user_string: String, c: char) -> String {
    let mut s = user_string;
    s.push(c);
    s
}

// Update the colour based on if the words match or not
pub fn update_colour ( words_match: bool, word: Word ) -> Word {
    if words_match {
        Word::new(word.word, Colour::Correct, word.location)
    } else {
        Word::new(word.word, Colour::Incorrect, word.location)
    }
}

// Update the word and return the result of the attempted update
pub fn update_word ( word: Word, mut wordblock: Block ) -> bool {
    wordblock.update_block(word.location, word.word, word.state) 
}


// Remove the first row of the wordlist
pub fn scroll_wordlist_up (words: &mut Vec<Word>, first_row: i32) {
    words.retain(|n| n.location.get_y() != first_row);
}
