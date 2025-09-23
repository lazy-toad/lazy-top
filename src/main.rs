use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use anyhow::Result;
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, TableState},
};
use std::{io, vec};
use sysinfo::System;

//-------------------------------------------------------------------------------------------------------

struct ColorTheme {
    pub base: Color,
    pub mantle: Color,
    pub text: Color,
    pub mauve: Color,
    pub pink: Color,
    pub yellow: Color,
    pub highlight_bg: Color,
    pub highlight_fg: Color,
}

#[derive(Clone)]
enum AppTheme {
    Nord,
    Gruvbox,
    SolarizedDark,
    MononokaiPro,
    GitHub,
    OrangeSunset,
}

impl AppTheme {
    fn get_colors(&self) -> ColorTheme {
        match self {
            // This is the theme you labeled "github dark" in the comments.
            AppTheme::Nord => ColorTheme {
                base: Color::Rgb(46, 52, 64),
                mantle: Color::Rgb(46, 52, 64),
                text: Color::Rgb(216, 222, 233),
                mauve: Color::Rgb(163, 190, 140),
                pink: Color::Rgb(180, 142, 173),
                yellow: Color::Rgb(136, 192, 208),
                highlight_bg: Color::Rgb(76, 86, 106),
                highlight_fg: Color::Rgb(236, 239, 244),
            },
            AppTheme::Gruvbox => ColorTheme {
                base: Color::Rgb(40, 40, 40),
                mantle: Color::Rgb(40, 40, 40),
                text: Color::Rgb(235, 219, 178),
                mauve: Color::Rgb(104, 157, 106),
                pink: Color::Rgb(214, 93, 14),
                yellow: Color::Rgb(215, 153, 33),
                highlight_bg: Color::Rgb(69, 133, 136),
                highlight_fg: Color::Rgb(40, 40, 40),
            },
            // This is the custom GitHub theme you defined in your original match.
            AppTheme::GitHub => ColorTheme {
                base: Color::Rgb(13, 17, 23),
                mantle: Color::Rgb(13, 17, 23),
                text: Color::Rgb(201, 209, 217),
                mauve: Color::Rgb(88, 166, 255),
                pink: Color::Rgb(188, 140, 255),
                yellow: Color::Rgb(139, 148, 158),
                highlight_bg: Color::Rgb(33, 38, 45),
                highlight_fg: Color::Rgb(88, 166, 255),
            },
            AppTheme::SolarizedDark => ColorTheme {
                base: Color::Rgb(0, 43, 54),
                mantle: Color::Rgb(0, 43, 54),
                text: Color::Rgb(131, 148, 150),
                mauve: Color::Rgb(42, 161, 152),
                pink: Color::Rgb(38, 139, 210),
                yellow: Color::Rgb(181, 137, 0),
                highlight_bg: Color::Rgb(88, 110, 117),
                highlight_fg: Color::Rgb(0, 43, 54),
            },
            AppTheme::MononokaiPro => ColorTheme {
                base: Color::Rgb(45, 42, 46),
                mantle: Color::Rgb(45, 42, 46),
                text: Color::Rgb(252, 252, 250),
                mauve: Color::Rgb(169, 220, 118),
                pink: Color::Rgb(255, 216, 102),
                yellow: Color::Rgb(255, 97, 136),
                highlight_bg: Color::Rgb(120, 220, 232),
                highlight_fg: Color::Rgb(45, 42, 46),
            },
            // This is the first, unnamed theme from your comments.
            AppTheme::OrangeSunset => ColorTheme {
                base: Color::Rgb(255, 171, 36),
                mantle: Color::Rgb(180, 85, 0),
                text: Color::Rgb(135, 206, 250),
                mauve: Color::Rgb(100, 149, 237),
                pink: Color::Rgb(70, 130, 180),
                yellow: Color::Rgb(224, 255, 255),
                highlight_bg: Color::Rgb(70, 130, 180),
                highlight_fg: Color::Rgb(215, 100, 0),
            },
        }
    }

    fn from_str(s: &str) -> Option<AppTheme> {
        match s.to_lowercase().as_str() {
            "nord" => Some(AppTheme::Nord),
            "gruvbox" => Some(AppTheme::Gruvbox),
            "solarizeddark" => Some(AppTheme::SolarizedDark),
            "orangesunset" => Some(AppTheme::OrangeSunset),
            "github" => Some(AppTheme::GitHub),
            "monokaipro" => Some(AppTheme::MononokaiPro),
            _ => None,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            AppTheme::Nord => "Nord",
            AppTheme::GitHub => "GitHub",
            AppTheme::Gruvbox => "Gruvbox",
            AppTheme::SolarizedDark => "SolarizedDark",
            AppTheme::OrangeSunset => "OrangeSunset",
            AppTheme::MononokaiPro => "MonokaiPro",
        }
    }

    fn variants() -> Vec<&'static str> {
        vec![
            "Nord",
            "Gruvbox",
            "SolarizedDark",
            "OrangeSunset",
            "Github",
            "MonokaiPro",
        ]
    }

    fn next(self) -> Self {
        match self {
            AppTheme::Nord => AppTheme::Gruvbox,
            AppTheme::Gruvbox => AppTheme::MononokaiPro,
            AppTheme::MononokaiPro => AppTheme::OrangeSunset,
            AppTheme::OrangeSunset => AppTheme::GitHub,
            AppTheme::GitHub => AppTheme::SolarizedDark,
            AppTheme::SolarizedDark => AppTheme::Nord,
        }
    }
}

#[derive(PartialEq)]
enum AppMode {
    Normal,
    Command,
}

struct ProcessItem {
    pid: String,
    name: String,
    cpu_usage: String,
}

struct App {
    sys: System,
    processes: Vec<ProcessItem>,
    table_state: TableState,
    theme: AppTheme,
    mode: AppMode,
    command_buffer: String,
}

impl App {
    fn new() -> Self {
        let mut table_state = TableState::default();
        table_state.select(Some(0));

        App {
            sys: System::new_all(),
            processes: Vec::new(),
            table_state,
            theme: AppTheme::Nord,
            mode: AppMode::Normal,
            command_buffer: String::new(),
        }
    }

    fn next(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.processes.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.processes.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.table_state.select(Some(i));
    }

    fn refresh(&mut self) {
        self.sys.refresh_all();

        self.processes.clear();

        for (pid, process) in self.sys.processes() {
            self.processes.push(ProcessItem {
                pid: pid.to_string(),
                name: process.name().to_string_lossy().into_owned(),
                cpu_usage: format!("{:.2}%", process.cpu_usage()),
            });
        }

        self.processes.sort_by(|a, b| {
            let a_cpu = a
                .cpu_usage
                .trim_end_matches('%')
                .parse::<f32>()
                .unwrap_or(0.0);
            let b_cpu = b
                .cpu_usage
                .trim_end_matches('%')
                .parse::<f32>()
                .unwrap_or(0.0);

            b_cpu
                .partial_cmp(&a_cpu)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }
}

//----------------------------------------------------------------------------------------------------------

fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    app.refresh();
    std::thread::sleep(std::time::Duration::from_millis(500));

    run_app(&mut terminal, &mut app)?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

//----------------------------------------------------------------------------------------------------------

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        //refresh data on every loop
        app.refresh();

        terminal.draw(|f| ui(f, app))?;

        if event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match app.mode {
                    AppMode::Normal => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char(':') => {
                            app.mode = AppMode::Command;
                            app.command_buffer.clear();
                        }
                        KeyCode::Down => app.next(),
                        KeyCode::Up => app.previous(),
                        KeyCode::Char('t') => app.theme = app.theme.clone().next(),
                        _ => {}
                    },
                    AppMode::Command => match key.code {
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
                                        Some(index) => theme_variants
                                            .get(index + 1)
                                            .unwrap_or(&theme_variants[0]),
                                        None => &theme_variants[0],
                                    };
                                    app.command_buffer = format!("theme {}", next_theme);
                                }
                            }
                        }
                        _ => {}
                    },
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let theme = app.theme.get_colors();

    f.render_widget(Clear, f.area());
    let base_block = Block::default().style(Style::default().bg(theme.base));
    f.render_widget(base_block, f.area());

    //layout with 2 chunks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Min(0)])
        .split(f.area());

    //sys info
    let sys_info_block = Block::default()
        .borders(Borders::ALL)
        .title(format!("SYSTEM INFO (Theme: {})", app.theme.as_str()))
        .border_style(Style::default().fg(theme.mauve));

    let total_mem = app.sys.total_memory() as f64 / (1024 * 1024 * 1024) as f64;
    let used_mem = app.sys.used_memory() as f64 / (1024 * 1024 * 1024) as f64;
    let mem_percent = (used_mem / total_mem) * 100.0;

    let cpu_usage = app.sys.global_cpu_usage();

    let info_text = format!(
        "CPU Usage: {:.2}%\n\nMemory Usage: {:.2}%\n{:.2} GiB / {:.2} GiB",
        cpu_usage, mem_percent, used_mem, total_mem
    );

    let sys_info_para = Paragraph::new(info_text)
        .block(sys_info_block)
        .style(Style::default().fg(theme.text));

    f.render_widget(sys_info_para, chunks[0]);

    //Process table
    let process_block = Block::default()
        .borders(Borders::ALL)
        .title("Processes")
        .border_style(Style::default().fg(theme.pink));
    let header_cells = ["PID", "Name", "CPU %"].iter().map(|h| {
        Cell::from(*h).style(
            Style::default()
                .fg(theme.yellow)
                .add_modifier(Modifier::BOLD),
        )
    });
    let header = Row::new(header_cells)
        .height(1)
        .style(Style::default().bg(theme.mantle));
    let rows = app.processes.iter().map(|item| {
        let cells = vec![
            Cell::from(item.pid.clone()),
            Cell::from(item.name.clone()),
            Cell::from(item.cpu_usage.clone()),
        ];
        let row_style = Style::default().fg(theme.text).bg(theme.base);
        Row::new(cells).height(1).style(row_style)
    });
    let highlight_style = Style::default()
        .bg(theme.highlight_bg)
        .fg(theme.highlight_fg)
        .add_modifier(Modifier::BOLD);
    let table = Table::new(
        rows,
        [
            Constraint::Length(10),
            Constraint::Min(20),
            Constraint::Length(10),
        ],
    )
    .header(header)
    .block(process_block)
    .row_highlight_style(highlight_style)
    .highlight_symbol(">> ");
    f.render_stateful_widget(table, chunks[1], &mut app.table_state);

    if app.mode == AppMode::Command {
        let command_text = format!(
            ":{} (Options: {})",
            app.command_buffer,
            AppTheme::variants().join(", ")
        );
        let command_paragraph =
            Paragraph::new(command_text).style(Style::default().fg(theme.text).bg(theme.mantle));
        f.render_widget(command_paragraph, chunks[2]);
    }
    // let process_block = Block::default().borders(Borders::ALL).title("Processes").border_style(Style::default().fg(theme.pink));

    // let header_cells = ["PID", "Name", "CPU %"].iter().map(|h| {
    //     Cell::from(*h).style(
    //         Style::default()
    //             .fg(ratatui::style::Color::White)
    //             .add_modifier(Modifier::BOLD),
    //     )
    // });

    // let header = Row::new(header_cells).height(1);

    // let rows = app.processes.iter().map(|item| {
    //     let cells = vec![
    //         Cell::from(item.pid.clone()),
    //         Cell::from(item.name.clone()),
    //         Cell::from(item.cpu_usage.clone()),
    //     ];

    //     let row_style = Style::default().fg(theme::TEXT).bg(theme::BASE);
    //     Row::new(cells).height(1).style(row_style)
    // });

    // let highlight_style = Style::default()
    //     .bg(theme::HIGHLIGHT_BG)
    //     .fg(theme::HIGHLIGHT_FG)
    //     .add_modifier(Modifier::BOLD);

    // let table = Table::new(
    //     rows,
    //     [
    //         Constraint::Length(10),
    //         Constraint::Min(20),
    //         Constraint::Length(10),
    //     ],
    // )
    // .header(header)
    // .block(process_block)
    // .highlight_style(highlight_style)
    // .highlight_symbol(">> ");

    // f.render_stateful_widget(table, chunks[1], &mut app.table_state);
}
