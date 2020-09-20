use std::sync::mpsc;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

use crossterm::event::{Event as TermEvent, KeyCode};

pub enum InputEvent {
    UserInput(TermEvent),
    Close,
    Tick,
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct TermEvents {
    rx: mpsc::Receiver<InputEvent>,
    ignore_exit_key: Arc<AtomicBool>,

    #[allow(unused)]
    input_handle: thread::JoinHandle<()>,

    #[allow(unused)]
    tick_handle: thread::JoinHandle<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct AppTermConfig {
    pub tick_rate: Duration,
}

impl Default for AppTermConfig {
    fn default() -> AppTermConfig {
        AppTermConfig {
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl TermEvents {
    pub fn new() -> TermEvents {
        TermEvents::with_config(AppTermConfig::default())
    }

    pub fn with_config(config: AppTermConfig) -> TermEvents {
        let (tx, rx) = mpsc::channel();
        let ignore_exit_key = Arc::new(AtomicBool::new(false));
        let input_handle = {
            let tx = tx.clone();
            thread::spawn(move || loop {
                if let Ok(event) = crossterm::event::read() {
                    // Kill the channel
                    if event == TermEvent::Key(KeyCode::Esc.into()) {
                        tx.send(InputEvent::Close).unwrap();
                        break;
                    }

                    // Send the message
                    tx.send(InputEvent::UserInput(event)).unwrap();
                }
            })
        };
        let tick_handle = {
            thread::spawn(move || loop {
                if tx.send(InputEvent::Tick).is_err() {
                    break;
                }
                thread::sleep(config.tick_rate);
            })
        };
        TermEvents {
            rx,
            ignore_exit_key,
            input_handle,
            tick_handle,
        }
    }

    pub fn next(&self) -> Result<InputEvent, mpsc::RecvError> {
        self.rx.recv()
    }

    pub fn disable_exit_key(&mut self) {
        self.ignore_exit_key.store(true, Ordering::Relaxed);
    }

    pub fn enable_exit_key(&mut self) {
        self.ignore_exit_key.store(false, Ordering::Relaxed);
    }
}
