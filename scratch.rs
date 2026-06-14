use ratatui::{backend::CrosstermBackend, Terminal, widgets::{Paragraph, Wrap, Block, Borders}, layout::Rect};
use std::io;
fn main() {
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|f| {
        let text = "This is a very long text without spaces: /home/antonin/app/projects/Hydra-ecosystem/CodeLens/src/main.rs";
        let p = Paragraph::new(text)
            .block(Block::default().title("TEST").borders(Borders::ALL))
            .wrap(Wrap { trim: false });
        f.render_widget(p, Rect::new(0, 0, 30, 10));
    }).unwrap();
}
