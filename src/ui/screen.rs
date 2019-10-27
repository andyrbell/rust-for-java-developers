use std::io;
use std::io::Write;

use termion::cursor::Goto;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::{TermionBackend, Backend};
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::{Terminal, Frame};
use tui::widgets::{Block, Borders, Paragraph, SelectableList, Tabs, Text, Widget};

use util::event::{Event, Events};
use crate::ui::app::Mode;

use super::app::App;
use super::util;
use crate::Options;

pub fn run(options: Options) -> Result<(), failure::Error> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();
    // App
    let mut app = App::new(options.offline)?;

    loop {
        draw(&mut terminal, &app)?;

        // stdout is buffered, flush it to see the effect immediately when hitting backspace.
        io::stdout().flush().ok();

        handle_events(&events, &mut app)?;

        if app.should_quit {
            break;
        }
    }
    Ok(())
}

fn handle_events(events: &Events, app: &mut App) -> Result<(), failure::Error> {
    match events.next()? {
        Event::Input(input) => match input {
            Key::Ctrl('c') | Key::Ctrl('d') => app.should_quit = true,
            Key::Char('\t') => app.next_tab()?,
            Key::Left => app.previous_tab()?,
            Key::Right => app.next_tab()?,
            Key::Down => app.next_talk(),
            Key::Up => app.previous_talk(),
            Key::Char('\n') if app.mode == Mode::Normal => {
                if let Some(selected) = app.selected {
                    if let Some(_talk) = app.talks.get(selected) {
//                          pressed enter on a talk
                    }
                }
            }
            Key::Char('\n') if app.mode == Mode::Search => {
                app.mode = Mode::Filtered;
            }
            Key::Char('/') => {
                app.mode = Mode::Search;
            }
            Key::Esc => {
                app.search_text = "".to_string();
                app.mode = Mode::Normal
            },
            Key::Backspace if app.mode == Mode::Search => {
                app.search_text.pop();
                app.selected = Some(0);
            }
            Key::Char(c) if app.mode == Mode::Search => {
                app.search_text.push(c);
                app.selected = Some(0);
            }
            _ => {}
        },
        Event::Tick => {
            app.advance();
        }
    }
    Ok(())
}

fn draw<B: Backend + std::io::Write>(terminal: &mut Terminal<B>, app: &App) -> Result<(), failure::Error> {
    terminal.draw(|mut f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(8), // banner
                    Constraint::Length(3), // tabs
                    Constraint::Min(20),   // main content
                    Constraint::Length(1)  // search bar
                ].as_ref()
            )
            .split(f.size());

        let (banner_panel, tab_panel, main_panel, search_panel) =
            (chunks[0], chunks[1], chunks[2], chunks[3]);

        draw_banner(&mut f, banner_panel);
        draw_tabs(&mut f, tab_panel, app);
        draw_search_bar(&mut f, search_panel, &app);
        draw_main_content(&mut f, main_panel, app);

    }).map_err(failure::Error::from)?;
    manage_cursor(terminal, app)
}

fn manage_cursor<B>(terminal: &mut Terminal<B>, app: &App) -> Result<(), failure::Error>
    where
        B: Backend + std::io::Write
{
    if let Ok(rect) = terminal.size() {
        write!(
            terminal.backend_mut(),
            "{}",
            Goto((app.search_text.len() +2 ) as u16, rect.height  as u16)
        )?;
    }

    if app.mode == Mode::Search {
        terminal.show_cursor()?;
    } else {
        terminal.hide_cursor()?;
    }
    Ok(())
}

fn draw_banner<B>(f: &mut Frame<B>, area: Rect)
    where
        B: Backend
{
    let banner = vec![
        Text::raw(String::from(r#"________                                      _________      .__               .___    .__          "#) + "\n"),
        Text::raw(String::from(r#"\______ \   _______  _________  ______  ___  /   _____/ ____ |  |__   ____   __| _/_ __|  |   ____  "#) + "\n"),
        Text::raw(String::from(r#" |    |  \_/ __ \  \/ /  _ \  \/  /\  \/  /  \_____  \_/ ___\|  |  \_/ __ \ / __ |  |  \  | _/ __ \ "#) + "\n"),
        Text::raw(String::from(r#" |    `   \  ___/\   (  <_> >    <  >    <   /        \  \___|   Y  \  ___// /_/ |  |  /  |_\  ___/ "#) + "\n"),
        Text::raw(String::from(r#"/_______  /\___  >\_/ \____/__/\_ \/__/\_ \ /_______  /\___  >___|  /\___  >____ |____/|____/\___  >"#) + "\n"),
        Text::raw(String::from(r#"        \/     \/                \/      \/         \/     \/     \/     \/     \/               \/ "#) + "\n")
    ];

    Paragraph::new(banner.iter())
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Left)
        .style(Style::default().fg(Color::Cyan))
        .wrap(false)
        .render(f, area);
}

fn draw_tabs<B>(f: &mut Frame<B>, area: Rect, app: &App)
    where
        B: Backend
{
    Tabs::default()
        .block(Block::default().borders(Borders::ALL).title("Day"))
        .titles(&["Monday","Tuesday","Wednesday","Thursday", "Friday"])
        .select(app.day.num_days_from_monday() as usize)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(Style::default().fg(Color::Yellow))
        .render(f, area);
}

fn draw_main_content<B>(f: &mut Frame<B>, area: Rect, app: &App)
    where
        B: Backend
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // master panel
            Constraint::Percentage(50)  // detail panel
        ].as_ref())
        .split(area);

    let (master, detail) = (chunks[0], chunks[1]);

    draw_master(f, master, app);
    draw_detail(f, detail, app);
}

fn draw_master<B>(f: &mut Frame<B>, area: Rect, app: &App)
    where
        B: Backend
{
    let style = Style::default().fg(Color::White).bg(Color::Black);
    SelectableList::default()
        .block(Block::default().borders(Borders::ALL).title("Schedule"))
        .items(&app.talk_titles())
        .select(app.selected)
        .style(style)
        .highlight_style(style.bg(Color::LightGreen).fg(Color::White).modifier(Modifier::BOLD))
        .highlight_symbol(">")
        .render(f, area);
}

fn draw_detail<B>(f: &mut Frame<B>, area: Rect, app: &App)
    where
        B: Backend
{
    let text = match app.get_selected() {
        // TODO
        _ => vec![Text::raw(String::from("TODO: Talk details"))]
    };

    Paragraph::new(text.iter())
        .block(Block::default().title("Details").borders(Borders::ALL))
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Left)
        .wrap(true)
        .render(f, area);

}

fn draw_search_bar<B>(f: &mut Frame<B>, area: Rect, app: &App)
    where
        B: Backend
{
    if app.mode == Mode::Search || app.mode == Mode::Filtered {
        Paragraph::new([Text::raw(format!("/{}", app.search_text))].iter())
            .style(Style::default().fg(Color::Yellow))
            .render(f, area);
    }
}