// PROTOTYPE — throwaway. Round 2 of "TUI views and layout" (issue #25): one simple, friendly design.
// Top tabs, a home screen with Pet + today's list + Play, card choices, ASCII sprites, short animations.
use std::time::{Duration, Instant};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, Paragraph},
};

const TABS: [&str; 5] = ["Today", "Habits", "Run", "Pet", "Unlocks"];
const FREEZE_COST: i32 = 15;
const FREEZE_CAP: u32 = 3;
const SLOTS: usize = 4;
const HIT_MS: u128 = 380;
const ACCENT: Color = Color::Cyan;

const HERO: [&str; 5] = ["   ___     ", "  (o_o)    ", " /|___|\\=> ", "  |   |    ", " _/   \\_   "];
const HERO_ATK: [&str; 5] = ["    ___       ", "   (>_<)      ", "  /|___|\\====>", "   |   |      ", "  _/   \\_     "];
const HERO_HURT: [&str; 5] = ["  ___      ", " (x_x)     ", "\\|___|/=>  ", " |   |     ", "_/   \\_    "];
const PET_IDLE: [&str; 3] = ["  ,_,  ", " (o.o) ", " /)_)~ "];
const PET_BLINK: [&str; 3] = ["  ,_,  ", " (-.-) ", " /)_)~ "];
const PET_HAPPY: [&str; 3] = [" \\,_,/ ", " (^o^) ", "  /)_)~"];

const GOBLIN: [&str; 5] = ["  ^   ^   ", " (>.<)/   ", "/|=-=|\\   ", " |   |    ", "_d   b_   "];
const RATS: [&str; 5] = ["          ", " ,~~.  ,~~.", "(o  )>(o  )>", " `--'  `--'", "  ~~    ~~ "];
const IMP: [&str; 5] = ["  \\_/    ", " (o o)~  ", " /|^|\\   ", "  / \\    ", " ^   ^   "];
const BEETLE: [&str; 5] = ["   \\  /   ", "  .-==-.  ", " ( o  o ) ", " /'-..-'\\ ", "  /    \\  "];
const WOLF: [&str; 5] = ["  /\\_/\\    ", " ( o o )__ ", "  > ^ <   )~", "   || ||   ", "   '' ''   "];
const KNIGHT: [&str; 5] = ["   _[]_   ", "  [ ++ ]  ", " /|====|\\ ", "  | || |  ", "  _|  |_  "];
const KING: [&str; 5] = ["  \\^^^^/  ", "  ( OO )  ", "_/|====|\\_", "  |/  \\|  ", " _/    \\_ "];

#[derive(Clone, Copy, PartialEq)]
enum Kind { Fight, Event, Shop, Rest, Boss }

#[derive(Clone, Copy, PartialEq)]
enum Trait { Fast, Armoured, Thorns }

impl Trait {
    fn text(self) -> &'static str {
        match self {
            Trait::Fast => "Fast: it strikes first",
            Trait::Armoured => "Armoured: your first hit does nothing",
            Trait::Thorns => "Thorns: you take 1 for every hit you land",
        }
    }
}

#[derive(Clone)]
struct Enemy { name: &'static str, hp: i32, atk: i32, def: i32, traits: Vec<Trait>, sprite: [&'static str; 5] }

#[derive(Clone, Copy, PartialEq)]
enum Cons { Whetstone, Tonic, Smoke }

impl Cons {
    fn short(self) -> &'static str {
        match self { Cons::Whetstone => "Whetstone +3 ATK", Cons::Tonic => "Tonic +3 DEF", Cons::Smoke => "Smoke bomb: ignore a Trait" }
    }
}

struct Item { name: &'static str, atk: i32, def: i32, plus: bool }

struct Habit { name: String, schedule: &'static str, streak: u32, lost: u32, week: [u8; 7] }
struct Quest { name: String, size: char, due: Option<&'static str>, matured: bool, done: bool }

#[derive(Clone)]
struct Hit { hero: bool, dmg: i32, hero_hp: i32, foe_hp: i32, note: &'static str }

struct Fight { enemy: Enemy, boss: bool, help: Vec<bool>, log: Vec<Hit>, won: bool }

#[derive(Clone, Copy, PartialEq)]
enum Mode { Tabs, Choose, Fight, FightAnim, Scout, Reward, Shop, Rest, Event, Closed, Wizard }

struct Wizard { step: usize, name: String, quest: bool, choice: usize }

struct App {
    tab: usize,
    mode: Mode,
    cursor: usize,
    habits: Vec<Habit>,
    quests: Vec<Quest>,
    today_cursor: usize,
    hab_cursor: usize,
    day_cursor: usize,
    gold: i32,
    steps: u32,
    hp: i32,
    max_hp: i32,
    atk: i32,
    def: i32,
    items: Vec<Item>,
    cons: Vec<Cons>,
    freezes: u32,
    essence: i32,
    unlocked: Vec<bool>,
    charges: u32,
    layers: Vec<Vec<Kind>>,
    at: usize,
    path: Vec<usize>,
    fight: Option<Fight>,
    anim: Instant,
    opened: Instant,
    wizard: Wizard,
    msg: String,
    closed_won: bool,
}

impl App {
    fn new() -> Self {
        let h = |name: &str, schedule, streak, lost, week| Habit { name: name.into(), schedule, streak, lost, week };
        App {
            tab: 0,
            mode: Mode::Tabs,
            cursor: 0,
            habits: vec![
                h("Read 20 min", "every day", 12, 0, [1, 1, 1, 2, 1, 1, 4]),
                h("Gym", "Mon Wed Fri", 5, 0, [3, 1, 3, 3, 1, 3, 4]),
                h("Meditate", "3 times a week", 2, 0, [3, 3, 1, 3, 3, 0, 4]),
                h("Floss", "every day", 1, 9, [1, 1, 1, 0, 1, 1, 4]),
                h("Call mum", "Sun", 7, 0, [3, 3, 1, 3, 3, 3, 3]),
            ],
            quests: vec![
                Quest { name: "Fix bike tyre".into(), size: 'M', due: Some("Fri"), matured: true, done: false },
                Quest { name: "Book dentist".into(), size: 'L', due: None, matured: true, done: false },
                Quest { name: "Clean inbox".into(), size: 'S', due: None, matured: false, done: false },
            ],
            today_cursor: 0,
            hab_cursor: 0,
            day_cursor: 6,
            gold: 34,
            steps: 2,
            hp: 31,
            max_hp: 40,
            atk: 6,
            def: 2,
            items: vec![Item { name: "Rusty blade", atk: 2, def: 0, plus: false }],
            cons: vec![Cons::Whetstone, Cons::Smoke],
            freezes: 1,
            essence: 120,
            unlocked: vec![true, false, false, false, false],
            charges: 2,
            layers: vec![
                vec![Kind::Fight, Kind::Event],
                vec![Kind::Fight, Kind::Shop, Kind::Rest],
                vec![Kind::Fight, Kind::Event, Kind::Shop],
                vec![Kind::Fight, Kind::Rest],
                vec![Kind::Event, Kind::Fight, Kind::Shop],
                vec![Kind::Fight, Kind::Rest, Kind::Fight],
            ],
            at: 2,
            path: vec![1, 0],
            fight: None,
            anim: Instant::now(),
            opened: Instant::now(),
            wizard: Wizard { step: 0, name: String::new(), quest: false, choice: 0 },
            msg: String::new(),
            closed_won: false,
        }
    }

    fn atk_total(&self) -> i32 { self.atk + self.items.iter().map(|i| i.atk + i.plus as i32).sum::<i32>() }
    fn def_total(&self) -> i32 { self.def + self.items.iter().map(|i| i.def).sum::<i32>() }

    fn due_today(&self) -> Vec<usize> { (0..self.habits.len()).filter(|&i| self.habits[i].week[6] != 3).collect() }
    fn today_rows(&self) -> usize { self.due_today().len() + self.quests.len() + 1 }

    fn next_kinds(&self) -> Vec<Kind> { self.layers.get(self.at).cloned().unwrap_or(vec![Kind::Boss]) }

    fn enemy_for(&self, kind: Kind, layer: usize) -> Enemy {
        if kind == Kind::Boss {
            return Enemy { name: "Hollow King", hp: 50, atk: 9, def: 3, traits: vec![Trait::Fast, Trait::Armoured], sprite: KING };
        }
        let (name, hp, atk, def, t, sprite) = [
            ("Rat swarm", 12, 4, 0, Trait::Fast, RATS),
            ("Mud imp", 14, 5, 1, Trait::Thorns, IMP),
            ("Bog goblin", 16, 6, 1, Trait::Fast, GOBLIN),
            ("Shell beetle", 20, 5, 3, Trait::Armoured, BEETLE),
            ("Briar wolf", 22, 7, 2, Trait::Thorns, WOLF),
            ("Grave knight", 26, 8, 3, Trait::Armoured, KNIGHT),
        ][layer];
        Enemy { name, hp, atk, def, traits: vec![t], sprite }
    }

    fn preview(&self, e: &Enemy, help: &[bool]) -> (bool, Vec<Hit>) {
        let (mut atk, mut def, mut e) = (self.atk_total(), self.def_total(), e.clone());
        for (i, c) in self.cons.iter().enumerate() {
            if !help.get(i).copied().unwrap_or(false) { continue; }
            match c {
                Cons::Whetstone => atk += 3,
                Cons::Tonic => def += 3,
                Cons::Smoke => { if !e.traits.is_empty() { e.traits.remove(0); } }
            }
        }
        simulate(self.hp, atk, def, &e)
    }

    fn toggle_habit_day(&mut self, i: usize, d: usize) {
        let h = &mut self.habits[i];
        let g = habit_gold(h.streak);
        match h.week[d] {
            4 | 0 => { h.week[d] = 1; h.streak += 1; self.gold += g; self.msg = format!("{} done  +{} Gold", h.name, g); }
            1 => { h.week[d] = if d == 6 { 4 } else { 0 }; h.streak = h.streak.saturating_sub(1); self.gold -= habit_gold(h.streak); self.msg = format!("{} unmarked", h.name); }
            _ => self.msg = format!("{} isn't due that day", h.name),
        }
    }

    fn toggle_quest(&mut self, j: usize) {
        let q = &mut self.quests[j];
        let g = quest_gold(q);
        q.done = !q.done;
        self.gold += if q.done { g } else { -g };
        self.msg = if q.done && !q.matured { format!("{} done  (new today, no Gold)", q.name) } else if q.done { format!("{} done  +{} Gold", q.name, g) } else { format!("{} reopened", q.name) };
    }

    fn buy_freeze(&mut self) {
        self.msg = if self.freezes >= FREEZE_CAP { "You already hold the most Freezes".into() }
        else if self.gold < FREEZE_COST { format!("A Freeze costs {FREEZE_COST} Gold") }
        else { self.gold -= FREEZE_COST; self.freezes += 1; format!("Bought a Freeze  ({}/{FREEZE_CAP})", self.freezes) };
    }

    fn enter_node(&mut self) {
        let kinds = self.next_kinds();
        let kind = kinds[self.cursor];
        let layer = self.at;
        self.steps -= 1;
        self.path.push(self.cursor);
        self.at += 1;
        self.cursor = 0;
        self.mode = match kind {
            Kind::Fight | Kind::Boss => {
                self.fight = Some(Fight { enemy: self.enemy_for(kind, layer), boss: kind == Kind::Boss, help: vec![false; self.cons.len() + 1], log: vec![], won: false });
                Mode::Fight
            }
            Kind::Shop => Mode::Shop,
            Kind::Rest => Mode::Rest,
            Kind::Event => Mode::Event,
        };
    }

    fn start_fight(&mut self) {
        let f = self.fight.as_ref().unwrap();
        if *f.help.last().unwrap() {
            self.charges -= 1;
            self.mode = Mode::Scout;
            self.anim = Instant::now();
            return;
        }
        let (won, log) = self.preview(&f.enemy, &f.help);
        let f = self.fight.as_mut().unwrap();
        f.won = won;
        f.log = log;
        self.mode = Mode::FightAnim;
        self.anim = Instant::now();
    }

    fn finish_fight(&mut self) {
        let f = self.fight.take().unwrap();
        self.hp = f.log.last().map_or(self.hp, |h| h.hero_hp).max(0);
        let mut i = 0;
        self.cons.retain(|_| { let keep = !f.help[i]; i += 1; keep });
        if !f.won { self.close(false); } else if f.boss { self.close(true); } else { self.mode = Mode::Reward; self.cursor = 0; }
    }

    fn close(&mut self, won: bool) {
        self.closed_won = won;
        self.essence += 10 * self.at as i32 + if won { 50 } else { 0 } + self.gold / 5;
        self.mode = Mode::Closed;
    }

    fn home(&mut self, msg: String) { self.mode = Mode::Tabs; self.tab = 0; self.msg = msg; }

    fn anim_index(&self) -> usize { (self.anim.elapsed().as_millis() / HIT_MS) as usize }

    fn rest_options(&self) -> Vec<String> {
        let mut v = vec![format!("Heal|+{} HP", 15.min(self.max_hp - self.hp))];
        for it in self.items.iter().filter(|i| !i.plus) { v.push(format!("Upgrade|{} -> {}+", it.name, it.name)); }
        v
    }
}

const SHOP: [(&str, i32, &str); 5] = [
    ("Iron sword", 20, "+3 ATK for the whole Run"),
    ("Leather vest", 18, "+2 DEF for the whole Run"),
    ("Whetstone", 8, "+3 ATK for one fight"),
    ("Ironbark tonic", 8, "+3 DEF for one fight"),
    ("Smoke bomb", 10, "ignore one Trait in a fight"),
];
const REWARDS: [(&str, &str, &str); 3] = [
    ("Item", "Spiked buckler", "+1 ATK  +1 DEF"),
    ("Consumable", "Ironbark tonic", "+3 DEF one fight"),
    ("Freeze", "Protects a Streak", "for one missed day"),
];
const EVENT: [(&str, &str); 3] = [
    ("Offer 10 Gold", "get Shrine charm, +1 ATK"),
    ("Drink from the font", "+10 HP"),
    ("Smash the shrine", "+12 Gold, -6 HP"),
];
const UNLOCKS: [(&str, i32, &str); 5] = [
    ("Treasure Node", 0, "a new Node type: free loot, no fight"),
    ("Thornmail", 80, "a new Item in the shop pool"),
    ("The Wandering Smith", 120, "a new event"),
    ("Moss: scarf", 60, "a Pet cosmetic"),
    ("Bestiary", 40, "codex entries for every enemy seen"),
];

fn habit_gold(streak: u32) -> i32 { (5.0 * (1.0 + streak.min(20) as f32 / 10.0)).round() as i32 }
fn quest_gold(q: &Quest) -> i32 {
    if !q.matured { return 0; }
    (match q.size { 'S' => 6, 'M' => 14, _ => 25 }) + if q.due.is_some() { 3 } else { 0 }
}

fn simulate(hp: i32, atk: i32, def: i32, e: &Enemy) -> (bool, Vec<Hit>) {
    let has = |t| e.traits.contains(&t);
    let (mut h, mut eh) = (hp, e.hp);
    let mut hero_turn = !has(Trait::Fast);
    let mut armour = has(Trait::Armoured);
    let mut log = vec![];
    while h > 0 && eh > 0 && log.len() < 200 {
        if hero_turn {
            let (dmg, note) = if armour { armour = false; (0, "blocked by its armour") } else { ((atk - e.def).max(1), "") };
            eh -= dmg;
            let note = if has(Trait::Thorns) { h -= 1; "thorns prick you for 1" } else { note };
            log.push(Hit { hero: true, dmg, hero_hp: h, foe_hp: eh.max(0), note });
        } else {
            let dmg = (e.atk - def).max(1);
            h -= dmg;
            log.push(Hit { hero: false, dmg, hero_hp: h.max(0), foe_hp: eh, note: "" });
        }
        hero_turn = !hero_turn;
    }
    (eh <= 0 && h > 0, log)
}

fn preview_line(won: bool, log: &[Hit], hp: i32) -> Line<'static> {
    let rounds = log.iter().filter(|h| h.hero).count();
    let end = log.last().map_or(hp, |h| h.hero_hp);
    if won { Line::from(format!(" You win in {rounds} rounds and end at HP {end}. ").black().on_green().bold()) }
    else { Line::from(format!(" You lose on round {rounds}. The Run would end. ").white().on_red().bold()) }
}

// ---------- drawing helpers ----------

fn sprite(rows: &[&'static str], style: Style, pad: usize) -> Vec<Line<'static>> {
    rows.iter().map(|r| Line::from(Span::styled(format!("{}{}", " ".repeat(pad), r), style))).collect()
}

fn bar(v: i32, max: i32, w: usize, c: Color) -> Span<'static> {
    let f = ((v.max(0) as f32 / max as f32) * w as f32).round() as usize;
    Span::styled(format!("{}{}", "█".repeat(f.min(w)), "░".repeat(w - f.min(w))), Style::new().fg(c))
}

fn column(area: Rect, w: u16) -> Rect {
    let w = w.min(area.width);
    Rect::new(area.x + (area.width - w) / 2, area.y, w, area.height)
}

fn popup(area: Rect, w: u16, h: u16) -> Rect {
    let (w, h) = (w.min(area.width), h.min(area.height));
    Rect::new(area.x + (area.width - w) / 2, area.y + (area.height - h) / 2, w, h)
}

fn card(f: &mut Frame, area: Rect, title: &str, color: Color, body: Vec<Line<'static>>, selected: bool) {
    let block = if selected {
        Block::bordered().border_type(BorderType::Thick).border_style(Style::new().fg(ACCENT))
    } else {
        Block::bordered().border_type(BorderType::Rounded).border_style(Style::new().fg(Color::DarkGray))
    };
    let mut lines = vec![Line::from(Span::styled(title.to_string(), Style::new().fg(color).bold())), Line::from("")];
    lines.extend(body);
    f.render_widget(Paragraph::new(lines).alignment(Alignment::Center).block(block), area);
}

fn cards_row(area: Rect, n: usize, w: u16, h: u16) -> Vec<Rect> {
    let row = Rect::new(area.x, area.y, area.width, h.min(area.height));
    Layout::horizontal(vec![Constraint::Length(w); n]).flex(Flex::Center).spacing(2).split(row).to_vec()
}

fn kind_info(k: Kind) -> (&'static str, Color) {
    match k {
        Kind::Fight => ("Fight", Color::Red),
        Kind::Event => ("Event", Color::Yellow),
        Kind::Shop => ("Shop", Color::Green),
        Kind::Rest => ("Rest", Color::Blue),
        Kind::Boss => ("Boss", Color::Magenta),
    }
}

fn pet_frame(app: &App, t: u128) -> (&'static [&'static str; 3], bool) {
    if app.opened.elapsed().as_millis() < 2600 {
        return if (t / 300) % 2 == 0 { (&PET_HAPPY, true) } else { (&PET_IDLE, false) };
    }
    if (t / 400) % 9 == 0 { (&PET_BLINK, false) } else { (&PET_IDLE, false) }
}

fn pet_xp(app: &App) -> i32 {
    let t = app.opened.elapsed().as_millis().min(2400) as i32;
    310 + 30 * t / 2400
}

fn hints(f: &mut Frame, area: Rect, pairs: &[(&str, &str)]) {
    let mut s = vec![];
    for (k, d) in pairs { s.push(format!(" {k} ").black().on_dark_gray()); s.push(format!(" {d}   ").dark_gray()); }
    f.render_widget(Paragraph::new(Line::from(s)).alignment(Alignment::Center), area);
}

fn progress(app: &App) -> Line<'static> {
    let mut s = vec!["Week 47 · Wednesday   ".dark_gray()];
    for i in 0..app.layers.len() { s.push(if i < app.at { "●".green() } else { "○".dark_gray() }); s.push(" ".into()); }
    s.push("── ".dark_gray());
    s.push("Hollow King".magenta().bold());
    s.push(format!("      {} Step{} left", app.steps, if app.steps == 1 { "" } else { "s" }).dark_gray());
    Line::from(s)
}

// ---------- screens ----------

fn draw(f: &mut Frame, app: &App) {
    let [top, gap, body, msg, keys] = Layout::vertical([Constraint::Length(2), Constraint::Length(1), Constraint::Min(0), Constraint::Length(1), Constraint::Length(1)]).areas(f.area());
    let _ = gap;
    if matches!(app.mode, Mode::Tabs | Mode::Wizard) { draw_tabs(f, app, top) } else { f.render_widget(Paragraph::new(vec![progress(app), Line::from("")]).alignment(Alignment::Center), top) }
    let body = column(body, 96);
    match app.mode {
        Mode::Tabs | Mode::Wizard => match app.tab {
            0 => draw_today(f, app, body),
            1 => draw_habits(f, app, body),
            2 => draw_run(f, app, body),
            3 => draw_pet(f, app, body),
            _ => draw_unlocks(f, app, body),
        },
        Mode::Choose => draw_choose(f, app, body),
        Mode::Fight | Mode::FightAnim | Mode::Scout => draw_fight(f, app, body),
        Mode::Reward => draw_reward(f, app, body),
        Mode::Shop => draw_shop(f, app, body),
        Mode::Rest => draw_rest(f, app, body),
        Mode::Event => draw_event(f, app, body),
        Mode::Closed => draw_closed(f, app, body),
    }
    if app.mode == Mode::Wizard { draw_wizard(f, app, body) }
    f.render_widget(Paragraph::new(Line::from(app.msg.clone().yellow())).alignment(Alignment::Center), msg);
    let k: &[(&str, &str)] = match app.mode {
        Mode::Wizard => &[("enter", "next"), ("esc", "cancel")],
        Mode::Tabs => match app.tab {
            0 => &[("tab", "sections"), ("up/down", "move"), ("space", "done"), ("p", "play"), ("a", "add"), ("q", "quit")],
            1 => &[("up/down", "move"), ("left/right", "day"), ("space", "mark"), ("r", "repair"), ("f", "buy Freeze"), ("a", "add")],
            2 => &[("tab", "sections"), ("p", "play"), ("q", "quit")],
            4 => &[("up/down", "move"), ("enter", "unlock")],
            _ => &[("tab", "sections"), ("q", "quit")],
        },
        Mode::Choose => &[("left/right", "choose"), ("enter", "go"), ("esc", "back home")],
        Mode::Fight => &[("1-3", "use help"), ("enter", "fight")],
        Mode::FightAnim if app.anim_index() >= app.fight.as_ref().map_or(0, |f| f.log.len()) => &[("enter", "continue")],
        Mode::FightAnim | Mode::Scout => &[("any key", "skip")],
        Mode::Reward | Mode::Rest => &[("left/right", "choose"), ("enter", "take")],
        Mode::Shop => &[("up/down", "move"), ("enter", "buy"), ("esc", "leave")],
        Mode::Event => &[("up/down", "move"), ("enter", "choose")],
        Mode::Closed => &[("R", "start a new prototype run"), ("q", "quit")],
    };
    hints(f, keys, k);
}

fn draw_tabs(f: &mut Frame, app: &App, area: Rect) {
    let [l, r] = Layout::horizontal([Constraint::Min(0), Constraint::Length(30)]).areas(area);
    let mut top = vec![Span::raw(" ")];
    let mut under = vec![Span::raw(" ")];
    for (i, t) in TABS.iter().enumerate() {
        let label = format!(" {t} ");
        let w = label.chars().count();
        if i == app.tab {
            top.push(Span::styled(label, Style::new().fg(ACCENT).bold()));
            under.push(Span::styled("━".repeat(w), Style::new().fg(ACCENT)));
        } else {
            top.push(Span::styled(label, Style::new().fg(Color::Gray)));
            under.push(Span::styled("─".repeat(w), Style::new().fg(Color::DarkGray)));
        }
        top.push("  ".into());
        under.push("──".dark_gray());
    }
    f.render_widget(Paragraph::new(vec![Line::from(top), Line::from(under)]), l);
    f.render_widget(Paragraph::new(Line::from(vec![
        format!("{} Gold", app.gold).yellow().bold(), "   ".into(), format!("HP {}/{} ", app.hp, app.max_hp).red().bold(),
    ])).alignment(Alignment::Right), r);
}

fn draw_today(f: &mut Frame, app: &App, area: Rect) {
    let area = column(area, 60);
    let [pet, _, list] = Layout::vertical([Constraint::Length(4), Constraint::Length(1), Constraint::Min(0)]).areas(area);
    let t = app.opened.elapsed().as_millis();
    let (frame, hop) = pet_frame(app, t);
    let [ps, pi] = Layout::horizontal([Constraint::Length(10), Constraint::Min(0)]).areas(pet);
    let mut art = sprite(frame, Style::new().fg(Color::Green), 0);
    if !hop { art.insert(0, Line::from("")); }
    f.render_widget(Paragraph::new(art), ps);
    let xp = pet_xp(app);
    let mut info = vec![
        Line::from(""),
        Line::from(vec!["Moss".green().bold(), "  ·  Sprout".dark_gray()]),
        Line::from(vec![bar(xp, 500, 16, Color::Green), format!("  {xp}/500 XP").dark_gray()]),
    ];
    if app.opened.elapsed().as_millis() < 4000 { info.push(Line::from("+30 XP from 3 Pi tasks".green().bold())); }
    f.render_widget(Paragraph::new(info), pi);

    let mut lines = vec![Line::from("Today".bold()), Line::from("")];
    let due = app.due_today();
    let mut row = 0;
    let sel = |l: Line<'static>, r: usize| if app.today_cursor == r { l.patch_style(Style::new().bg(Color::Rgb(40, 44, 52))).patch_style(Style::new().add_modifier(Modifier::BOLD)) } else { l };
    for &i in &due {
        let h = &app.habits[i];
        let done = h.week[6] == 1;
        let l = Line::from(vec![
            if app.today_cursor == row { " > ".fg(ACCENT) } else { "   ".into() },
            if done { "✓ ".green().bold() } else { "○ ".dark_gray() },
            Span::styled(format!("{:<24}", h.name), if done { Style::new().fg(Color::DarkGray) } else { Style::new() }),
            if done { "".into() } else { format!("+{} Gold", habit_gold(h.streak)).yellow() },
        ]);
        lines.push(sel(l, row));
        row += 1;
    }
    for q in &app.quests {
        let l = Line::from(vec![
            if app.today_cursor == row { " > ".fg(ACCENT) } else { "   ".into() },
            if q.done { "✓ ".green().bold() } else { "○ ".dark_gray() },
            Span::styled(format!("{:<24}", q.name), if q.done { Style::new().fg(Color::DarkGray) } else { Style::new() }),
            if q.done { "".into() } else if q.matured { format!("+{} Gold", quest_gold(q)).yellow() } else { "ready tomorrow".dark_gray() },
        ]);
        lines.push(sel(l, row));
        row += 1;
    }
    lines.push(Line::from(""));
    lines.push(Line::from(""));
    let play_sel = app.today_cursor == row;
    let play = if app.steps == 0 { "   No Steps left. Come back tomorrow.".dark_gray().into() } else {
        Line::from(vec![
            if play_sel { " > ".fg(ACCENT) } else { "   ".into() },
            Span::styled("  Play today's Node  ", if play_sel { Style::new().black().bg(ACCENT).bold() } else { Style::new().fg(ACCENT).bold().add_modifier(Modifier::REVERSED) }),
            format!("   {} Steps", app.steps).dark_gray(),
        ])
    };
    lines.push(play);
    f.render_widget(Paragraph::new(lines), list);
}

fn draw_habits(f: &mut Frame, app: &App, area: Rect) {
    let days = ["T", "F", "S", "S", "M", "T", "W"];
    let mut lines = vec![
        Line::from(vec!["Habits".bold(), format!("{:>66}", "").into(), format!("Freezes {}/{}", app.freezes, FREEZE_CAP).cyan().bold()]),
        Line::from(""),
        Line::from(vec![format!("   {:<16}{:<17}{:<11}", "", "", "").into(), days.iter().enumerate().map(|(i, d)| if i == 6 { format!("{d} ") } else { format!("{d} ") }).collect::<String>().dark_gray()]),
    ];
    for (i, h) in app.habits.iter().enumerate() {
        let selected = app.hab_cursor == i;
        let mut s = vec![
            if selected { " > ".fg(ACCENT) } else { "   ".into() },
            Span::styled(format!("{:<16}", h.name), if selected { Style::new().bold() } else { Style::new() }),
            format!("{:<17}", h.schedule).dark_gray(),
            format!("{:<11}", format!("streak {}", h.streak)).yellow(),
        ];
        for (d, v) in h.week.iter().enumerate() {
            let g = match v { 1 => "●".green(), 2 => "*".cyan(), 0 => "○".red(), 4 => "◌".white(), _ => "·".dark_gray() };
            let g = if selected && d == app.day_cursor { g.add_modifier(Modifier::REVERSED) } else { g };
            s.push(g);
            s.push(" ".into());
        }
        if h.lost > 0 && h.week.contains(&0) { s.push("  r: repair for 1 Freeze".cyan()); }
        lines.push(Line::from(s));
    }
    lines.push(Line::from(""));
    lines.push(Line::from("Quests".bold()));
    lines.push(Line::from(""));
    for q in &app.quests {
        lines.push(Line::from(vec![
            "   ".into(),
            Span::styled(format!("{:<16}", q.name), if q.done { Style::new().fg(Color::DarkGray).add_modifier(Modifier::CROSSED_OUT) } else { Style::new() }),
            format!("size {}", q.size).magenta(),
            q.due.map(|d| format!("  ·  due {d}")).unwrap_or_default().dark_gray(),
        ]));
    }
    lines.push(Line::from(""));
    lines.push(Line::from("   ● done   * covered by a Freeze   ○ missed   ◌ today   · not due".dark_gray()));
    f.render_widget(Paragraph::new(lines), area);
}

fn draw_run(f: &mut Frame, app: &App, area: Rect) {
    let [m, side] = Layout::horizontal([Constraint::Min(0), Constraint::Length(44)]).areas(area);
    let mut lines = vec![Line::from(vec!["  Boss  ".magenta().bold(), "Hollow King".magenta().bold()]), Line::from("")];
    for r in (0..app.layers.len()).rev() {
        let mut s = vec![format!("  {}  ", r + 1).dark_gray()];
        for (j, k) in app.layers[r].iter().enumerate() {
            let (name, c) = kind_info(*k);
            let st = if r < app.at { if app.path[r] == j { Style::new().fg(Color::Green).bold() } else { Style::new().fg(Color::DarkGray) } } else { Style::new().fg(c) };
            s.push(Span::styled(format!("{:<9}", name), st));
        }
        if r == app.at { s.push("<- next".fg(ACCENT)); }
        lines.push(Line::from(s));
    }
    lines.push(Line::from("     camp".dark_gray()));
    lines.push(Line::from(""));
    lines.push(Line::from("  green = the way you came".dark_gray()));
    f.render_widget(Paragraph::new(lines), m);

    let boss = app.enemy_for(Kind::Boss, 0);
    let mut s = sprite(&boss.sprite, Style::new().fg(Color::Magenta), 2);
    s.push(Line::from(""));
    s.push(Line::from(format!("HP {}  ATK {}  DEF {}", boss.hp, boss.atk, boss.def)));
    for t in &boss.traits { s.push(Line::from(t.text().yellow())); }
    s.push(Line::from(""));
    s.push(Line::from("Hero".bold()));
    s.push(Line::from(vec![bar(app.hp, app.max_hp, 12, Color::Red), format!(" {}/{}", app.hp, app.max_hp).into()]));
    s.push(Line::from(format!("ATK {}  DEF {}", app.atk_total(), app.def_total())));
    for it in &app.items { s.push(Line::from(format!("- {}{}", it.name, if it.plus { "+" } else { "" }))); }
    for c in &app.cons { s.push(Line::from(format!("- {}", c.short())).dark_gray()); }
    f.render_widget(Paragraph::new(s), side);
}

fn draw_pet(f: &mut Frame, app: &App, area: Rect) {
    let area = column(area, 60);
    let t = app.opened.elapsed().as_millis();
    let (frame, _) = pet_frame(app, t);
    let mut lines = vec![Line::from("")];
    lines.extend(sprite(frame, Style::new().fg(Color::Green), 0));
    lines.push(Line::from(""));
    lines.push(Line::from(vec!["Moss".green().bold(), "  ·  Stage 2 of 5, Sprout".dark_gray()]));
    lines.push(Line::from(vec![bar(pet_xp(app), 500, 24, Color::Green), format!("  {}/500 XP to Stage 3", pet_xp(app)).dark_gray()]));
    lines.push(Line::from(""));
    lines.push(Line::from("Abilities".bold()));
    lines.push(Line::from(vec!["  Scout    ".cyan().bold(), "walk ahead and skip a fight".into(), format!("   {}/2 left this Run", app.charges).dark_gray()]));
    lines.push(Line::from(vec!["  Burrow   ".cyan(), "turn the next Node into a rest".into()]));
    lines.push(Line::from("  Haggle   reroll a shop             at Stage 3".dark_gray()));
    lines.push(Line::from("  ???                                at Stage 4".dark_gray()));
    lines.push(Line::from("  ???                                at Stage 5".dark_gray()));
    lines.push(Line::from(""));
    lines.push(Line::from("Moss grows when pi finishes work for you. It never gets hungry.".dark_gray()));
    f.render_widget(Paragraph::new(lines).alignment(Alignment::Center), area);
}

fn draw_unlocks(f: &mut Frame, app: &App, area: Rect) {
    let area = column(area, 70);
    let mut lines = vec![Line::from(vec!["Unlocks".bold(), format!("{:>46}", "").into(), format!("{} Essence", app.essence).magenta().bold()]), Line::from("")];
    for (i, (name, cost, what)) in UNLOCKS.iter().enumerate() {
        let sel = app.cursor == i;
        lines.push(Line::from(vec![
            if sel { " > ".fg(ACCENT) } else { "   ".into() },
            if app.unlocked[i] { "✓ ".green() } else { "  ".into() },
            Span::styled(format!("{:<22}", name), if sel { Style::new().bold() } else { Style::new() }),
            if app.unlocked[i] { "unlocked   ".green() } else { format!("{:<11}", format!("{cost} Essence")).magenta() },
            what.dark_gray(),
        ]));
    }
    f.render_widget(Paragraph::new(lines), area);
}

fn draw_choose(f: &mut Frame, app: &App, area: Rect) {
    let [_, title, _, row] = Layout::vertical([Constraint::Length(1), Constraint::Length(1), Constraint::Length(2), Constraint::Min(0)]).areas(area);
    f.render_widget(Paragraph::new("Which way?".bold()).alignment(Alignment::Center), title);
    let kinds = app.next_kinds();
    let rects = cards_row(row, kinds.len(), 26, 11);
    for (i, k) in kinds.iter().enumerate() {
        let (name, c) = kind_info(*k);
        let body = match k {
            Kind::Fight | Kind::Boss => {
                let e = app.enemy_for(*k, app.at);
                let (won, log) = app.preview(&e, &[]);
                let end = log.last().map_or(app.hp, |h| h.hero_hp);
                vec![
                    Line::from(e.name.bold()),
                    Line::from(e.traits.iter().map(|t| match t { Trait::Fast => "fast", Trait::Armoured => "armoured", Trait::Thorns => "thorny" }).collect::<Vec<_>>().join(", ").yellow()),
                    Line::from(""),
                    if won { Line::from(format!("you win, HP {} -> {}", app.hp, end).green()) } else { Line::from("you would lose".red().bold()) },
                    Line::from("then pick a reward".dark_gray()),
                ]
            }
            Kind::Event => vec![Line::from("something"), Line::from("unexpected"), Line::from(""), Line::from("a choice with".dark_gray()), Line::from("a known outcome".dark_gray())],
            Kind::Shop => vec![Line::from("spend your"), Line::from(format!("{} Gold", app.gold).yellow()), Line::from(""), Line::from("Items and".dark_gray()), Line::from("Consumables".dark_gray())],
            Kind::Rest => vec![Line::from("catch your breath"), Line::from(""), Line::from(format!("heal up to {} HP", 15.min(app.max_hp - app.hp)).green()), Line::from("or upgrade an Item".dark_gray())],
        };
        card(f, rects[i], name, c, body, app.cursor == i);
    }
}

fn draw_fight(f: &mut Frame, app: &App, area: Rect) {
    let fight = app.fight.as_ref();
    let enemy = fight.map(|f| f.enemy.clone()).unwrap_or_else(|| app.enemy_for(Kind::Fight, app.at.saturating_sub(1)));
    let [stage, _, stats, _, preview, _, helpers] = Layout::vertical([
        Constraint::Length(6), Constraint::Length(1), Constraint::Length(4), Constraint::Length(1), Constraint::Length(3), Constraint::Length(1), Constraint::Min(0),
    ]).areas(area);
    let [hero_a, pet_a, mid, foe_a] = Layout::horizontal([Constraint::Length(18), Constraint::Length(10), Constraint::Min(0), Constraint::Length(22)]).areas(stage);

    let idx = app.anim_index();
    let animating = app.mode == Mode::FightAnim;
    let log = fight.map(|f| f.log.clone()).unwrap_or_default();
    let shown = if animating { idx.min(log.len()) } else { 0 };
    let cur = if animating && shown > 0 && shown <= log.len() && idx < log.len() + 1 { log.get(shown - 1) } else { None };
    let hero_hp = if animating && shown > 0 { log[shown - 1].hero_hp } else { app.hp };
    let foe_hp = if animating && shown > 0 { log[shown - 1].foe_hp } else { enemy.hp };

    let (hero_rows, hero_style, hero_pad) = match cur {
        Some(h) if h.hero && idx < log.len() => (&HERO_ATK, Style::new().fg(Color::White).bold(), 2),
        Some(h) if !h.hero && idx < log.len() => (&HERO_HURT, Style::new().fg(Color::Red), 0),
        _ if hero_hp <= 0 => (&HERO_HURT, Style::new().fg(Color::DarkGray), 0),
        _ => (&HERO, Style::new().fg(Color::White), 1),
    };
    let mut hs = vec![Line::from("")];
    hs.extend(sprite(hero_rows, hero_style, hero_pad));
    f.render_widget(Paragraph::new(hs), hero_a);

    let scout_t = app.anim.elapsed().as_millis() as usize;
    let pet_pad = if app.mode == Mode::Scout { (scout_t / 60).min(40) } else { 0 };
    let pet_frame: &[&str; 3] = if app.mode == Mode::Scout || cur.is_some_and(|h| h.hero) { &PET_HAPPY } else { &PET_IDLE };
    let mut ps = vec![Line::from(""), Line::from(""), Line::from("")];
    if app.mode == Mode::Scout {
        let [_, wide] = Layout::horizontal([Constraint::Length(0), Constraint::Min(0)]).areas(Rect::new(pet_a.x, pet_a.y, pet_a.width + mid.width, pet_a.height));
        let mut v = vec![Line::from(""), Line::from(""), Line::from("")];
        v.extend(sprite(pet_frame, Style::new().fg(Color::Green), pet_pad));
        f.render_widget(Paragraph::new(v), wide);
    } else {
        ps.extend(sprite(pet_frame, Style::new().fg(Color::Green), 0));
        f.render_widget(Paragraph::new(ps), pet_a);
    }

    let foe_hurt = cur.is_some_and(|h| h.hero && h.dmg > 0) && idx < log.len();
    let foe_hits = cur.is_some_and(|h| !h.hero) && idx < log.len();
    let foe_style = if foe_hp <= 0 { Style::new().fg(Color::DarkGray) } else if foe_hurt { Style::new().fg(Color::Red).add_modifier(Modifier::BOLD) } else { Style::new().fg(Color::Yellow) };
    let mut es = vec![Line::from("")];
    let foe_rows: Vec<Line> = sprite(&enemy.sprite, foe_style, if foe_hits { 0 } else { 3 });
    es.extend(foe_rows);
    f.render_widget(Paragraph::new(es), foe_a);
    if app.mode != Mode::Scout {
        f.render_widget(Paragraph::new(vec![Line::from(""), Line::from(""), Line::from(""), Line::from("vs".dark_gray())]).alignment(Alignment::Center), mid);
    }

    let [hs_a, _, fs_a] = Layout::horizontal([Constraint::Length(30), Constraint::Min(0), Constraint::Length(40)]).areas(stats);
    f.render_widget(Paragraph::new(vec![
        Line::from("Hero".bold()),
        Line::from(vec![bar(hero_hp, app.max_hp, 14, Color::Red), format!(" HP {}/{}", hero_hp.max(0), app.max_hp).into()]),
        Line::from(format!("ATK {}   DEF {}", app.atk_total(), app.def_total()).dark_gray()),
    ]), hs_a);
    let mut fl = vec![
        Line::from(enemy.name.bold()),
        Line::from(vec![bar(foe_hp, enemy.hp, 14, Color::Yellow), format!(" HP {}/{}", foe_hp.max(0), enemy.hp).into()]),
        Line::from(format!("ATK {}   DEF {}", enemy.atk, enemy.def).dark_gray()),
    ];
    for t in &enemy.traits { fl.push(Line::from(t.text().yellow())); }
    f.render_widget(Paragraph::new(fl), fs_a);

    match app.mode {
        Mode::Fight => {
            let fi = fight.unwrap();
            let p = if *fi.help.last().unwrap() { Line::from(" Moss scouts a way around. No fight, no reward. ".black().on_cyan().bold()) } else {
                let (won, log) = app.preview(&fi.enemy, &fi.help);
                preview_line(won, &log, app.hp)
            };
            f.render_widget(Paragraph::new(p).alignment(Alignment::Center).block(Block::bordered().border_type(BorderType::Rounded).border_style(Style::new().fg(Color::DarkGray))), preview);
            let mut s = vec!["Help:  ".dark_gray()];
            for (i, c) in app.cons.iter().enumerate() {
                let on = fi.help[i];
                s.push(Span::styled(format!(" {} ", i + 1), Style::new().black().bg(if on { ACCENT } else { Color::DarkGray })));
                s.push(Span::styled(format!(" {}    ", c.short()), if on { Style::new().fg(ACCENT).bold() } else { Style::new() }));
            }
            let n = app.cons.len();
            let on = fi.help[n];
            s.push(Span::styled(format!(" {} ", n + 1), Style::new().black().bg(if on { ACCENT } else { Color::DarkGray })));
            s.push(Span::styled(format!(" Moss: Scout ({} left)", app.charges), if on { Style::new().fg(ACCENT).bold() } else if app.charges == 0 { Style::new().fg(Color::DarkGray) } else { Style::new().fg(Color::Green) }));
            f.render_widget(Paragraph::new(Line::from(s)).alignment(Alignment::Center), helpers);
        }
        Mode::FightAnim => {
            let done = idx >= log.len();
            let mut l: Vec<Line> = log[..shown].iter().rev().take(3).map(|h| {
                let who = if h.hero { format!("You hit {} for {}", enemy.name, h.dmg) } else { format!("{} hits you for {}", enemy.name, h.dmg) };
                let who = if h.note.is_empty() { who } else { format!("{who}  ({})", h.note) };
                Line::from(if h.hero { who.white() } else { who.red() })
            }).collect();
            l.reverse();
            if done {
                let won = fight.unwrap().won;
                l = vec![if won { Line::from(format!(" Victory! You end at HP {}. ", hero_hp).black().on_green().bold()) } else { Line::from(" The Hero falls. ".white().on_red().bold()) }, Line::from(""), Line::from("press enter".dark_gray())];
            }
            f.render_widget(Paragraph::new(l).alignment(Alignment::Center), Rect::new(preview.x, preview.y, preview.width, preview.height + helpers.height));
        }
        _ => {
            f.render_widget(Paragraph::new(Line::from("Moss sniffs out a hidden path around the fight...".green())).alignment(Alignment::Center), preview);
        }
    }
}

fn draw_reward(f: &mut Frame, app: &App, area: Rect) {
    let [_, title, _, row] = Layout::vertical([Constraint::Length(1), Constraint::Length(1), Constraint::Length(2), Constraint::Min(0)]).areas(area);
    f.render_widget(Paragraph::new("You won. Pick one reward.".bold()).alignment(Alignment::Center), title);
    for (i, r) in cards_row(row, 3, 26, 8).into_iter().enumerate() {
        let (t, name, what) = REWARDS[i];
        let c = [Color::Yellow, Color::Green, Color::Cyan][i];
        card(f, r, t, c, vec![Line::from(name.bold()), Line::from(what.dark_gray())], app.cursor == i);
    }
}

fn draw_shop(f: &mut Frame, app: &App, area: Rect) {
    let area = column(area, 70);
    let mut lines = vec![Line::from(""), Line::from(vec!["Shop".green().bold(), format!("{:>50}", "").into(), format!("{} Gold", app.gold).yellow().bold()]), Line::from("")];
    for (i, (name, price, what)) in SHOP.iter().enumerate() {
        let sel = app.cursor == i;
        lines.push(Line::from(vec![
            if sel { " > ".fg(ACCENT) } else { "   ".into() },
            Span::styled(format!("{:<18}", name), if sel { Style::new().bold() } else { Style::new() }),
            Span::styled(format!("{:>3} Gold   ", price), Style::new().fg(if app.gold >= *price { Color::Yellow } else { Color::DarkGray })),
            what.dark_gray(),
        ]));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(vec![if app.cursor == SHOP.len() { " > ".fg(ACCENT) } else { "   ".into() }, "Leave".into()]));
    lines.push(Line::from(""));
    lines.push(Line::from(format!("   Item slots {}/{}", app.items.len(), SLOTS).dark_gray()));
    f.render_widget(Paragraph::new(lines), area);
}

fn draw_rest(f: &mut Frame, app: &App, area: Rect) {
    let [_, title, _, row] = Layout::vertical([Constraint::Length(1), Constraint::Length(1), Constraint::Length(2), Constraint::Min(0)]).areas(area);
    f.render_widget(Paragraph::new(vec![Line::from("A quiet camp. Pick one.".bold()), Line::from(format!("HP {}/{}", app.hp, app.max_hp).dark_gray())]).alignment(Alignment::Center), title);
    let opts = app.rest_options();
    for (i, r) in cards_row(row, opts.len(), 28, 7).into_iter().enumerate() {
        let (t, what) = opts[i].split_once('|').unwrap();
        card(f, r, t, if i == 0 { Color::Green } else { Color::Yellow }, vec![Line::from(what.to_string())], app.cursor == i);
    }
}

fn draw_event(f: &mut Frame, app: &App, area: Rect) {
    let area = column(area, 64);
    let mut lines = vec![
        Line::from(""),
        Line::from("    .-^-.".yellow()),
        Line::from("   ( ~~~ )   A cracked shrine hums in the reeds.".yellow()),
        Line::from("    |___|    Something listens.".yellow()),
        Line::from(""),
    ];
    for (i, (o, out)) in EVENT.iter().enumerate() {
        let sel = app.cursor == i;
        lines.push(Line::from(vec![
            if sel { " > ".fg(ACCENT) } else { "   ".into() },
            Span::styled(format!("{:<22}", o), if sel { Style::new().bold() } else { Style::new() }),
            out.dark_gray(),
        ]));
    }
    f.render_widget(Paragraph::new(lines), area);
}

fn draw_closed(f: &mut Frame, app: &App, area: Rect) {
    let lines = vec![
        Line::from(""),
        Line::from(if app.closed_won { "You beat the Hollow King!".green().bold() } else { "The Hero fell. This week's Run is over.".red().bold() }),
        Line::from(""),
        Line::from(format!("You now have {} Essence to spend on Unlocks.", app.essence)),
        Line::from(""),
        Line::from("Your Streaks, Freezes and Moss are safe. A new Run starts Monday.".dark_gray()),
    ];
    f.render_widget(Paragraph::new(lines).alignment(Alignment::Center), area);
}

fn draw_wizard(f: &mut Frame, app: &App, area: Rect) {
    let w = &app.wizard;
    let pop = popup(area, 56, 12);
    f.render_widget(Clear, pop);
    let opts: Vec<&str> = match w.step {
        1 => vec!["It repeats  (a Habit)", "Just once  (a Quest)"],
        2 if !w.quest => vec!["Every day", "On certain days", "A few times a week"],
        2 => vec!["S  I resist it a little", "M  I resist it quite a bit", "L  I really don't want to"],
        _ => vec![],
    };
    let q = match w.step { 0 => "What do you want to do?", 1 => "Does it repeat?", _ if w.quest => "How much do you resist doing it?", _ => "How often?" };
    let mut lines = vec![Line::from(format!("Step {} of 3", w.step + 1).dark_gray()), Line::from(""), Line::from(q.bold()), Line::from("")];
    if w.step == 0 {
        lines.push(Line::from(vec!["  ".into(), w.name.clone().fg(ACCENT).bold(), "▏".fg(ACCENT)]));
    } else {
        for (i, o) in opts.iter().enumerate() {
            lines.push(Line::from(vec![if w.choice == i { " > ".fg(ACCENT) } else { "   ".into() }, Span::styled(o.to_string(), if w.choice == i { Style::new().bold() } else { Style::new() })]));
        }
    }
    f.render_widget(Paragraph::new(lines).block(Block::bordered().border_type(BorderType::Rounded).border_style(Style::new().fg(ACCENT)).title(" Add ")), pop);
}

// ---------- input ----------

fn key(app: &mut App, code: KeyCode) -> bool {
    if app.mode == Mode::Wizard { return key_wizard(app, code); }
    match code {
        KeyCode::Char('q') => return false,
        KeyCode::Char('R') => { *app = App::new(); return true; }
        _ => {}
    }
    let up = matches!(code, KeyCode::Up | KeyCode::Char('k'));
    let down = matches!(code, KeyCode::Down | KeyCode::Char('j'));
    let left = matches!(code, KeyCode::Left | KeyCode::Char('h'));
    let right = matches!(code, KeyCode::Right | KeyCode::Char('l'));
    let wrap = |c: usize, n: usize, d: i32| (c as i32 + d).rem_euclid(n.max(1) as i32) as usize;
    match app.mode {
        Mode::Tabs => {
            match code {
                KeyCode::Tab => { app.tab = (app.tab + 1) % TABS.len(); app.cursor = 0; return true; }
                KeyCode::BackTab => { app.tab = (app.tab + TABS.len() - 1) % TABS.len(); app.cursor = 0; return true; }
                KeyCode::Char(c @ '1'..='5') => { app.tab = c as usize - '1' as usize; app.cursor = 0; return true; }
                KeyCode::Char('a') if app.tab <= 1 => { app.wizard = Wizard { step: 0, name: String::new(), quest: false, choice: 0 }; app.mode = Mode::Wizard; return true; }
                KeyCode::Char('p') if app.tab != 1 => { play(app); return true; }
                _ => {}
            }
            match app.tab {
                0 => {
                    let n = app.today_rows();
                    if up { app.today_cursor = wrap(app.today_cursor, n, -1) }
                    if down { app.today_cursor = wrap(app.today_cursor, n, 1) }
                    if matches!(code, KeyCode::Char(' ') | KeyCode::Enter) {
                        let due = app.due_today();
                        let c = app.today_cursor;
                        if c < due.len() { app.toggle_habit_day(due[c], 6) }
                        else if c < due.len() + app.quests.len() { app.toggle_quest(c - due.len()) }
                        else { play(app) }
                    }
                }
                1 => {
                    if up { app.hab_cursor = wrap(app.hab_cursor, app.habits.len(), -1) }
                    if down { app.hab_cursor = wrap(app.hab_cursor, app.habits.len(), 1) }
                    if left { app.day_cursor = app.day_cursor.saturating_sub(1) }
                    if right { app.day_cursor = (app.day_cursor + 1).min(6) }
                    match code {
                        KeyCode::Char(' ') => app.toggle_habit_day(app.hab_cursor, app.day_cursor),
                        KeyCode::Char('f') => app.buy_freeze(),
                        KeyCode::Char('r') => {
                            let h = &mut app.habits[app.hab_cursor];
                            if h.lost == 0 || !h.week.contains(&0) { app.msg = "Nothing to repair here".into(); }
                            else if app.freezes == 0 { app.msg = "You need a Freeze to repair a Streak".into(); }
                            else {
                                let d = h.week.iter().position(|&v| v == 0).unwrap();
                                h.week[d] = 2;
                                h.streak += h.lost;
                                h.lost = 0;
                                app.freezes -= 1;
                                app.msg = format!("{} repaired: streak {}", h.name, h.streak);
                            }
                        }
                        _ => {}
                    }
                }
                4 => {
                    if up { app.cursor = wrap(app.cursor, UNLOCKS.len(), -1) }
                    if down { app.cursor = wrap(app.cursor, UNLOCKS.len(), 1) }
                    if code == KeyCode::Enter {
                        let (name, cost, _) = UNLOCKS[app.cursor];
                        if app.unlocked[app.cursor] { app.msg = "Already unlocked".into(); }
                        else if app.essence < cost { app.msg = format!("{name} needs {cost} Essence"); }
                        else { app.essence -= cost; app.unlocked[app.cursor] = true; app.msg = format!("Unlocked {name}"); }
                    }
                }
                _ => {}
            }
        }
        Mode::Choose => {
            let n = app.next_kinds().len();
            if left { app.cursor = wrap(app.cursor, n, -1) }
            if right { app.cursor = wrap(app.cursor, n, 1) }
            if code == KeyCode::Enter { app.enter_node() }
            if code == KeyCode::Esc { app.home(String::new()) }
        }
        Mode::Fight => {
            if let KeyCode::Char(c @ '1'..='9') = code {
                let i = c as usize - '1' as usize;
                let n = app.cons.len();
                let charges = app.charges;
                let f = app.fight.as_mut().unwrap();
                if i < f.help.len() {
                    if i == n && charges == 0 { app.msg = "Moss has no Scout charges left".into(); } else { f.help[i] = !f.help[i]; }
                }
            }
            if code == KeyCode::Enter { app.start_fight() }
        }
        Mode::FightAnim => {
            let n = app.fight.as_ref().unwrap().log.len();
            if app.anim_index() < n { app.anim = Instant::now() - Duration::from_millis((HIT_MS * n as u128) as u64) }
            else if code == KeyCode::Enter { app.finish_fight() }
        }
        Mode::Scout => { app.fight = None; app.home(format!("Moss found a way around. {} Scout left.", app.charges)) }
        Mode::Reward => {
            if left { app.cursor = wrap(app.cursor, 3, -1) }
            if right { app.cursor = wrap(app.cursor, 3, 1) }
            if code == KeyCode::Enter {
                match app.cursor {
                    0 if app.items.len() < SLOTS => app.items.push(Item { name: "Spiked buckler", atk: 1, def: 1, plus: false }),
                    1 => app.cons.push(Cons::Tonic),
                    2 if app.freezes < FREEZE_CAP => app.freezes += 1,
                    _ => { app.msg = "No room for that one".into(); return true; }
                }
                let msg = format!("You took the {}", REWARDS[app.cursor].1);
                app.home(msg);
            }
        }
        Mode::Shop => {
            if up { app.cursor = wrap(app.cursor, SHOP.len() + 1, -1) }
            if down { app.cursor = wrap(app.cursor, SHOP.len() + 1, 1) }
            if code == KeyCode::Esc || (code == KeyCode::Enter && app.cursor == SHOP.len()) { app.home("You leave the shop".into()); return true; }
            if code == KeyCode::Enter {
                let (name, price, _) = SHOP[app.cursor];
                if app.gold < price { app.msg = format!("Not enough Gold for the {name}"); }
                else if app.cursor < 2 && app.items.len() >= SLOTS { app.msg = "Your Item slots are full".into(); }
                else {
                    app.gold -= price;
                    match app.cursor {
                        0 => app.items.push(Item { name: "Iron sword", atk: 3, def: 0, plus: false }),
                        1 => app.items.push(Item { name: "Leather vest", atk: 0, def: 2, plus: false }),
                        2 => app.cons.push(Cons::Whetstone),
                        3 => app.cons.push(Cons::Tonic),
                        _ => app.cons.push(Cons::Smoke),
                    }
                    app.msg = format!("Bought the {name}");
                }
            }
        }
        Mode::Rest => {
            let n = app.rest_options().len();
            if left { app.cursor = wrap(app.cursor, n, -1) }
            if right { app.cursor = wrap(app.cursor, n, 1) }
            if code == KeyCode::Enter {
                if app.cursor == 0 {
                    let h = 15.min(app.max_hp - app.hp);
                    app.hp += h;
                    app.home(format!("You rest. +{h} HP"));
                } else {
                    let idx = (0..app.items.len()).filter(|&i| !app.items[i].plus).nth(app.cursor - 1).unwrap();
                    app.items[idx].plus = true;
                    let msg = format!("{} is now {}+", app.items[idx].name, app.items[idx].name);
                    app.home(msg);
                }
            }
        }
        Mode::Event => {
            if up { app.cursor = wrap(app.cursor, 3, -1) }
            if down { app.cursor = wrap(app.cursor, 3, 1) }
            if code == KeyCode::Enter {
                match app.cursor {
                    0 if app.gold >= 10 && app.items.len() < SLOTS => {
                        app.gold -= 10;
                        app.items.push(Item { name: "Shrine charm", atk: 1, def: 0, plus: false });
                        app.home("The shrine gives you a charm. +1 ATK".into());
                    }
                    0 => app.msg = "You need 10 Gold and a free Item slot".into(),
                    1 => { let h = 10.min(app.max_hp - app.hp); app.hp += h; app.home(format!("The water is cold and kind. +{h} HP")); }
                    _ => { app.gold += 12; app.hp = (app.hp - 6).max(1); app.home("Coins spill out. +12 Gold, -6 HP".into()); }
                }
            }
        }
        Mode::Closed | Mode::Wizard => {}
    }
    true
}

fn play(app: &mut App) {
    if app.steps == 0 { app.msg = "No Steps left. One more arrives tomorrow.".into(); return; }
    app.mode = Mode::Choose;
    app.cursor = 0;
    app.msg = String::new();
}

fn key_wizard(app: &mut App, code: KeyCode) -> bool {
    let w = &mut app.wizard;
    if code == KeyCode::Esc { app.mode = Mode::Tabs; return true; }
    match w.step {
        0 => match code {
            KeyCode::Char(c) => w.name.push(c),
            KeyCode::Backspace => { w.name.pop(); }
            KeyCode::Enter if !w.name.trim().is_empty() => { w.step = 1; w.choice = 0; }
            _ => {}
        },
        _ => {
            let n = if w.step == 1 { 2 } else { 3 };
            match code {
                KeyCode::Up | KeyCode::Char('k') => w.choice = (w.choice + n - 1) % n,
                KeyCode::Down | KeyCode::Char('j') => w.choice = (w.choice + 1) % n,
                KeyCode::Enter if w.step == 1 => { w.quest = w.choice == 1; w.step = 2; w.choice = 0; }
                KeyCode::Enter => {
                    let name = w.name.trim().to_string();
                    if w.quest {
                        app.quests.push(Quest { name: name.clone(), size: ['S', 'M', 'L'][w.choice], due: None, matured: false, done: false });
                    } else {
                        let schedule = ["every day", "Mon Wed Fri", "3 times a week"][w.choice];
                        app.habits.push(Habit { name: name.clone(), schedule, streak: 0, lost: 0, week: [3, 3, 3, 3, 3, 3, 4] });
                    }
                    app.mode = Mode::Tabs;
                    app.msg = format!("Added {name}");
                }
                _ => {}
            }
        }
    }
    true
}

fn run(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut app = App::new();
    loop {
        if app.mode == Mode::Scout && app.anim.elapsed().as_millis() > 1600 {
            app.fight = None;
            app.home(format!("Moss found a way around. {} Scout left.", app.charges));
        }
        terminal.draw(|f| draw(f, &app))?;
        if !event::poll(Duration::from_millis(50))? { continue; }
        let Event::Key(k) = event::read()? else { continue };
        if k.kind != KeyEventKind::Press { continue; }
        if !key(&mut app, k.code) { return Ok(()); }
    }
}

fn main() -> std::io::Result<()> {
    if let Ok(keys) = std::env::var("DUMP") {
        let mut app = App::new();
        app.opened = Instant::now() - Duration::from_secs(10);
        for part in keys.split(',').skip(1) {
            let code = match part { "enter" => KeyCode::Enter, "tab" => KeyCode::Tab, "esc" => KeyCode::Esc, "space" => KeyCode::Char(' '), "down" => KeyCode::Down, "right" => KeyCode::Right, "wait" => { app.anim -= Duration::from_secs(30); continue; } s => KeyCode::Char(s.chars().next().unwrap()) };
            key(&mut app, code);
        }
        let mut t = ratatui::Terminal::new(ratatui::backend::TestBackend::new(110, 30)).unwrap();
        t.draw(|f| draw(f, &app)).unwrap();
        let b = t.backend().buffer();
        for y in 0..b.area.height { println!("{}", (0..b.area.width).map(|x| b[(x, y)].symbol().to_string()).collect::<String>()); }
        return Ok(());
    }
    let mut terminal = ratatui::init();
    let r = run(&mut terminal);
    ratatui::restore();
    r
}
