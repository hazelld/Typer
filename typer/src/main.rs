extern crate ncurses;

use std::str;
use ncurses::*;

#[derive(Clone,Copy)]
struct Point {
    x: i32,
    y: i32,
}

struct Block {
    start: Point,
    end: Point, 
    size: Point,
    cursor: Point,
}

impl Block {

    // Initialize a Block. Only need a start and size points,
    // the end Point is created here.
    fn new (start: Point, size: Point) -> Block {
        
        // Declare end & cursor first to avoid moving the value out of start and size
        // as Point does not implement the Copy trait
        Block { end: Point { x: size.x + start.x + 1, y: size.y + start.y + 1 },
                cursor: Point { x: start.x + 1, y: start.y + 1 },
                start: start,
                size: size,
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
    
    // Clear the block's area of content, point cursor to start
    fn clear_block (&mut self) {
        for i in self.start.y+1..self.end.y {
            for j in self.start.x+1..self.end.x {
                mvaddch(i, j, 32);
            }
        }
        self.cursor = Point { x: self.start.x + 1, y: self.start.y + 1 };
        refresh();
    }

    // Write content into the block at the current cursor position.
    // Returns the starting point of what was printed and the result of attempted write
    // TODO: Take colour as arg
    fn write_block(&mut self, content: &str) -> (Point, bool) {
        let size = content.len() as i32;
        let oldx = self.cursor.x;
        let oldy = self.cursor.y;

        // Determine if there is room to print. If the block is full then exit function
        if check_move(oldx, size, self.end.x) == false {   
            if check_move(oldy, 2, self.end.y) == false {
                return(Point { x: 0, y: 0 }, false);
            } else {
                
                // Move the cursor 2 lines down and back to start
                self.cursor.y += 2;
                self.cursor.x = self.start.x + 1;
            }
        }

        // Move to start and print
        mv(self.cursor.y, self.cursor.x);
        printw(content);
        refresh();
        
        // Update the cursors position after the write
        self.cursor.x += size;
        return (Point { x: oldx, y: oldy }, true)
    }

    // Update a part of the block without moving the cursor
    // TODO: take colour as an arg
    fn update_block (&self, location: Point, content: &str) {

    }
    
    
}

fn check_move ( cur: i32, size: i32, end: i32) -> bool { 
    if cur + size >= end {  return false; }
    true
}
 
fn main() {
    initscr();
    cbreak();
    noecho();

    let mut b = Block::new( Point { x: 5,  y: 5 },
                        Point { x: 75, y: 5 },
                 );
    let e = Block::new( Point { x: 5,  y: 15 },
                        Point { x: 10, y: 1 },
                 );
    b.draw_block();
    e.draw_block();
    let mut done = false;

    while !done {
        let (p, a) = b.write_block("Hello ");    
    
        if a == false {
            b.clear_block();
            //done = true;
        }
        getch();
    }

    refresh();
    endwin();
}

