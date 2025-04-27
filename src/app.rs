

use std::env;
use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::palette::material::BLUE;
use ratatui::style::Color;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;
use crate::components::component_base::FocusableWidget;
use crate::components::file_browser::FileBrowser;
use crate::components::video_info::VideoInfo; // Assuming VideoInfo is in this module
use crate::events::events::AppEvent;
use crate::state::AppState;
use crate::focus_manager::FocusManager;

const APP_BACKGROUND: Color = BLUE.c900;
pub struct App {
    file_browser: FileBrowser,
    video_info: VideoInfo,
    main_layout: Layout,
    state: AppState,
    focus: FocusManager,
}

impl App {
    pub fn new() -> Self {

        let main_layout = Layout::default()
        
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]);
    
        let state = AppState {
            selected_file: None,
        };

        let mut file_browser = FileBrowser::new(env::current_dir().unwrap_or_else(|_| {
            eprintln!("Failed to get current directory");
            std::process::exit(1);
        }));
        file_browser.set_focus(true);

        let mut video_info = VideoInfo::new();
        video_info.set_focus(false);

        App { file_browser, video_info, main_layout, state, focus: FocusManager::new(3) }
    }

    pub fn handle_event(&mut self, event: &AppEvent) {

        if let AppEvent::Input(key) = event {
            match key.code {
                KeyCode::Tab => {
                    self.focus.next();
                    return;
                }
                KeyCode::BackTab => {
                    self.focus.previous();
                    return;
                }
                _ => {}
            }
        }

        // Delegate input to the focused widget
        match self.focus.current() {
            0 => {
                self.file_browser.set_focus(true);
                self.file_browser.handle_event(event, &mut self.state)
            },
            1 => {
                self.video_info.set_focus(true);
                self.video_info.handle_event(event, &mut self.state)
            },
            _ => {}
        }
    }

    pub fn render(&mut self, f: &mut Frame<'_>) -> Result<(), Box<dyn std::error::Error>> {
            // Render the file browser
        let chunks = self.main_layout.split(f.area());

        let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(25),
            Constraint::Percentage(75),
        ])
        .split(chunks[0]);

        self.file_browser.render(f,inner_layout[0], true, &self.state);
        f.render_widget(
            Paragraph::new(self.state.selected_file.clone().unwrap_or_else(|| "No file selected".to_string()))
                .block(Block::new().borders(Borders::ALL).style(ratatui::style::Style::default().bg(APP_BACKGROUND))),
                
            chunks[1]);
        self.video_info.render(f, inner_layout[1], false, &self.state);       
        Ok(())
    }
}