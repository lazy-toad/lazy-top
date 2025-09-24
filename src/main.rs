mod app;
mod tui;
mod ui;

//--------------------------------------------------------------------------------------------------------

use crossterm::event::{self, Event, KeyCode};

use anyhow::Result;
use app::{App, AppMode, AppTheme};
use ratatui::Terminal;
use tui::{init, restore};
use ui::ui;

//----------------------------------------------------------------------------------------------------------

fn main() -> Result<()> {
    let mut terminal = init()?;
    let mut app = App::new();

    run_app(&mut terminal, &mut app)?;

    restore()?;
    Ok(())
}

//----------------------------------------------------------------------------------------------------------

fn handle_key_event(app: &mut App, key_code: KeyCode) {
    match app.mode {
        AppMode::Normal => match key_code {
            KeyCode::Char('q') => {}
            KeyCode::Char(':') => {
                app.mode = AppMode::Command;
                app.command_buffer.clear();
            }
            KeyCode::Char('/') => {
                app.mode = AppMode::Filtering;
                app.filter_query.clear();
            }
            KeyCode::Char('k') => app.kill_selected_process(),
            KeyCode::Down => app.next(),
            KeyCode::Up => app.previous(),
            KeyCode::Char('t') => app.theme = app.theme.clone().next(),
            KeyCode::Char('c') => app.cycle_sort_coloumn(),
            _ => {}
        },
        AppMode::Command => match key_code {
            KeyCode::Enter => {
                let parts: Vec<&str> = app.command_buffer.split_whitespace().collect();
                if let Some(command) = parts.get(0) {
                    if *command == "theme" {
                        if let Some(theme_name) = parts.get(1) {
                            if let Some(new_theme) = AppTheme::from_str(theme_name) {
                                app.theme = new_theme;
                            }
                        }
                    }
                }
                app.mode = AppMode::Normal;
            }
            KeyCode::Char(c) => {
                app.command_buffer.push(c);
            }
            KeyCode::Backspace => {
                app.command_buffer.pop();
            }
            KeyCode::Esc => {
                app.mode = AppMode::Normal;
            }
            KeyCode::Tab => {
                let parts: Vec<&str> = app.command_buffer.split_whitespace().collect();
                if let Some(command) = parts.get(0) {
                    if *command == "theme" {
                        let current_theme = parts.get(1).unwrap_or(&"").to_lowercase();
                        let theme_variants = AppTheme::variants();
                        let current_index = theme_variants
                            .iter()
                            .position(|&v| v.to_lowercase() == current_theme);
                        let next_theme = match current_index {
                            Some(index) => {
                                theme_variants.get(index + 1).unwrap_or(&theme_variants[0])
                            }
                            None => &theme_variants[0],
                        };
                        app.command_buffer = format!("theme {}", next_theme);
                    }
                }
            }
            _ => {}
        },
        AppMode::Filtering => match key_code {
            KeyCode::Char(c) => {
                app.filter_query.push(c);
            }
            KeyCode::Backspace => {
                app.filter_query.pop();
            }
            KeyCode::Enter | KeyCode::Esc => {
                app.mode = AppMode::Normal;
            }
            _ => {}
        },
    }
}

//-----------------------------------------------------------------------------------------------------------

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        app.refresh();
        terminal.draw(|f| ui(f, app))?;

        if event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') && app.mode == AppMode::Normal {
                    return Ok(());
                }
                handle_key_event(app, key.code);
            }
        }
    }
}

//-------------------------------------------------------------------------------------------------------------
