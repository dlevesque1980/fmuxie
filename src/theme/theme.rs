use ratatui::style::{palette::tailwind::{BLUE, CYAN, GRAY, SLATE}, Color, Modifier, Style};

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub text: Color,
    pub background: Color,
    pub highlight: Style,
    pub directory: Color,
}

const DEFAULT: Theme = Theme {
    text: GRAY.c500,
    background: BLUE.c900,
    highlight: Style::new().bg(CYAN.c500).add_modifier(Modifier::BOLD),
    directory: SLATE.c200,
};

impl Default for Theme {
    fn default() -> Self { DEFAULT }
}