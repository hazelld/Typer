#[macro_use]
extern crate chan;
extern crate ncurses;

use std::thread;
use std::char;
use ncurses::*;
use view::*;
use handler::*;
use wordlist::*;

mod wordlist;
mod view;
mod handler;

pub struct Stats {
    correct_chars: u32,
    incorrect_chars: u32,
}

fn main() {
    
    view::init_view();

    let mut words: Vec<Word> = wordlist::load_wordlist("wordlist.txt");
    
    let mut word_block = Block::new( 
                        Point::new(5, 5),
                        Point::new(75, 5),
                 );

    let mut input_block = Block::new( 
                    Point::new(5, 15),
                    Point::new(15, 1),
                 );

    let mut time_block = Block::new(
                            Point::new(50, 15),
                            Point::new(2,1),
                            );
    
    let mut sec_left = 60;

    // Fill up the screen
    word_block.draw_block();
    input_block.draw_block();
    time_block.draw_block();
    wordlist::write_wordlist(word_block, &mut words);
    time_block.write_block(sec_left.to_string());
    
    
    let end = chan::tick_ms(60000);
    let timer = chan::tick_ms(1000);
    let (send, recv) = chan::sync(0);
    
    // Keep track of user's string and current word
    let mut user_str = String::new();
    let mut cur_word = words[0];

    // Get input on seperate thread
    thread::spawn(move || {
        loop {

            chan_select! {
                default => {
                    let ch = getch();
            
                    if ch > 0 {
                        user_str = handle_input(cur_word, user_str, input_block, 
                                                ch as u8, word_block);
                    }
                },

                // Exit thread when given signal 
                recv.recv() => {
                    return;
                },
            }
        }
    });

    loop {

        chan_select! {

            timer.recv() => {
                sec_left = sec_left - 1;
                time_block.clear_block();
                time_block.write_block(sec_left.to_string());
            },
            
            end.recv() => {
                send.send(());
                endwin();
                return;
            },
            
        }
    }
}

fn handle_input (cur_word: Word,/*, word_list: &mut Vec<Word>,*/ user_str: String,
                 mut in_block: Block, input_char: u8, mut word_block: Block) -> String {
    word_block.clear_block();
    
    match input_char {       
        127 => return "backspace".to_string(),
        27 => return "exit".to_string(),
        32 => return "space".to_string(),
        97 ... 122 => char_input(user_str, word_block, in_block, input_char as char, cur_word),
        _ => return user_str,
    };
    
}

fn char_input (user_str: String, mut word_block: Block, mut in_block: Block, 
               input_char: char, cur_word: Word) -> String {
    let ustr = add_char_end(user_str, input_char);
    compare(ustr, cur_word).update_colour(cur_word).update_word(word_block);
    ustr
}
