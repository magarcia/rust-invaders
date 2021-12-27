use std::{cmp::max, env, time::Duration};

use crossterm::terminal;
use rusty_time::timer::Timer;

use crate::frame::{Drawable, Frame};

pub struct Invader {
    x: usize,
    y: usize,
}

pub struct Invaders {
    army: Vec<Invader>,
    move_timer: Timer,
    direction: i32,
}

impl Default for Invaders {
    fn default() -> Self {
        Self::new()
    }
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();
        let (cols, rows) = terminal::size().unwrap();

        for x in 0..cols as usize {
            for y in 0..rows as usize {
                if (x > 1)
                    && (x < cols as usize - 2)
                    && (y > 0)
                    && (y < 9)
                    && (x % 4 == 0)
                    && (y % 2 == 0)
                {
                    army.push(Invader { x, y });
                }
            }
        }

        Self {
            army,
            move_timer: Timer::from_millis(2000),
            direction: 1,
        }
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);

        let (cols, _) = terminal::size().unwrap();

        if self.move_timer.ready {
            self.move_timer.reset();

            let mut downwards = false;
            if self.direction == -1 {
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if min_x == 0 {
                    self.direction = 1;
                    downwards = true;
                }
            } else {
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                if max_x == cols as usize - 1 {
                    self.direction = -1;
                    downwards = true;
                }
            }
            if downwards {
                let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);
                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else {
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as i32) + self.direction) as usize;
                }
            }

            return true;
        }

        false
    }

    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }

    pub fn reached_bottom(&self) -> bool {
        let (_, rows) = terminal::size().unwrap();
        self.army.iter().map(|invader| invader.y).max().unwrap_or(0) >= rows as usize - 1
    }

    pub fn kill_invader_at(&mut self, x: usize, y: usize) -> bool {
        if let Some(idx) = self
            .army
            .iter()
            .position(|invader| (invader.x == x) && (invader.y == y))
        {
            self.army.remove(idx);
            true
        } else {
            false
        }
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        let args = env::args();
        let fruity = args.collect::<String>().contains(&String::from("--fruity"));
        for (i, invader) in self.army.iter().enumerate() {
            if !fruity {
                frame[invader.x][invader.y] = if (self.move_timer.time_left.as_secs_f32()
                    / self.move_timer.duration.as_secs_f32())
                    > 0.5
                {
                    "x"
                } else {
                    "+"
                };
            } else {
                let vegetables = [
                    "🍏", "🍎", "🍐", "🍊", "🍋", "🍌", "🍉", "🍇", "🍓", "🍈", "🍒", "🍑", "🍍",
                    "🥭", "🥥", "🥝", "🍅", "🍆", "🥑", "🥦", "🥒", "🥬", "🌶", "🌽", "🥕", "🥔",
                    "🍠",
                ];
                let len = vegetables.len();
                frame[invader.x][invader.y] = vegetables[i % len];
            }
            // frame[invader.x][invader.y] = "👾";
        }
    }
}
