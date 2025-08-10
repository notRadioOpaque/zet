use ratatui::{
    Frame,
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
};

pub fn render(frame: &mut Frame) {
    let title = Line::from(" Zet ").bold().blue().centered();
    let text = "Your Personal Zettelkasten-style Note App!\n\n\
            Press `Esc`, `Ctrl-C` or `q` to stop running.";
    frame.render_widget(
        Paragraph::new(text)
            .bold()
            .block(Block::bordered().title(title))
            .centered(),
        frame.area(),
    )
}
