mod types;
mod utils;
mod ui;

use types::MongoLog;
use utils::make_object_from_untyped_object;
use ui::ui;

use std::io::{stdin};
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
    pub log_view_active: bool,
    pub details_offset: (u16,u16),
    pub logs:Vec<MongoLog>,
    pub view_items: Vec<Vec<String>>,
    pub view_logs: Vec<MongoLog>,
    pub verbose_filters:VerboseFilters,
    pub selected_log:Option<MongoLog>,
    pub filtered_msgs:Vec<String>
}
pub struct VerboseFilters {
    pub informational:bool,
    pub warning:bool,
    pub error:bool,
    pub fatal:bool
}
impl App {
    fn new() -> App {
        App {
            state: TableState::default(),
            log_view_active:true,
            details_offset:(0,0),
            logs: vec![],
            view_items: vec![],
            view_logs:vec![],
            verbose_filters: VerboseFilters { informational: true, warning: true, error: true, fatal: true },
            selected_log: None,
            filtered_msgs: vec![]
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

            let filtered_logs:Vec<&MongoLog> = self.logs.iter().filter(|l| 
                flags.iter().any(|f|f==&l.s.as_str()) == true
                && 
                self.filtered_msgs.contains(&l.msg)==false
            ).collect();
            self.view_logs = filtered_logs.iter().map(|l| l.clone().to_owned()).collect();
            self.view_items = filtered_logs.iter().enumerate().map(|(id,l)| 
                vec![
                    id.to_string(),
                    l.t.to_string(),
                    l.s.to_string(),
                    l.c.to_string(),
                    l.ctx.to_string(),
                    l.msg.to_string()
                ]
            ).collect();
            
            self.state.select(None);
            self.selected_log = None;
            if !self.view_items.is_empty(){
                self.next();
            }
        }   
        else{
            println!("No logs here, friend");
        }
    }
    pub fn next(&mut self) {
        if self.log_view_active{
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
            self.set_selected_log();
            self.details_offset = (0,0);
        }
        else{
            self.details_offset.0 +=1;
        }
    }

    pub fn previous(&mut self) {
        if self.log_view_active{
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
            self.set_selected_log();
            self.details_offset = (0,0);
        }
        else{
            let checked = self.details_offset.0.checked_sub(1);
            if checked.is_some() {
                self.details_offset.0 -=1;
            }
        }
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
    pub fn toggle_verbosity_warning(&mut self){
        self.verbose_filters.warning = !self.verbose_filters.warning;
        self.filter_logs();
    }
    pub fn toggle_verbosity_error(&mut self){
        self.verbose_filters.error = !self.verbose_filters.error;
        self.filter_logs();
    }
    pub fn toggle_verbosity_fatal(&mut self){
        self.verbose_filters.fatal = !self.verbose_filters.fatal;
        self.filter_logs();
    }
    pub fn set_selected_log(&mut self){
        let index = self.state.selected();
        if index.is_none(){
            self.selected_log = None
        }
        else{
            let log = &self.view_logs[index.unwrap()];
            self.selected_log = Some(log.clone())
        }
    }
    pub fn exclude_selected_msg(&mut self){
        let log = self.get_log_for_selection();
        if log.is_some(){
            let msg = &log.unwrap().msg;
            if !self.filtered_msgs.contains(msg){
                self.filtered_msgs.push(msg.to_string());
                self.filter_logs();
            }
        }
    }
    pub fn reset_msg_filter(&mut self){
        self.filtered_msgs = vec![];
        self.filter_logs();
    }
    fn get_log_for_selection(&mut self) -> Option<MongoLog>{
        if self.state.selected().is_some(){
            let log = self.view_logs.get(self.state.selected().unwrap());
            let clone = log.unwrap().clone();
            return Some(clone)
        }
        return None
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
                KeyCode::Char('c') => return Ok(()),
                KeyCode::Down => app.next(),
                KeyCode::Up => app.previous(),
                KeyCode::Enter => app.enter(),
                KeyCode::Tab => app.log_view_active=!app.log_view_active,
                KeyCode::Char('r') => app.reset_msg_filter(),
                KeyCode::Char('i') => app.toggle_verbosity_informational(),
                KeyCode::Char('w') => app.toggle_verbosity_warning(),
                KeyCode::Char('e') => app.toggle_verbosity_error(),
                KeyCode::Char('f') => app.toggle_verbosity_fatal(),
                KeyCode::Char('-') => app.exclude_selected_msg(),
                _ => {}
            }
        }
    }
}