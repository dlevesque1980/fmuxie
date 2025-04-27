use std::process::Command;

use crossterm::event::KeyCode;
use ratatui::{layout::Rect, text::Line, widgets::{Block, Borders, ListState, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState}, Frame};
use serde_json::Value;

use crate::{events::events::AppEvent, state::AppState, theme::theme::Theme};

use super::component_base::FocusableWidget;

pub struct VideoInfo {
    vertical_scroll_state: ScrollbarState,
    vertical_scroll: usize,
    state: ListState,
    theme: Theme,
    focused: bool,
}

impl FocusableWidget for VideoInfo {

    fn render(&mut self, f: &mut Frame, area: Rect, focused: bool, app_state: &AppState) {
        let _ = focused;

        let filename_owned = app_state.selected_file.clone().unwrap_or_else(|| "No file selected".to_string());
        let filename = filename_owned.as_str(); 
        let is_video: bool = is_video_with_ffprobe(filename);
        if  is_video == false {
        
            let paragraph = Paragraph::new(format!("value: {}",is_video.to_string()))
            .block(Block::new().borders(Borders::ALL).style(ratatui::style::Style::default().bg(self.theme.background)));
            f.render_widget(paragraph,area);
            return;
        }


        let output = Command::new("ffprobe")
                                    .args([ "-print_format","json",
                                            "-show_format",
                                            "-show_streams",
                                            &filename,
                                            ])
                                            .output()
                                            .expect("Failed to execute ffprobe");                                            
        if !output.status.success() {
            return;
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let result: Value = serde_json::from_str(&stdout).unwrap_or_else(|_| {
            eprintln!("Failed to parse ffprobe output");
            Value::Null
        });

        let binding = vec![];
        let streams = result["streams"].as_array().unwrap_or(&binding);
        let mut label = vec![];

        for stream in streams {
            if let Some(index) = stream.get("index") {
                label.push(Line::from(format!("Stream Index: {}\n", index.as_u64().unwrap_or(0))));
            }
            if let Some(codec_type) = stream.get("codec_type") {
                label.push(Line::from(format!("Codec Type: {}\n", codec_type.as_str().unwrap_or("Unknown"))));
            }               

            if let Some(codec_name) = stream.get("codec_name") {
                label.push(Line::from(format!("Codec Name: {}\n", codec_name.as_str().unwrap_or("Unknown"))));
            }
            if let Some(codec_name) = stream.get("codec_long_name") {
                label.push(Line::from(format!("Long Codec Name: {}\n", codec_name.as_str().unwrap_or("Unknown"))));
            }
         
            if let Some(width) = stream.get("width") {
                label.push(Line::from(format!("Width: {}\n", width.as_u64().unwrap_or(0))));
            }
            if let Some(height) = stream.get("height") {
                label.push(Line::from(format!("Height: {}\n", height.as_u64().unwrap_or(0))));
            }

            if let Some(channel_layout) = stream.get("channel_layout") {
                label.push(Line::from(format!("Channel Layout: {}\n", channel_layout.as_str().unwrap_or("Unknown"))));
            }

            let tags = stream.get("tags").unwrap_or(&Value::Null);
            if let Some(language) = tags.get("language") {
                if !language.as_str().unwrap_or("").is_empty() {
                    label.push(Line::from(format!("Language: {}\n", language.as_str().unwrap_or("Unknown"))));
                }
            }
            if let Some(title) = tags.get("title") {
                if !title.as_str().unwrap_or("").is_empty() {
                    label.push(Line::from(format!("Title: {}\n", title.as_str().unwrap_or("Unknown"))));
                }
            };

            label.push(Line::from(""));
        }

        self.vertical_scroll_state = self.vertical_scroll_state.content_length(label.len());

        let paragraph = Paragraph::new(label)
        .block(Block::new().borders(Borders::ALL)
        .style(ratatui::style::Style::default().bg(self.theme.background)))
        .scroll((self.vertical_scroll as u16, 0));

        f.render_widget(paragraph,area);
        f.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            area,
            &mut self.vertical_scroll_state,
        );

    }
    
    fn handle_event(&mut self, event: &AppEvent, app_state: &mut AppState) {
        let _ = app_state;
        if !self.has_focus() { return; }

        if let AppEvent::Input(key) = event {
            match key.code {
                KeyCode::Down => {
                    self.vertical_scroll = self.vertical_scroll.saturating_add(3);
                    self.vertical_scroll_state =
                        self.vertical_scroll_state.position(self.vertical_scroll);
                }
                KeyCode::Up => {
                    self.vertical_scroll = self.vertical_scroll.saturating_sub(3);
                    self.vertical_scroll_state =
                        self.vertical_scroll_state.position(self.vertical_scroll);
                }

                _ => {}
            }
        }
    }

    fn has_focus(&self) -> bool {
        self.focused
    }

    fn set_focus(&mut self, _focused: bool) {
        self.focused = _focused;
    }

}

impl VideoInfo {
    pub fn new() -> Self {
        VideoInfo {
            state: ListState::default(),
            theme: Theme::default(),
            focused: false,
            vertical_scroll_state: ScrollbarState::default(),
            vertical_scroll: 0,
        }
    } 
}

fn is_video_with_ffprobe(path: &str) -> bool {

    let output = Command::new("ffprobe")
        .args([
            "-v", "error",
            "-select_streams", "v", // video streams only
            "-show_entries", "stream=codec_type",
            "-of", "default=noprint_wrappers=1:nokey=1",
            path, // <-- no "-i"!
        ])
        .output()
        .expect("Failed to execute ffprobe");

    if !output.status.success() {
        return false;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.contains("video")
}