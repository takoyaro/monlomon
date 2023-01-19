use tui::{
    backend::{Backend},
    layout::{Constraint, Layout, Rect, Direction, Alignment},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, Paragraph, Wrap},
    Frame, text::{Spans, Span},
};

use crate::App;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {

    //let verbose_table = raw_table_to_typetable(&app.verbose_table, vec!["Verbose Level","Log Count"]);
    let log_table = raw_table_to_typetable(&app.view_items, vec!["#","Time","Severity","Component","Context","Message"]);
    let rects = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(95),Constraint::Percentage(5)])
        .margin(2)
        .split(f.size());

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Blue);
    let header_cells = log_table.Headers
        .iter()
        .map(|h| Cell::from(h.to_string()).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let rows = log_table.Rows.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(&*c.as_str()));
        Row::new(cells).height(height as u16).bottom_margin(1)
    });
    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Latest Events Counts"))
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
    f.render_stateful_widget(t, rects[0], &mut app.state);
    
    let legend_rect = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .horizontal_margin(2)
        .split(Rect { x: 0, y: rects[0].height+2, width: f.size().width, height: 1 });
    
    let informational_status = if app.verbose_filters.informational==true{
        "Show"
    }
    else{
        "Hide"
    };
    // println!("{:?}",legend_rect);
    let legend_paragraph = Paragraph::new(vec![
        Spans::from(vec![
            Span::styled("Q", Style::default().fg(Color::White).bg(Color::Green).add_modifier(Modifier::UNDERLINED).add_modifier(Modifier::BOLD)),
            Span::raw("uit"),
            make_span_spacer(),
            Span::styled("↑", Style::default().fg(Color::White).bg(Color::Green).add_modifier(Modifier::UNDERLINED).add_modifier(Modifier::BOLD)),
            Span::raw("Up"),
            make_span_spacer(),
            Span::styled("↓", Style::default().fg(Color::White).bg(Color::Green).add_modifier(Modifier::UNDERLINED).add_modifier(Modifier::BOLD)),
            Span::raw("Down"),
            make_span_spacer(),
            Span::styled("⮐ ", Style::default().fg(Color::White).bg(Color::Green).add_modifier(Modifier::UNDERLINED).add_modifier(Modifier::BOLD)),
            Span::raw("View Details"),
            make_span_spacer(),
            Span::styled("I", Style::default().fg(Color::White).bg(Color::Green).add_modifier(Modifier::UNDERLINED)),
            Span::raw(informational_status),
            Span::raw(" Informational logs")
        ])
    ])
    .style(Style::default().bg(Color::White).fg(Color::Black))
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true })
    .block(Block::default().style(Style::default().bg(Color::White).fg(Color::Black)));
    f.render_widget(legend_paragraph, legend_rect[0]);
}

pub fn make_span_spacer() -> Span<'static>{
    Span::styled("  ", Style::default().fg(Color::Black).bg(Color::Reset).add_modifier(Modifier::UNDERLINED))
}
pub fn raw_table_to_typetable<'a>(raw:&'a Vec<Vec<String>>,headers:Vec<&'a str>)->VecTable<'a>{
    let rows:Vec<Vec<String>> = raw.iter().map(|r| r.iter().map(|sr|sr.to_string()).collect()).collect();
    return VecTable{
        Headers:headers,
        Rows:rows
    }
}
#[derive(Debug,Clone)]
pub struct VecTable<'a>{
    pub Headers:Vec<&'a str>,
    pub Rows:Vec<Vec<String>>
}