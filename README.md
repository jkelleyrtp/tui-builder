# Tui-builder

An opinionated model on quickly getting up and running with a Tui App - similar to redux toolkit.

Uses these defaults:
- Model, View, Controller paradigm
- Crossterm with multithreaded event system
- ctrl-c to exit

Quickly build a tui app

```rust
fn main() -> Result<()> {
    let mut state = AppState {};

    let handler = move |state: &AppState, event: InputEvent| -> Result<()> {
        // Ingest input events and modify the state
    }

    let renderer = move |state: AppState| {
        // Render the state with tui
        let chunks = tui::layout::Layout::default();
        let block = tui::widgets::Block::default()
            .title("Block 3")
            .borders(tui::widgets::Borders::ALL);
    };

    // Override the state implementation (normally defaults to state default)
    TuiBuilder::<AppState>::with_state(state)
        // Set the default refresh rate on the terminal
        .tick_rate(250)
        // Provide a way of handling raw input events
        .event_handler()
        // Provide a key code used to kill the app
        .kill_signal(KeyCodes::(CtrlC))
        // Provide a way of rendering the state
        .renderer(renderer)
        // Launch the app
        .launch()
}
```
