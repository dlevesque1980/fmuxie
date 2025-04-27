
use ratatui::{
    layout::Rect,

    style::Stylize, 
    text::Line, 
    widgets::{Block, Borders, HighlightSpacing, List, ListItem, ListState}, Frame
};
use std::{fs, os::unix::ffi::OsStrExt, path::{Path, PathBuf}};
use crate::{state::AppState, theme::theme::Theme};
use crossterm::event::KeyCode;

use crate::events::events::AppEvent;

use super::component_base::FocusableWidget;


#[derive(Debug)]
pub struct BrowseItem {
    pub filename: String,
    pub is_dir: bool,
    pub is_hidden: bool,
}
pub struct FileBrowser {
    current_dir: PathBuf,
    entries: Vec<BrowseItem>,
    state: ListState,
    theme: Theme,
    focused: bool,
}

trait Extension {
    fn is_hidden(self) -> bool;
}

impl Extension for &PathBuf {
    #[cfg(unix)]
    fn is_hidden(self) -> bool {
        self.file_name()
            .unwrap()
            .as_bytes()[0] == b'.'
    }
}

impl FocusableWidget for FileBrowser {
    fn render(&mut self, f: &mut Frame, area: Rect, focused: bool, app_state: &AppState) {
        let _ = app_state;
        let _ = focused;

        let block = Block::new()
        .title(Line::raw("File list").centered())
        .borders(Borders::ALL)
        .bg(self.theme.background);        


        let items: Vec<ListItem> = self
        .entries
        .iter()
        .enumerate()
        .map(|(_i, item)| {
            let line = if item.is_dir {
                Line::styled(item.filename.clone(), self.theme.directory)
            } else {
                Line::styled(item.filename.clone(), self.theme.text)
            };
            ListItem::new(line)
        })
        .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(self.theme.highlight)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        f.render_stateful_widget(list, area, &mut self.state);
    }

    fn handle_event(&mut self, event: &AppEvent, app_state: &mut AppState) {
        if !self.has_focus() { return; }

        if let AppEvent::Input(key) = event {
            match key.code {
                KeyCode::Up => self.move_selection(-1),
                KeyCode::Down => self.move_selection(1),
                KeyCode::Enter => self.enter_dir(),
                KeyCode::Left => self.go_back(),
                _ => {}
            }
            app_state.selected_file = self.state.selected().map(|i| {
                format!(
                    "{}/{}",
                    self.current_dir.to_str().unwrap_or_default(),
                    self.entries[i].filename.clone()
                )
            });
        }
    }

    fn has_focus(&self) -> bool {
        self.focused
    }

    fn set_focus(&mut self, _focused: bool) {
        self.focused = _focused;
    }
}

impl FileBrowser {
    pub fn new(path: PathBuf) -> Self {
        let entries = Self::read_dir(&path);
        let mut state = ListState::default();
        if !entries.is_empty() {
            state.select(Some(0));
        }
        Self {
            current_dir: path,
            theme: Theme::default(),
            entries,
            state,
            focused: true,
        }
    }

    fn read_dir(path: &Path) -> Vec<BrowseItem> {
        fs::read_dir(path)
            .map(|rd| {
                let mut items: Vec<BrowseItem> = rd
                    .filter_map(|e| e.ok())
                    .map(|e| {
                        let is_dir = e.path().is_dir();
                        let is_hidden = e.path().is_hidden();
                        let filename = e.file_name().to_string_lossy().into_owned();
                        
                        BrowseItem {
                            filename,
                            is_dir,
                            is_hidden,
                        }
                    })
                    .filter(|item| !item.is_hidden)
                    .collect();
                items.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then(a.filename.cmp(&b.filename)));
                if path.parent().unwrap().exists() { 
                    items.insert(0, BrowseItem { filename: "..".to_string(), is_dir: true, is_hidden: false });
                }
                items
            })
            .unwrap_or_default()
    }

    fn move_selection(&mut self, offset: isize) {
        if let Some(selected) = self.state.selected() {
            let len = self.entries.len() as isize;
            let new = (selected as isize + offset).rem_euclid(len);
            self.state.select(Some(new as usize));
        }
    }

    fn enter_dir(&mut self) {
        if let Some(i) = self.state.selected() {
            let entry = &self.entries[i];
            let mut new_path = self.current_dir.clone();
            if entry.is_dir {
                new_path.push(entry.filename.clone() + "/");
                if new_path.is_dir() {
                    new_path = new_path.canonicalize().unwrap_or(new_path);
                    self.current_dir = new_path;
                    self.entries = Self::read_dir(&self.current_dir);
                    self.state = ListState::default();
                    if !self.entries.is_empty() {
                        // Select the first entry in the new directory
                        self.state.select(Some(1));
                    }
                }
            }
        }
    }

    fn go_back(&mut self) { 
        if self.current_dir.pop() {
            self.entries = Self::read_dir(&self.current_dir);
            self.state = ListState::default();
            if !self.entries.is_empty() {
                // Select the first entry in the new directory
                self.state.select(Some(1));
            }
        }
    }
}
