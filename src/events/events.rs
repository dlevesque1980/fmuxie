// events.rs
use crossterm::event::KeyEvent;

#[derive(Debug, Clone)]
pub enum AppEvent {
    Input(KeyEvent),
    Tick,
    Custom(String),
}