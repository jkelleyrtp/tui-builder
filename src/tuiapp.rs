use crossterm::event::{Event as TermEvent, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::{io, sync::mpsc, thread, time::Duration};
use tui::{backend::CrosstermBackend, Terminal};

enum InputEvent {
    UserInput(TermEvent),
    Close,
    Tick,
}

pub trait TuiApp {
    // Apply an App Action to the app state
    fn event_handler(&self, action: TermEvent) -> Result<(), ()>;

    fn render(&self);

    fn launch(&mut self) -> Result<(), ()> {
        // Always let us ctrl-c the app
        let (tx, rx) = mpsc::channel();
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
                thread::sleep(Duration::from_millis(250));
            })
        };

        let backend = CrosstermBackend::new(io::stdout());
        // TODO
        let mut terminal = Terminal::new(backend).unwrap();
        enable_raw_mode().unwrap();

        Ok(())
    }
}
