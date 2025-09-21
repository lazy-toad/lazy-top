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
    style::{Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};
use std::{io, vec};
use sysinfo::System;

//-------------------------------------------------------------------------------------------------------

struct ProcessItem {
    pid: String,
    name: String,
    cpu_usage: String,
}

struct App {
    sys: System,
    processes: Vec<ProcessItem>,
}

impl App {
    fn new() -> Self {
        App {
            sys: System::new_all(),
            processes: Vec::new(),
        }
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
        .split(f.area());

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

    //Process table
    let process_block = Block::default().borders(Borders::ALL).title("Processes");

    let header_cells = ["PID", "Name", "CPU %"].iter().map(|h| {
        Cell::from(*h).style(
            Style::default()
                .fg(ratatui::style::Color::White)
                .add_modifier(Modifier::BOLD),
        )
    });

    let header = Row::new(header_cells).height(1);

    let rows = app.processes.iter().map(|item| {
        let cells = vec![
            Cell::from(item.pid.clone()),
            Cell::from(item.name.clone()),
            Cell::from(item.cpu_usage.clone()),
        ];
        Row::new(cells).height(1)
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(10),
            Constraint::Min(20),
            Constraint::Length(10),
        ],
    )
    .header(header)
    .block(process_block);

    f.render_widget(table, chunks[1]);
}
