use tui::{
    backend::{Backend},
    layout::{Constraint, Layout, Rect, Direction, Alignment},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, Paragraph, Wrap},
    Frame, text::{Spans, Span, Text}
};

use crate::{App};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {

    //let verbose_table = raw_table_to_typetable(&app.verbose_table, vec!["Verbose Level","Log Count"]);
    let rects = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(95),Constraint::Percentage(5)])
        .split(f.size());

    ui_logs(f, app, rects.clone());
    ui_legend(f, app, rects.clone());

}

fn ui_logs<B: Backend>(f: &mut Frame<B>, app:&mut App,rects:Vec<Rect>){
    let log_table = raw_table_to_typetable(&app.view_items, vec!["#","Time","Severity","Component","Context","Message"]);

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Blue);
    let header_cells = log_table.headers
        .iter()
        .map(|h| Cell::from(h.to_string()).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let rows = log_table.rows.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(&*c.as_str()));
        Row::new(cells).height(height as u16).bottom_margin(1)
    });
    let blockstyle:Style =  if app.log_view_active{ Style::default().fg(Color::Cyan)}
        else{
            Style::default().fg(Color::White)
    };
    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).border_style(blockstyle).title("MONLOMON"))
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(10),
            Constraint::Percentage(20),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(40)
        ]);
    
    let main_rect = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(60),Constraint::Percentage(40)])
    .horizontal_margin(2)
    .split(rects[0]);
    f.render_stateful_widget(t, main_rect[0], &mut app.state);
    
    let blockstyle:Style =  if app.log_view_active{ Style::default().fg(Color::White)}
        else{
            Style::default().fg(Color::Cyan)
    };
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(blockstyle)
        .title("Details")
        .title_alignment(Alignment::Center);
    
    let selected_log = &app.selected_log;
    if selected_log.is_some(){
        let log = selected_log.clone().unwrap();
        let attr = log.attr;
        //let obj = json!({&attr});
        let j = serde_json::to_string_pretty(&attr);
        let jsonstr = j.unwrap();
        //let text = vec![Spans::from(json_string_to_span_array(jsonstr.as_str()))];
        let paragraph = Paragraph::new(Text::from(jsonstr))
            .style(Style::default().bg(Color::Reset).fg(Color::Reset))
            .block(block)
            .alignment(Alignment::Left)
            .scroll(app.details_offset);
        f.render_widget(paragraph, main_rect[1]);
    }
    else{
        let paragraph = Paragraph::new(Text::from("No log selected"))
            .style(Style::default().bg(Color::Reset).fg(Color::Reset))
            .block(block)
            .alignment(Alignment::Left)
            .scroll(app.details_offset);
        f.render_widget(paragraph, main_rect[1]);
    }
}
fn ui_legend<B: Backend>(f: &mut Frame<B>, app: &mut App,rects:Vec<Rect>) {
    let legend_rect = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .horizontal_margin(2)
        .split(Rect { x: 0, y: rects[0].height, width: f.size().width, height: 1 });
    
        let informational_status = if app.verbose_filters.informational==true{
            (Color::Green,Color::Rgb(228, 228, 228))
        }
        else{
            (Color::Red,Color::Rgb(228, 228, 228))
        };
        let warning_status = if app.verbose_filters.warning==true{
            (Color::Green,Color::Rgb(228, 228, 228))
        }
        else{
            (Color::Red,Color::Rgb(228, 228, 228))
        };
        let error_status = if app.verbose_filters.error==true{
            (Color::Green,Color::Rgb(228, 228, 228))
        }
        else{
            (Color::Red,Color::Rgb(228, 228, 228))
        };
        let fatal_status = if app.verbose_filters.fatal==true{
            (Color::Green,Color::Rgb(133, 228, 228))
        }
        else{
            (Color::Red,Color::Rgb(228, 228, 228))
        };
    // println!("{:?}",legend_rect);
    let legend_paragraph = Paragraph::new(vec![
        Spans::from(vec![
            make_span_hint("[Q]"),Span::raw("uit"),make_span_spacer(),
            make_span_hint("[↑]"),Span::raw("Scroll Up"),make_span_spacer(),
            make_span_hint("[↓]"),Span::raw("Scroll Down"),make_span_spacer(),
            make_span_hint("[Tab]"),Span::raw("Switch Panes"),make_span_spacer(),
            make_span_spacer(),
            Span::raw("Verbose Level Visibility: "),
            make_span_hint("[I]"),
            Span::styled("nformational",Style::default().bg(informational_status.0).fg(informational_status.1)),
            make_span_spacer(),
            make_span_hint("[W]"),
            Span::styled("arning",Style::default().bg(warning_status.0).fg(warning_status.1)),
            make_span_spacer(),
            make_span_hint("[E]"),
            Span::styled("rror",Style::default().bg(error_status.0).fg(error_status.1)),
            make_span_spacer(),
            make_span_hint("[F]"),
            Span::styled("atal",Style::default().bg(fatal_status.0).fg(fatal_status.1)),
        ])
    ])
    .style(Style::default().bg(Color::Reset).fg(Color::Reset))
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true })
    .block(Block::default().style(Style::default().bg(Color::Reset).fg(Color::Reset)));
    f.render_widget(legend_paragraph, legend_rect[0]);
}


fn make_span_spacer() -> Span<'static>{
    Span::styled("  ", Style::default().fg(Color::Reset).bg(Color::Reset))
}
fn make_span_hint(s:&str) -> Span{
    Span::styled(s, Style::default().fg(Color::White).bg(Color::Cyan).add_modifier(Modifier::UNDERLINED).add_modifier(Modifier::BOLD))
}

pub fn raw_table_to_typetable<'a>(raw:&'a Vec<Vec<String>>,headers:Vec<&'a str>)->VecTable<'a>{
    let rows:Vec<Vec<String>> = raw.iter().map(|r| r.iter().map(|sr|sr.to_string()).collect()).collect();
    return VecTable{
        headers,
        rows
    }
}
#[derive(Debug,Clone)]
pub struct VecTable<'a>{
    pub headers:Vec<&'a str>,
    pub rows:Vec<Vec<String>>
}