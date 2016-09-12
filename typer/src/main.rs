extern crate ncurses;
use ncurses::*;

fn main() {
    initscr();
    cbreak();
    printw("Hello World");
    refresh();
    getch();
    endwin();
}
