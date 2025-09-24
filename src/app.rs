use ratatui::style::Color;
use ratatui::widgets::TableState;
use std::cmp::Ordering;
use sysinfo::{Pid, System};

//-----------------------------------------------------------------------------------------------------------------

pub struct ColorTheme {
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
pub enum AppTheme {
    Nord,
    Gruvbox,
    SolarizedDark,
    MononokaiPro,
    GitHub,
    OrangeSunset,
}

impl AppTheme {
    pub fn get_colors(&self) -> ColorTheme {
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

    pub fn from_str(s: &str) -> Option<AppTheme> {
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

    pub fn as_str(&self) -> &'static str {
        match self {
            AppTheme::Nord => "Nord",
            AppTheme::GitHub => "GitHub",
            AppTheme::Gruvbox => "Gruvbox",
            AppTheme::SolarizedDark => "SolarizedDark",
            AppTheme::OrangeSunset => "OrangeSunset",
            AppTheme::MononokaiPro => "MononokaiPro",
        }
    }

    pub fn variants() -> Vec<&'static str> {
        vec![
            "Nord",
            "Gruvbox",
            "SolarizedDark",
            "OrangeSunset",
            "GitHub",
            "MononokaiPro",
        ]
    }

    pub fn next(self) -> Self {
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

#[derive(Debug, PartialEq)]
pub enum AppMode {
    Normal,
    Command,
    Filtering,
}

pub struct ProcessItem {
    pub pid: String,
    pub name: String,
    pub cpu_usage: String,
    pub memory: String,
}

pub struct App {
    pub sys: System,
    pub processes: Vec<ProcessItem>,
    pub table_state: TableState,
    pub theme: AppTheme,
    pub mode: AppMode,
    pub command_buffer: String,
    
    pub sort_by: SortBy,
    pub filter_query: String,
}

impl App {
    pub fn new() -> Self {
        let mut table_state = TableState::default();
        table_state.select(Some(0));

        App {
            sys: System::new_all(),
            processes: Vec::new(),
            table_state,
            theme: AppTheme::Nord,
            mode: AppMode::Normal,
            command_buffer: String::new(),
            sort_by: SortBy::Cpu,
            filter_query: String::new(),
        }
    }

    pub fn cycle_sort_coloumn(&mut self) {
        self.sort_by = self.sort_by.next();
    }

    pub fn kill_selected_process(&mut self) {
        if let Some(selected) = self.table_state.selected() {
            if let Some(item) = self.processes.get(selected) {
                if let Ok(pid) = item.pid.parse::<usize>() {
                    if let Some(process) = self.sys.process(Pid::from(pid)) {
                        process.kill();
                    }
                }
            }
        }
    }

    pub fn next(&mut self) {
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

    pub fn refresh(&mut self) {
        self.sys.refresh_all();

        // Temporarily hold processes
        let mut processes: Vec<ProcessItem> = self
            .sys
            .processes()
            .iter()
            .map(|(pid, process)| ProcessItem {
                pid: pid.to_string(),
                name: process.name().to_string_lossy().into_owned(),
                cpu_usage: format!("{:.2}%", process.cpu_usage()),
                memory: format!("{:.2} MB", process.memory() as f64 / 1024.0 / 1024.0),
            })
            .collect();

        // Filter the processes if a query exists
        if !self.filter_query.is_empty() {
            let query = self.filter_query.to_lowercase();
            processes.retain(|p| p.name.to_lowercase().contains(&query));
        }

        // Sort the processes based on the current sort_by state
        processes.sort_by(|a, b| {
            match self.sort_by {
                SortBy::Pid => {
                    let pid_a = a.pid.parse::<usize>().unwrap_or(0);
                    let pid_b = b.pid.parse::<usize>().unwrap_or(0);
                    pid_a.cmp(&pid_b)
                }
                SortBy::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                SortBy::Cpu => {
                    let cpu_a = a
                        .cpu_usage
                        .trim_end_matches('%')
                        .parse::<f32>()
                        .unwrap_or(0.0);
                    let cpu_b = b
                        .cpu_usage
                        .trim_end_matches('%')
                        .parse::<f32>()
                        .unwrap_or(0.0);
                    cpu_b.partial_cmp(&cpu_a).unwrap_or(Ordering::Equal) // Note: Higher CPU is better
                }
                SortBy::Memory => {
                    let mem_a = a
                        .memory
                        .split_whitespace()
                        .next()
                        .unwrap_or("0")
                        .parse::<f32>()
                        .unwrap_or(0.0);
                    let mem_b = b
                        .memory
                        .split_whitespace()
                        .next()
                        .unwrap_or("0")
                        .parse::<f32>()
                        .unwrap_or(0.0);
                    mem_b.partial_cmp(&mem_a).unwrap_or(Ordering::Equal) // Note: Higher Memory is "greater"
                }
            }
        });

        // Now assign the processed list to the app state
        self.processes = processes;

        // Ensure selection is not out of bounds
        if self.table_state.selected().is_some()
            && self.table_state.selected().unwrap() >= self.processes.len()
        {
            self.table_state
                .select(Some(self.processes.len().saturating_sub(1)));
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortBy {
    Pid,
    Name,
    Cpu,
    Memory,
}

impl SortBy {
    pub fn next(self) -> Self {
        match self {
            SortBy::Pid => Self::Name,
            SortBy::Name => SortBy::Cpu,
            SortBy::Cpu => SortBy::Memory,
            SortBy::Memory => SortBy::Pid,
        }
    }
}
//-----------------------------------------------------------------------------------------------------------------------------------------------
