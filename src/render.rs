use std::io::{Stdout, Write};
use crossterm::{QueueableCommand, terminal::{ClearType, Clear}, style::{SetBackgroundColor, Color}};

use crate::frame::Frame;

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
   if force {
    stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
    stdout.queue(Clear(ClearType::All)).unwrap();
    stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
   }
   for (x, col) in curr_frame.iter().enumerate(){
    for (y, s) in col.iter().enumerate(){
        if force || last_frame[x][y] != *s {
            stdout.queue(crossterm::cursor::MoveTo(x as u16, y as u16)).unwrap();
            print!("{}", *s);
        }
    }
   }
   stdout.flush().unwrap();

}