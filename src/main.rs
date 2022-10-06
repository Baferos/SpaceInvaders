use std::{error::Error, io, time::Duration};
use crossterm::{terminal::{self, EnterAlternateScreen}, ExecutableCommand, cursor::{Hide, Show}, event::{self, KeyCode}};
use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "sounds/explode.wav");
    audio.add("lose", "sounds/lose.wav");
    audio.add("move", "sounds/move.wav");
    audio.add("pew", "sounds/pew.wav");
    audio.add("startup", "sounds/startup.wav");
    audio.add("win", "sounds/win.wav");
    audio.play("startup");
    
    // Terminal

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;
    


    // Game loop
    'mainGameLoop: loop{
        //check for input
        while event::poll(Duration::default())? {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'mainGameLoop},
                    _ => {}
                }
            }
        }
    }


    //Cleanup
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())

    
}
