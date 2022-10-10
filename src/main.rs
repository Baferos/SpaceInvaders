use crossterm::{
    cursor::{Hide, Show},
    event::{self, KeyCode},
    terminal::{self, EnterAlternateScreen},
    ExecutableCommand,
};
use rusty_audio::Audio;
use space_invaders::{
    frame::{self, Drawable},
    invaders::Invaders,
    player::Player,
    render,
};
use std::{
    error::Error,
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "sounds/explode.ogg");
    audio.add("lose", "sounds/lose.wav");
    audio.add("move", "sounds/move.wav");
    audio.add("pew", "sounds/pew.ogg");
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
        #[allow(clippy::while_let_loop)]
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // Game loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();
    'mainGameLoop: loop {
        //Per frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = frame::new_frame();
        //check for input
        while event::poll(Duration::default())? {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'mainGameLoop;
                    }
                    _ => {}
                }
            }
        }

        //Updates
        player.update(delta);
        if invaders.update(delta) {
            audio.play("move");
        }
        if player.detect_hits(&mut invaders) {
            audio.play("explode");
        }

        //Draw & render
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut curr_frame)
        }
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));

        //win/lose
        if invaders.all_killed() {
            audio.play("win");
            break 'mainGameLoop;
        }
        if invaders.reached_bottom() {
            audio.play("lose");
            break 'mainGameLoop;
        }
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
