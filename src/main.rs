use std::{error::Error, io, time::Duration, sync::mpsc, thread};
use crossterm::{terminal::{self, EnterAlternateScreen}, ExecutableCommand, cursor::{Hide, Show}, event::{self, KeyCode}};
use rusty_audio::Audio;
use space_invaders::{frame, render};

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

    // Render loop in a seperate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame =  match render_rx.recv() {
                Ok(x) => x,  
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame= curr_frame;
        }
        
    });

    
    // Game loop
    'mainGameLoop: loop{
        //Per frame init
        let mut curr_frame = frame::new_frame();
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
        //Draw & render
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }

    //Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())

    
}
