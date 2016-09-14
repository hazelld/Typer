extern crate ncurses;
use ncurses::*;

/**/
struct Point {
    x: i32,
    y: i32,
}

/**/
struct Block {
    start: Point,
    end: Point, 
    size: Point,
}

trait Convert {
    fn convert(&self) -> bool;
}

impl Block {

    // Initialize a Block. Only need a start and size points,
    // the end Point is created here.
    fn new (start: Point, size: Point) -> Block {
        let endx = size.x + start.x + 1;
        let endy = size.y + start.y + 1;

        Block { start: start,
                size: size,
                end: Point { x: endx, y: endy },
        }
    }

    // Draw the box based on it's given size and location
    fn draw_block(&self) {
        for n in self.start.x+1..self.end.x {
            mvaddch(self.start.y, n, 45);
            mvaddch(self.end.y, n, 45);
        }
    
        for i in self.start.y+1..self.end.y {
            mvaddch(i, self.start.x, 124);
            mvaddch(i, self.end.x, 124);
        }
    }
    
    // Clear the block's area of content
    fn clear_block (&self) {
        for i in self.start.y+1..self.end.y {
            for j in self.start.x+1..self.end.x {
                mvaddch(i, j, 32);
            }
        }
        refresh();
    }

    fn write_block<T: Convert>(&self, content: T) -> bool {
        mv(self.start.y+1, self.start.x+1);
        printw("Hello");
        true
    }
}


fn main() {
    initscr();
    cbreak();
    let b = Block::new( Point { x: 5,  y: 5 },
                        Point { x: 75, y: 5 },
                 );
    let e = Block::new( Point { x: 5,  y: 15 },
                        Point { x: 10, y: 1 },
                 );
    b.draw_block();
    e.draw_block();
    
    refresh();
    getch();
    endwin();
}

/*  This is how the content types will be set up.
struct Test {
    val: i32,
}

impl Convert for Test {
    fn convert(&self) -> bool{
        printw("\nHERE");
        true
    }
}
*/
