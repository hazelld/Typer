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

    // Get input on seperate thread
    thread::spawn(move || {
        loop {

            chan_select! {
                default => {
                    let ch = getch();
            
                    if ch > 0 {
                        input_block.write_block(((ch as u8) as char).to_string());
                        
                        word_block.clear_block();
                        handler::scroll_wordlist_up(&mut words, 6);
                        wordlist::write_wordlist(word_block, &mut words);
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

