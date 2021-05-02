use anyhow::{Context, Result};
use crossterm::{
    event,
    event::{DisableMouseCapture, EnableMouseCapture, Event as TermEvent, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};

enum InputEvent {
    UserInput(KeyEvent),
    Close,
    Tick,
}

pub trait TuiApp {
    // Apply an App Action to the app state
    fn event_handler(&self, action: TermEvent) -> Result<()>;

    fn render<B: Backend>(&mut self, frame: &mut Frame<B>);

    fn handle_key(&mut self, key: KeyEvent);

    fn tick(&mut self);

    fn should_quit(&self) -> bool;

    fn launch(&mut self) -> Result<()> {
        enable_raw_mode()?;

        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend).unwrap();

        // Setup input handling
        let (tx, rx) = mpsc::channel();

        let tick_rate = Duration::from_millis(250);
        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                // poll for tick rate duration, if no events, sent tick event.
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if event::poll(timeout).unwrap() {
                    if let TermEvent::Key(key) = event::read().unwrap() {
                        tx.send(InputEvent::UserInput(key)).unwrap();
                    }
                }
                if last_tick.elapsed() >= tick_rate {
                    tx.send(InputEvent::Tick).unwrap();
                    last_tick = Instant::now();
                }
            }
        });

        terminal.clear()?;

        loop {
            terminal.draw(|frame| self.render(frame))?;

            // terminal.draw(|f| ui::draw(f, &mut app))?;
            match rx.recv()? {
                InputEvent::UserInput(event) => match event.code {
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        execute!(
                            terminal.backend_mut(),
                            LeaveAlternateScreen,
                            DisableMouseCapture
                        )?;
                        terminal.show_cursor()?;
                        break;
                    }
                    _ => self.handle_key(event),
                },
                InputEvent::Tick => {
                    self.tick();
                }
                InputEvent::Close => {
                    break;
                }
            }
            if self.should_quit() {
                break;
            }
        }

        Ok(())
    }
}
