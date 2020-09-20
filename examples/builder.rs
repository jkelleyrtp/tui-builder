use tui_template::{builder::TuiBuilder, tuiapp::TuiApp};

#[derive(Default)]
struct AppState {}

impl TuiApp for AppState {
    fn event_handler(&self, action: crossterm::event::Event) -> Result<(), ()> {
        match action {
            _ => {}
        };
        Ok(())
    }

    fn render(&self) {
        let chunks = tui::layout::Layout::default();
        let block = tui::widgets::Block::default()
            .title("Block 3")
            .borders(tui::widgets::Borders::ALL);
    }
}

fn main() {
    let state = AppState {};

    // Override the state implementation (normally defaults to state default)
    TuiBuilder::<AppState>::with_state(state)
        // Set some options on the TuiApp
        .tick_rate(250)
        // Launch the app
        .launch();
}
