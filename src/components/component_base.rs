// widget_base.rs
use ratatui::{Frame, layout::Rect};
use crate::{state::AppState, AppEvent};

pub trait FocusableWidget {
    fn render(&mut self, f: &mut Frame, area: Rect, focused: bool, app_state: &AppState);
    fn handle_event(&mut self, event: &AppEvent, app_state: &mut AppState);
    fn has_focus(&self) -> bool;
    fn set_focus(&mut self, focused: bool);
}