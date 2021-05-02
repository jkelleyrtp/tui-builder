use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::{
        Axis, BarChart, Block, BorderType, Borders, Cell, Chart, Dataset, Gauge, LineGauge, List,
        ListItem, Paragraph, Row, Sparkline, Table, Tabs, Wrap,
    },
    Frame,
};
use tui_template::tuiapp::TuiApp;

#[derive(Default)]
struct AppState {}

impl TuiApp for AppState {
    fn event_handler(&self, action: crossterm::event::Event) -> anyhow::Result<()> {
        Ok(())
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent) {}

    fn tick(&mut self) {}

    fn should_quit(&self) -> bool {
        false
    }

    fn render<B: tui::backend::Backend>(&mut self, f: &mut tui::Frame<B>) {
        // Wrapping block for a group
        // Just draw the block and the group on the same area and build the group
        // with at least a margin of 1
        let size = f.size();
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Main block with round corners")
            .border_type(BorderType::Rounded);
        f.render_widget(block, size);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(4)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(f.size());

        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[0]);
        let block = Block::default()
            .title(vec![
                Span::styled("With", Style::default().fg(Color::Yellow)),
                Span::from(" background"),
            ])
            .style(Style::default().bg(Color::Green));
        f.render_widget(block, top_chunks[0]);

        let block = Block::default().title(Span::styled(
            "Styled title",
            Style::default()
                .fg(Color::White)
                .bg(Color::Red)
                .add_modifier(Modifier::BOLD),
        ));
        f.render_widget(block, top_chunks[1]);

        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[1]);
        let block = Block::default().title("With borders").borders(Borders::ALL);
        f.render_widget(block, bottom_chunks[0]);
        let block = Block::default()
            .title("With styled borders and doubled borders")
            .border_style(Style::default().fg(Color::Cyan))
            .borders(Borders::LEFT | Borders::RIGHT)
            .border_type(BorderType::Double);
        f.render_widget(block, bottom_chunks[1]);
    }
}

fn main() {
    let mut state = AppState {};

    state.launch().unwrap();
    // // Override the state implementation (normally defaults to state default)
    // TuiBuilder::<AppState>::with_state(state)
    //     // Set some options on the TuiApp
    //     .tick_rate(250)
    //     // Launch the app
    //     .launch();
}
