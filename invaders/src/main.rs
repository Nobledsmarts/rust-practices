use std::{error::Error, io, sync::mpsc, thread, time::Duration};
use crossterm::{cursor::{Hide, Show}, event::{self, Event, KeyCode}, terminal::{self, EnterAlternateScreen}, ExecutableCommand};
use invaders::{frame::{self, new_frame, Drawable}, player::Player, render::{self, render}};
use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
// fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
// fn main() -> Result<(), Box<(dyn Error + 'static)>> {
    let mut audio = Audio::new();
    audio.add("explode", "sounds/explode.wav");
    audio.add("lose", "sounds/lose.wav");
    audio.add("move", "sounds/move.wav");
    audio.add("pew", "sounds/pew.wav");
    audio.add("startup", "sounds/startup.wav");
    audio.add("win", "sounds/win.wav");

    audio.play("startup");

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break
            };

            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    let mut player = Player::new();

    'gameloop : loop {

        // per frame init
        let mut curr_frame = new_frame();


        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),

                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        audio.wait();
                        break 'gameloop; 
                    },
                    _ => {}
                }
            }
        }

        // draw and render
        player.draw(&mut curr_frame);
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }
    
    drop(render_tx);
    render_handle.join().unwrap();

    audio.wait();

    stdout.execute(Show)?;
    stdout.execute(EnterAlternateScreen)?;
    terminal::disable_raw_mode()?;

    return Ok(());
    // println!("Hello, world!");
}
