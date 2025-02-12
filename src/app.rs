use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Row, Table},
    Terminal,
};
use std::{
    cmp::{max, min},
    io::{self, stdout},
};
pub struct App {
    commands: Vec<String>,
    selected_idx: usize,
    num_commands: usize,
}

impl App {
    pub fn new(commands: Vec<String>) -> Self {
        let num_commands = commands.len();
        App {
            commands,
            selected_idx: num_commands - 1,
            num_commands,
        }
    }

    pub fn run(&mut self) -> io::Result<Option<String>> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = self.run_app(&mut terminal);

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        result
    }

    fn run_app(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    ) -> io::Result<Option<String>> {
        loop {
            terminal.draw(|frame| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(3), Constraint::Length(1)])
                    .split(frame.area());

                let height = chunks[0].height as usize;

                let start_idx = self.selected_idx.saturating_sub(height - 1);
                let end_idx = start_idx + height;

                let rows: Vec<Row> = self.commands[start_idx..end_idx]
                    .iter()
                    .enumerate()
                    .map(|(i, cmd)| {
                        let index = self.num_commands.saturating_sub(start_idx + i).to_string();
                        let style = if start_idx + i == self.selected_idx {
                            Style::default().bg(Color::Cyan)
                        } else {
                            Style::default()
                        };
                        Row::new(vec![index, cmd.clone()]).style(style)
                    })
                    .collect();

                let table = Table::new(rows, &[Constraint::Length(5), Constraint::Min(20)])
                    .column_spacing(1);
                frame.render_widget(table, chunks[0]);

                let instructions = Paragraph::new(vec![Line::from(vec![
                    Span::raw("<Enter>: select, "),
                    Span::raw("q: quit, "),
                    Span::raw("j/k: navigate, "),
                    Span::raw("{/}: skip page, "),
                    Span::raw("G/g: first or last command"),
                ])])
                .style(Style::default().fg(Color::DarkGray));
                frame.render_widget(instructions, chunks[1]);
            })?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('j') => {
                        if self.selected_idx + 1 < self.num_commands {
                            self.selected_idx += 1;
                        }
                    }
                    KeyCode::Char('k') => {
                        if self.selected_idx > 0 {
                            self.selected_idx -= 1;
                        }
                    }
                    KeyCode::Char('}') => {
                        self.selected_idx = min(
                            self.selected_idx + terminal.size()?.height as usize - 2,
                            self.num_commands - 1,
                        );
                    }
                    KeyCode::Char('{') => {
                        self.selected_idx = max(
                            self.selected_idx as isize - terminal.size()?.height as isize + 2,
                            0,
                        ) as usize;
                    }
                    KeyCode::Char('g') => {
                        self.selected_idx = 0;
                    }
                    KeyCode::Char('G') => {
                        self.selected_idx = self.num_commands - 1;
                    }
                    KeyCode::Enter => {
                        let selected = self.commands.get(self.selected_idx);
                        return Ok(selected.cloned());
                    }
                    KeyCode::Char('q') => return Ok(None),
                    _ => {}
                }
            }
        }
    }
}
