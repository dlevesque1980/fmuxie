mod app;
mod components;
mod events;
mod theme;
pub mod state;
pub mod focus_manager;

use crate::app::App;
use events::{event_queue::EventQueue, events::AppEvent};
use ratatui::{
    backend::CrosstermBackend, Terminal
};
use crossterm::{execute, terminal::{self, EnterAlternateScreen}, ExecutableCommand};
use std::io;

fn app_start() -> Result<(), io::Error> {
    // Terminal setup
    terminal::enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    Ok(())
}
fn app_end() -> Result<(), io::Error> {
    terminal::disable_raw_mode()?;
    execute!(io::stdout(), terminal::LeaveAlternateScreen)?;
    Ok(())
}

fn main() -> Result<(), io::Error> {
    // Terminal setup
    app_start()?;
    let stdout: io::Stdout = io::stdout();
    let backend: CrosstermBackend<io::Stdout> = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;// Terminal<CrosstermBackend<io::Stdout>>
    let mut app = App::new();
  
    let event_queue = EventQueue::new();

    // Main loop
    loop {
        terminal.draw(|f| {
            if let Err(err) = app.render(f) {
                eprintln!("error: {}", err.to_string());
                std::process::exit(1);
            }
        })?;

        match event_queue.receiver.recv() {
            Ok(event) => {
                match &event {
                    AppEvent::Input(key) => {
                        match key.code {
                            // 🔚 Quitter l'app
                            crossterm::event::KeyCode::Char('q') | crossterm::event::KeyCode::Esc => break,
                            _ => {}
                        }
                    }
                    AppEvent::Tick => {
                        // tu peux faire un refresh, mise à jour de l’état, etc.
                    }
                    _ => {}
                }

                // 📥 Envoyer l'événement au widget
                app.handle_event(&event);
            }
            Err(err) => {
                eprintln!("Erreur dans la file d'événements : {err}");
                break;
            }
        }

        // Render the UI

    }
  
    terminal.draw(|f| {
        if let Err(err) = app.render(f) {
            eprintln!("error: {}", err.to_string());
            std::process::exit(1);
        }
    })?;

    app_end()?;
    Ok(())
}