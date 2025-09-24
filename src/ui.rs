use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table},
};

use crate::app::{App, AppMode, AppTheme};

pub fn ui(f: &mut Frame, app: &mut App) {
    let theme = app.theme.get_colors();

    f.render_widget(Clear, f.area());
    let base_block = Block::default().style(Style::default().bg(theme.base));
    f.render_widget(base_block, f.area());

    //layout with 3 chunks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
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
}
