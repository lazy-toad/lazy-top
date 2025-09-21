use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{
    Frame, Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, Borders, Paragraph},
};

use anyhow::Result;
use std::io;
use sysinfo::{Cpu, System};

struct App {
    sys: System,
}

impl App {
    fn new() -> Self {
        App {
            sys: System::new_all(),
        }
    }

    fn refresh(&mut self) {
        self.sys.refresh_cpu_all();
        self.sys.refresh_memory();
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    run_app(&mut terminal, &mut app)?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        //refresh data on every loop
        app.refresh();

        terminal.draw(|f| ui(f, app))?;

        if event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    //layout with 2 chunks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Min(0)])
        .split(f.size());

    //sys info
    let sys_info_block = Block::default().borders(Borders::ALL).title("System Info");

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
        .style(Style::default().fg(ratatui::style::Color::LightBlue));

    f.render_widget(sys_info_para, chunks[0]);

    let process_block = Block::default().borders(Borders::ALL).title("Processes");
    f.render_widget(process_block, chunks[1]);
}
