
use view::*;
use wordlist::*;


// Returns true if the string and word match
pub fn compare ( user_string: String, word: Word ) -> bool {
    return user_string == word.word;
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
