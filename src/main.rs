mod types;
mod utils;
mod ui;

use types::MongoLog;
use utils::make_object_from_untyped_object;
use ui::ui;

use std::{io::{stdin}, collections::HashMap};
use serde_json::Value;
use std::io;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::{TableState}, Terminal,
};

pub struct App {
    pub state: TableState,
    pub logs:Vec<MongoLog>,
    pub view_items: Vec<Vec<String>>,
    pub verbose_table:Vec<Vec<String>>,
    pub verbose_filters:VerboseFilters
}
pub struct VerboseFilters {
    pub informational:bool,
    pub warning:bool,
    pub error:bool,
    pub fatal:bool
}
impl<'a> App {
    fn new() -> App {
        App {
            state: TableState::default(),
            logs: vec![],
            view_items: vec![],
            verbose_table: vec![],
            verbose_filters: VerboseFilters { informational: true, warning: true, error: true, fatal: true }
        }
    }
    fn push(&mut self, row:MongoLog){
        self.logs.push(row);
    }
    fn filter_logs(&mut self){
        if self.logs.len()>0{
            let mut flags:Vec<&str> = vec![];
            if self.verbose_filters.informational==true {flags.push("Informational")}
            if self.verbose_filters.warning==true {flags.push("Warning")}
            if self.verbose_filters.error==true {flags.push("Error")}
            if self.verbose_filters.fatal==true {flags.push("Fatal")}

            let filtered_logs:Vec<&MongoLog> = self.logs.iter().filter(|l| flags.iter().any(|f|f==&l.s.as_str())).collect();
            self.view_items = filtered_logs.iter().enumerate().map(|(id,l)| vec![id.to_string(),l.t.to_string(),l.s.to_string(),l.c.to_string(),l.ctx.to_string(),l.msg.to_string()]).collect();
        }
        else{
            println!("No logs here, friend");
        }
    }
    pub fn filter_msg(&mut self,msg:&str){
        let filtered_items:Vec<&MongoLog> = self.logs.iter().filter(|l|l.msg==msg).collect();
        self.view_items = filtered_items.iter().map(|i| vec![i.t.to_string(),i.s.to_string(),i.c.to_string(),i.msg.to_string()]).collect();
    }
    pub fn filter_verbose(&mut self){
        let mut filtered_items:HashMap<String,Vec<&MongoLog>> = HashMap::new();
        for log in &self.logs {
            let key = &log.s;
            filtered_items.entry(key.to_string()).or_insert(vec![]).push(log);
        }
        let mut table_vec:Vec<Vec<String>> = Vec::new();
        for key in filtered_items.keys() {
            table_vec.push(vec![key.clone(),filtered_items.get(key).unwrap().len().to_string()])
        }
        self.verbose_table = table_vec;
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.view_items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.view_items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub fn enter(&mut self){
        let index = self.state.selected();
        if index.is_some(){
            println!("{}",index.unwrap());
        }
    }
    pub fn toggle_verbosity_informational(&mut self){
        self.verbose_filters.informational = !self.verbose_filters.informational;
        self.filter_logs();
    }
}


fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();

    let stdin = stdin();
    let lines = stdin.lines();
    for line in lines {
        if line.is_ok(){
            let parsed:Result<Value,serde_json::Error> = serde_json::from_str(&line.unwrap());
            if parsed.is_ok(){
                let log= make_object_from_untyped_object(&parsed.as_ref().unwrap());
                app.push(log)
            }
        }
    }
    app.filter_verbose();
    app.filter_logs();
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
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => app.next(),
                KeyCode::Up => app.previous(),
                KeyCode::Char('c') => return Ok(()),
                KeyCode::Enter => app.enter(),
                KeyCode::Char('r') => app.filter_logs(),
                KeyCode::Char('i') => app.toggle_verbosity_informational(),
                _ => {}
            }
        }
    }
}