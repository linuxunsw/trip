use std::io;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::*,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame, 
};

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(70),
                Constraint::Percentage(30),
            ])
            .split(frame.area());

        frame.render_widget(self.build_main(), layout[0]);
        frame.render_widget(self.build_sidebar(), layout[1]);
    }

    fn build_sidebar(&self) -> impl Widget {
        let text = Text::raw("this is a sidebar");
        
        let block = Block::bordered()
            .border_set(border::ROUNDED);

        Paragraph::new(text)
            .block(block)
            .centered()
    }

    fn build_main(&self) -> impl Widget {
        let text = Text::raw("this is the main window");
        
        let block = Block::bordered()
            .border_set(border::ROUNDED);

        Paragraph::new(text)
            .block(block)
            .centered()
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            },
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
