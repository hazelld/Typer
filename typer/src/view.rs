extern crate ncurses;

use std::str;
use ncurses::*;


// Set the different colours
pub enum Colour {
    Correct,
    Incorrect,
    Current,
    Nothing,
}

// Pairings of colours 
static COLOUR_PAIR_CORRECT: i16 = 1;
static COLOUR_PAIR_INCORRECT: i16 = 2;

// Colour Values in ncurses
static COLOUR_CORRECT: i16 = 2; //Green
static COLOUR_INCORRECT: i16 = 1; //Red

#[derive(Clone,Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new (x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}


pub struct Block {
    start: Point,
    end: Point, 
    size: Point,
    cursor: Point,
}

impl Block {

    // Initialize a Block. Only need a start and size points,
    // the end Point is created here.
    pub fn new (start: Point, size: Point) -> Block {
        
        // Declare end & cursor first to avoid moving the value out of start and size
        // as Point does not implement the Copy trait
        Block { end: Point { x: size.x + start.x + 1, y: size.y + start.y + 1 },
                cursor: Point { x: start.x + 1, y: start.y + 1 },
                start: start,
                size: size,
        }
    }

    // Draw the box based on it's given size and location
    pub fn draw_block(&self) {
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
    pub fn clear_block (&mut self) {
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
    pub fn write_block(&mut self, content: String) -> (Point, bool) {
        let size = content.len() as i32;
        let oldx = self.cursor.x;
        let oldy = self.cursor.y;

        // Determine if there is room to print. If the block is full then exit function
        let (buff_point, result) = get_new_point(self.cursor, self.end, self.start, size);
        if result == false { return (buff_point, result); }
    
        // Move to start and print
        self.cursor = buff_point;
        mv(self.cursor.y, self.cursor.x);
        printw(&content);
        refresh();
        
        // Update the cursors position after the write
        self.cursor.x += size;
        return (Point { x: oldx, y: oldy }, true)
    }

    // Update a part of the block without moving the cursor
    pub fn update_block (&mut self, location: Point, content: &str, state: Colour) -> bool {
        let size = content.len() as i32;

        let (buff_point, result) = get_new_point(location, self.end, self.start, size);
        if result == false { return false; }

        // We cant update any other line so verify the y-value didnt change
        if buff_point.y != location.y { return false; }
        mv(buff_point.y, buff_point.x);
        
        
        let attr = get_attr(state);
        attron(attr);
        printw(content);
        attroff(attr);
        refresh();
        true
    }
}

// Check if the current location is large enough to fit the given size. If there is not 
// enough room on the current line, move down 2 lines and check if there is room there.
//
// Returns: First open location that fits the string, True if there was an open spot
//          False if there is no open space in the block
fn get_new_point ( start: Point, end: Point, block_origin: Point, size: i32) -> (Point, bool) {
    
    if check_move(start.x, size, end.x) == false {
        if check_move(start.y, 2, end.y-1) == false {
            return ( Point { x: 0, y: 0 }, false );
        } else {
            return (Point { x: block_origin.x + 1, y: start.y + 2 }, true);
        }
    }

    // If we do nothing return nothing 
    ( Point { x: start.x, y: start.y }, true )
}

fn check_move ( cur: i32, size: i32, end: i32) -> bool { 
    if cur + size > end {  return false; }
    true
}


fn get_attr (state: Colour) -> attr_t {
    
    match state {
        Colour::Incorrect => { COLOR_PAIR(COLOUR_PAIR_INCORRECT) },
        Colour::Correct => { COLOR_PAIR(COLOUR_PAIR_CORRECT) },
        Colour::Current => { A_STANDOUT() },
        Colour::Nothing => { 0 },
    }
}

pub fn init_view() {
    initscr();
    cbreak();
    noecho();
    
    //use_default_colors();
    start_color();

    // -1 as bg keeps the default
    init_pair(COLOUR_PAIR_INCORRECT, COLOUR_INCORRECT, 0); 
    init_pair(COLOUR_PAIR_CORRECT, COLOUR_CORRECT, 0);
    
}
