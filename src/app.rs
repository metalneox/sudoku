use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::{Duration, Instant};

use crate::sudoku::Sudoku;
use crate::ui;

pub use crate::sudoku::Difficulty;

pub struct App {
    pub sudoku: Sudoku,
    pub cursor: (usize, usize), // (row, col)
    pub should_quit: bool,
    pub message: Option<String>,
    pub difficulty: Difficulty,
    pub timer_start: Option<Instant>,
    pub timer_elapsed: Duration,
    pub timer_stopped: bool,
}

impl Default for App {
    fn default() -> Self {
        let difficulty = Difficulty::Easy;
        App {
            sudoku: Sudoku::generate_puzzle(Some(difficulty)),
            cursor: (0, 0),
            should_quit: false,
            message: None,
            difficulty,
            timer_start: Some(Instant::now()),
            timer_elapsed: Duration::ZERO,
            timer_stopped: false,
        }
    }
}

impl App {
    pub fn new() -> Self {
        let app: App = App::default();
        app
    }

    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
        loop {
            terminal.draw(|f| ui::draw(f, self))?;

            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => self.should_quit = true,
                        KeyCode::Up if self.cursor.0 > 0 => self.cursor.0 -= 1,
                        KeyCode::Down if self.cursor.0 < 8 => self.cursor.0 += 1,
                        KeyCode::Left if self.cursor.1 > 0 => self.cursor.1 -= 1,
                        KeyCode::Right if self.cursor.1 < 8 => self.cursor.1 += 1,
                        KeyCode::Char('n') => {
                            self.sudoku = Sudoku::generate_puzzle(Some(self.difficulty));
                            self.cursor = (0, 0);
                            self.timer_start = Some(Instant::now());
                            self.timer_elapsed = Duration::ZERO;
                            self.timer_stopped = false;
                            //self.message = Some("Nuova partita generata!".to_string());
                        }
                        KeyCode::Char(c) if c.is_ascii_digit() && c != '0' => {
                            let num = c.to_digit(10).unwrap() as u8;
                            self.sudoku.set(self.cursor.0, self.cursor.1, num);
                        }
                        KeyCode::Backspace | KeyCode::Char('z') => {
                            self.sudoku.clear(self.cursor.0, self.cursor.1);
                        }
                        KeyCode::Tab => {
                            self.difficulty = self.difficulty.next();
                        }
                        KeyCode::Enter if self.message.is_some() => {
                            self.sudoku = Sudoku::generate_puzzle(Some(self.difficulty));
                            self.cursor = (0, 0);
                            self.message = None;
                            self.timer_start = Some(Instant::now());
                            self.timer_elapsed = Duration::ZERO;
                            self.timer_stopped = false;
                        }
                        KeyCode::Esc => {
                            self.should_quit = true;
                        }
                        _ => {}
                    }
                }
            }

            if self.message.is_none() && self.sudoku.is_complete() {
                self.timer_elapsed = self.timer_start.map_or(Duration::ZERO, |t| t.elapsed());
                self.timer_stopped = true;
                self.message = Some(
                    "Congratulazioni! Puzzle completato. Premi Invio per resettare.".to_string(),
                );
            }

            if self.should_quit {
                break;
            }
        }
        Ok(())
    }
}
