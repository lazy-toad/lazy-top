use ratatui::style::Color;
use ratatui::widgets::TableState;
use sysinfo::{Pid, Process, System};

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

#[derive(PartialEq)]
pub enum AppMode {
    Normal,
    Command,
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
        }
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

        self.processes.clear();

        for (pid, process) in self.sys.processes() {
            self.processes.push(ProcessItem {
                pid: pid.to_string(),
                name: process.name().to_string_lossy().into_owned(),
                cpu_usage: format!("{:.2}%", process.cpu_usage()),
                memory: format!("{:.2} MB", process.memory() as f64 / (1024.0 * 1024.0)),
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

//-----------------------------------------------------------------------------------------------------------------------------------------------
