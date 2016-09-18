extern crate ncurses;

use std::str;
use ncurses::*;
use view::*;

mod view;

fn main() {
    
    view::init_view();

    let mut b = Block::new( 
                        Point::new(5, 5),
                        Point::new(75, 5),
                 );

    let e = Block::new( 
                    Point::new(5, 15),
                    Point::new(10, 1),
                 );

    b.draw_block();
    e.draw_block();

    let done = false;
    while !done {
        let (p, a) = b.write_block("-");    
    
        if a == false {
            b.clear_block();
            //done = true;
        }
        getch();
        b.update_block(p, "A", Colour::Correct);
    }

    refresh();
    endwin();
}

