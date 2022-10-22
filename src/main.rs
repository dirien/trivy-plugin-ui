use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::string::String;

use tui::widgets::{TableState};

use clap::Parser;
use crate::cli::Args;

mod trivy;
mod cli;
mod lib;
mod ui;

pub struct App {
    state: TableState,
    image_name: String,
    trivy: trivy::Trivy,
    show_popup: bool,
    pop_scroll: u16,
}

impl App {
    fn new(image_name: String, trivy: trivy::Trivy) -> App {
        App {
            state: TableState::default(),
            image_name,
            trivy,
            show_popup: false,
            pop_scroll: 0,
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                i + 1
            }
            None => 3,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i <= 0 {
                    0
                } else {
                    i - 1
                }
            }
            None => 3,
        };
        self.state.select(Some(i));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let object = trivy::trivy(&args.image_name);

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new(args.image_name, object);
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    if app.show_popup {
                        app.pop_scroll = 0;
                        app.show_popup = false;
                    } else {
                        return Ok(());
                    }
                }
                KeyCode::Esc => {
                    if app.show_popup {
                        app.pop_scroll = 0;
                        app.show_popup = false;
                    } else {
                        return Ok(());
                    }
                }
                KeyCode::Down => {
                    if app.show_popup {
                        app.pop_scroll += 1;
                    } else {
                        app.next();
                    }
                }
                KeyCode::Up => {
                    if app.show_popup {
                        if app.pop_scroll > 0 {
                            app.pop_scroll -= 1;
                        }
                    } else {
                        app.previous();
                    }
                }
                KeyCode::Enter => app.show_popup = !app.show_popup,
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    ui::build_ui(f, app);
}


