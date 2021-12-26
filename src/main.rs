use crossterm::cursor::{Hide, Show};
use crossterm::event;
use crossterm::event::KeyCode;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{terminal, ExecutableCommand};
use invaders::frame::{new_frame, Drawable};
use invaders::invaders::Invaders;
use invaders::player::Player;
use invaders::render;
use rusty_audio::Audio;
use std::error::Error;
use std::{io, thread};
use std::sync::mpsc;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();

    setup_audio(&mut audio);

    audio.play("startup");

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop (separate thread)
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
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
    'gameloop: loop {
        // Pre-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        // Input
        while event::poll(Duration::default())? {
            if let event::Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    },
                    KeyCode::Left | KeyCode::Char('h') => player.move_left(),
                    KeyCode::Right | KeyCode::Char('l') => player.move_right(),
                    KeyCode::Char(' ') => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    },
                    _ => {}
                }

            }
        }

        // Updates
        player.update(delta);
        if invaders.update(delta) {
            audio.play("move");
        }
        if player.detect_hits(&mut invaders) {
            audio.play("explode");
        }

        // Draw & Render
        player.draw(&mut curr_frame);
        invaders.draw(&mut &mut curr_frame);
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));


        // Win or lose?
        if invaders.all_killed() {
            audio.play("win");
            break 'gameloop;
        }
        if invaders.reached_bottom() {
            audio.play("lose");
            break 'gameloop;
        }
    }

   

    // Cleanup
    drop(render_tx);
    audio.wait();
    render_handle.join().unwrap();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

fn setup_audio(audio: &mut Audio) {
    audio.add("explode", "sounds/explode.wav");
    audio.add("lose", "sounds/lose.wav");
    audio.add("win", "sounds/win.wav");
    audio.add("pew", "sounds/pew.wav");
    audio.add("startup", "sounds/startup.wav");
    audio.add("move", "sounds/move.wav");
}
