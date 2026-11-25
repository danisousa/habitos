use std::error::Error;
use std::time::Duration;

use habitos::{Game, HabitRow, Schedule};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Clear, Paragraph},
};

const TABS: [&str; 5] = ["Today", "Habits", "Run", "Pet", "Unlocks"];
const HABITS_TAB: usize = 1;
const ACCENT: Color = Color::Cyan;
const ASCII_BORDER: border::Set = border::Set {
    top_left: "+",
    top_right: "+",
    bottom_left: "+",
    bottom_right: "+",
    vertical_left: "|",
    vertical_right: "|",
    horizontal_top: "-",
    horizontal_bottom: "-",
};

enum Wizard {
    Name(String),
    Schedule(String),
}

struct App {
    game: Game,
    tab: usize,
    cursor: usize,
    wizard: Option<Wizard>,
    habits: Vec<HabitRow>,
}

pub fn run(game: Game) -> Result<(), Box<dyn Error>> {
    let mut app = App {
        game,
        tab: 0,
        cursor: 0,
        wizard: None,
        habits: vec![],
    };
    let mut terminal = ratatui::init();
    let result = event_loop(&mut terminal, &mut app);
    ratatui::restore();
    result
}

fn event_loop(terminal: &mut DefaultTerminal, app: &mut App) -> Result<(), Box<dyn Error>> {
    loop {
        app.habits = app.game.habits()?;
        app.cursor = app.cursor.min(app.habits.len().saturating_sub(1));
        terminal.draw(|f| draw(f, app))?;
        if !event::poll(Duration::from_millis(500))? {
            continue;
        }
        let Event::Key(key) = event::read()? else {
            continue;
        };
        if key.kind != KeyEventKind::Press {
            continue;
        }
        if !handle_key(app, key.code)? {
            return Ok(());
        }
    }
}

fn handle_key(app: &mut App, code: KeyCode) -> Result<bool, Box<dyn Error>> {
    if let Some(wizard) = app.wizard.take() {
        app.wizard = wizard_key(app, wizard, code)?;
        return Ok(true);
    }
    match code {
        KeyCode::Char('q') => return Ok(false),
        KeyCode::Tab => app.tab = (app.tab + 1) % TABS.len(),
        KeyCode::BackTab => app.tab = (app.tab + TABS.len() - 1) % TABS.len(),
        KeyCode::Char(c @ '1'..='5') => app.tab = c as usize - '1' as usize,
        _ if app.tab == HABITS_TAB => habits_key(app, code)?,
        _ => {}
    }
    Ok(true)
}

fn habits_key(app: &mut App, code: KeyCode) -> Result<(), Box<dyn Error>> {
    match code {
        KeyCode::Up | KeyCode::Char('k') => app.cursor = app.cursor.saturating_sub(1),
        KeyCode::Down | KeyCode::Char('j') => app.cursor += 1,
        KeyCode::Char('a') => app.wizard = Some(Wizard::Name(String::new())),
        KeyCode::Char(' ') => {
            if let Some(habit) = app.habits.get(app.cursor) {
                app.game.mark_habit_done_today(habit.id)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn wizard_key(
    app: &mut App,
    wizard: Wizard,
    code: KeyCode,
) -> Result<Option<Wizard>, Box<dyn Error>> {
    if code == KeyCode::Esc {
        return Ok(None);
    }
    Ok(match (wizard, code) {
        (Wizard::Name(mut name), KeyCode::Char(c)) => {
            name.push(c);
            Some(Wizard::Name(name))
        }
        (Wizard::Name(mut name), KeyCode::Backspace) => {
            name.pop();
            Some(Wizard::Name(name))
        }
        (Wizard::Name(name), KeyCode::Enter) if !name.trim().is_empty() => {
            Some(Wizard::Schedule(name.trim().to_string()))
        }
        (Wizard::Schedule(name), KeyCode::Enter) => {
            app.game.add_habit(&name, Schedule::EveryDay)?;
            app.cursor = app.habits.len();
            None
        }
        (wizard, _) => Some(wizard),
    })
}

fn draw(f: &mut Frame, app: &App) {
    let [tabs, _, body, keys] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .areas(f.area());
    draw_tabs(f, app, tabs);
    let body = Layout::horizontal([Constraint::Max(80)])
        .flex(Flex::Center)
        .areas::<1>(body)[0];
    if app.tab == HABITS_TAB {
        draw_habits(f, app, body);
    }
    if let Some(wizard) = &app.wizard {
        draw_wizard(f, wizard, body);
    }
    let hint: &[(&str, &str)] = match (&app.wizard, app.tab) {
        (Some(_), _) => &[("enter", "next"), ("esc", "cancel")],
        (None, HABITS_TAB) => &[
            ("up/down", "move"),
            ("space", "done today"),
            ("a", "add"),
            ("q", "quit"),
        ],
        (None, _) => &[("tab", "sections"), ("q", "quit")],
    };
    draw_hint(f, hint, keys);
}

fn draw_tabs(f: &mut Frame, app: &App, area: Rect) {
    let mut names = vec![Span::raw(" ")];
    let mut under = vec![Span::raw(" ")];
    for (i, name) in TABS.iter().enumerate() {
        let label = format!(" {} {name} ", i + 1);
        let width = label.len();
        if i == app.tab {
            names.push(label.fg(ACCENT).bold());
            under.push("-".repeat(width).fg(ACCENT));
        } else {
            names.push(label.dark_gray());
            under.push(" ".repeat(width).into());
        }
    }
    f.render_widget(
        Paragraph::new(vec![Line::from(names), Line::from(under)]),
        area,
    );
}

fn draw_habits(f: &mut Frame, app: &App, area: Rect) {
    let mut lines = vec![Line::from("Habits".bold()), Line::from("")];
    if app.habits.is_empty() {
        lines.push(Line::from(
            "   No Habits yet. Press a to add one.".dark_gray(),
        ));
    }
    for (i, habit) in app.habits.iter().enumerate() {
        let selected = i == app.cursor;
        lines.push(Line::from(vec![
            if selected {
                " > ".fg(ACCENT)
            } else {
                "   ".into()
            },
            if habit.done_today {
                "[x] ".green()
            } else {
                "[ ] ".dark_gray()
            },
            Span::styled(
                format!("{:<30}", habit.name),
                if selected {
                    Style::new().bold()
                } else {
                    Style::new()
                },
            ),
            schedule_label(habit.schedule).dark_gray(),
        ]));
    }
    f.render_widget(Paragraph::new(lines), area);
}

fn schedule_label(schedule: Schedule) -> &'static str {
    match schedule {
        Schedule::EveryDay => "every day",
    }
}

fn draw_wizard(f: &mut Frame, wizard: &Wizard, area: Rect) {
    let [popup] = Layout::vertical([Constraint::Length(9)])
        .flex(Flex::Center)
        .areas(area);
    let [popup] = Layout::horizontal([Constraint::Length(50)])
        .flex(Flex::Center)
        .areas(popup);
    f.render_widget(Clear, popup);
    let lines = match wizard {
        Wizard::Name(name) => vec![
            Line::from("Step 1 of 2".dark_gray()),
            Line::from(""),
            Line::from("What do you want to do?".bold()),
            Line::from(""),
            Line::from(vec![
                "  ".into(),
                name.clone().fg(ACCENT).bold(),
                "_".fg(ACCENT),
            ]),
        ],
        Wizard::Schedule(_) => vec![
            Line::from("Step 2 of 2".dark_gray()),
            Line::from(""),
            Line::from("How often?".bold()),
            Line::from(""),
            Line::from(vec![
                " > ".fg(ACCENT),
                schedule_label(Schedule::EveryDay).bold(),
            ]),
        ],
    };
    let block = Block::bordered()
        .border_set(ASCII_BORDER)
        .border_style(Style::new().fg(ACCENT))
        .title(" Add ");
    f.render_widget(Paragraph::new(lines).block(block), popup);
}

fn draw_hint(f: &mut Frame, pairs: &[(&str, &str)], area: Rect) {
    let mut spans = vec![];
    for (key, action) in pairs {
        spans.push(format!(" {key} ").black().on_dark_gray());
        spans.push(format!(" {action}   ").dark_gray());
    }
    f.render_widget(
        Paragraph::new(Line::from(spans)).alignment(Alignment::Center),
        area,
    );
}
