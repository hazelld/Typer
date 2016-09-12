extern crate ncurses;
use ncurses::*;


const WORD_LINES: i32 = 2;
const BLANK_LINES: i32 = WORD_LINES + 1;
const TIME: i32 = 60;

/**/
struct Point {
    x: i32,
    y: i32,
}

/**/
impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

}

/**/
struct Block {
    start: Point,
    size: Point,
}

impl Block {
    
    fn new(start: Point, size: Point) -> Block {
        let (x,y) = (start.x, start.y);
        Block {
            start: start,
            size: Point::new(x + size.x + 1, y + size.y + 1),
        }
    }

}

/**/
struct Stats {
    correct: i32,
    incorrect: i32,
}

/**/
fn main() {
    initscr();
    cbreak();
    setup_scr();
    refresh();
    getch();
    endwin();
}

/**/
fn setup_scr() {
    let block = Block::new( Point::new(4,4), 
                            Point::new(75,WORD_LINES + BLANK_LINES), 
    );

    draw_block(block.start, block.size);
}

fn draw_block(start: Point, size: Point) {
    for n in start.x+1..size.x {
        mvaddch(start.y, n, 45);
        mvaddch(size.y, n, 45);
    }
    
    for i in start.y+1..size.y {
        mvaddch(i, start.x, 124);
        mvaddch(i, size.x, 124);
    }
}
