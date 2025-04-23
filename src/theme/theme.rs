use ratatui::style::{palette::tailwind::{BLUE, CYAN, GRAY, SLATE}, Color, Modifier, Style};

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub text: Color,
    pub background: Color,
    pub highlight: Style,
    pub directory: Color,
    pub file: Color
}

const DEFAULT: Theme = Theme {
    text: SLATE.c200,
    background: BLUE.c900,
    highlight: Style::new().bg(CYAN.c500).add_modifier(Modifier::BOLD),
    directory: SLATE.c200,
    file: GRAY.c500
};

impl Default for Theme {
    fn default() -> Self { DEFAULT }
}