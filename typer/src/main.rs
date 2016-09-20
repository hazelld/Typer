extern crate ncurses;

use ncurses::*;
use view::*;

mod wordlist;
mod view;

fn main() {
    
    view::init_view();

    let words = &mut wordlist::load_wordlist("wordlist.txt");
    
    let b = Block::new( 
                        Point::new(5, 5),
                        Point::new(75, 5),
                 );

    let e = Block::new( 
                    Point::new(5, 15),
                    Point::new(10, 1),
                 );

    b.draw_block();
    e.draw_block();

    wordlist::write_wordlist(b, words);
    getch();
    /*
    let done = false;
    while !done {
        let (p, a) = b.write_block(String::from("."));    
    
        if a == false {
            b.clear_block();
            //done = true;
        }
        getch();
        b.update_block(p, "A", Colour::Nothing);
    }*/

    refresh();
    endwin();
}

