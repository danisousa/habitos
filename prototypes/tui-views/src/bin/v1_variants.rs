// PROTOTYPE — throwaway. Three structurally different layouts of the Habitos TUI over the same
// in-memory mock state, switchable with [ and ]. Answers "TUI views and layout" (issue #25).
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Cell, Clear, Paragraph, Row, Table, Wrap},
};

const VARIANTS: [&str; 3] = ["A — lazygit panels", "B — guided flow", "C — k9s views"];
const FREEZE_COST: i32 = 15;
const FREEZE_CAP: u32 = 3;
const STEP_CAP: u32 = 3;
const SLOTS: usize = 4;

#[derive(Clone, Copy, PartialEq)]
enum Kind { Fight, Event, Shop, Rest, Boss }

#[derive(Clone, Copy, PartialEq)]
enum Trait { Fast, Armoured, Thorns }

impl Trait {
    fn label(self) -> &'static str {
        match self {
            Trait::Fast => "Fast (strikes first)",
            Trait::Armoured => "Armoured (ignores your first hit)",
            Trait::Thorns => "Thorns (you take 1 per hit you land)",
        }
    }
}

#[derive(Clone)]
struct Enemy { name: &'static str, hp: i32, atk: i32, def: i32, traits: Vec<Trait> }

#[derive(Clone, Copy, PartialEq)]
enum Cons { Whetstone, Tonic, Smoke }

impl Cons {
    fn label(self) -> &'static str {
        match self {
            Cons::Whetstone => "Whetstone (+3 ATK this fight)",
            Cons::Tonic => "Ironbark tonic (+3 DEF this fight)",
            Cons::Smoke => "Smoke bomb (ignore one Trait)",
        }
    }
}

#[derive(Clone)]
struct Item { name: &'static str, atk: i32, def: i32, plus: bool }

struct Habit { name: &'static str, schedule: &'static str, streak: u32, due: bool, done: bool, week: [u8; 6] }
struct Quest { name: &'static str, size: char, due: Option<&'static str>, matured: bool, done: bool }

enum Node {
    Fight { enemy: Enemy, toggles: Vec<bool>, cursor: usize, boss: bool },
    Reward { cursor: usize },
    Shop { cursor: usize },
    Rest { cursor: usize },
    Event { cursor: usize },
    Closed { won: bool, essence: i32 },
}

#[derive(Clone, Copy, PartialEq)]
enum BStage { CheckIn, Map, Node, Done }

#[derive(Clone, Copy, PartialEq)]
enum CView { Today, Run, Pet, Habits }

struct App {
    variant: usize,
    habits: Vec<Habit>,
    quests: Vec<Quest>,
    gold: i32,
    gold_today: i32,
    steps: u32,
    hp: i32,
    max_hp: i32,
    atk: i32,
    def: i32,
    items: Vec<Item>,
    cons: Vec<Cons>,
    freezes: u32,
    essence: i32,
    charges: u32,
    layers: Vec<Vec<Kind>>,
    at: usize,
    path: Vec<usize>,
    map_cursor: usize,
    node: Option<Node>,
    nodes_today: u32,
    today_cursor: usize,
    msg: String,
    a_focus: usize,
    b_stage: BStage,
    c_view: CView,
    c_cmd: Option<String>,
}

impl App {
    fn new(variant: usize) -> Self {
        let h = |name, schedule, streak, due, week| Habit { name, schedule, streak, due, done: false, week };
        App {
            variant,
            habits: vec![
                h("Read 20 min", "daily", 12, true, [1, 1, 1, 2, 1, 1]),
                h("Gym", "Mon Wed Fri", 5, true, [1, 3, 1, 3, 1, 3]),
                h("Meditate", "3 / week", 2, true, [3, 1, 3, 3, 0, 3]),
                h("Floss", "daily", 0, true, [1, 1, 0, 0, 1, 0]),
                h("Call mum", "Sun", 7, false, [3, 3, 3, 3, 1, 3]),
            ],
            quests: vec![
                Quest { name: "Fix bike tyre", size: 'M', due: Some("Fri"), matured: true, done: false },
                Quest { name: "Book dentist", size: 'L', due: None, matured: true, done: false },
                Quest { name: "Clean inbox", size: 'S', due: None, matured: false, done: false },
            ],
            gold: 34,
            gold_today: 0,
            steps: 2,
            hp: 31,
            max_hp: 40,
            atk: 6,
            def: 2,
            items: vec![Item { name: "Rusty blade", atk: 2, def: 0, plus: false }],
            cons: vec![Cons::Whetstone, Cons::Smoke],
            freezes: 1,
            essence: 120,
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
            map_cursor: 0,
            node: None,
            nodes_today: 0,
            today_cursor: 0,
            msg: "Wed · Run day 3/7 · 2 Steps banked".into(),
            a_focus: 0,
            b_stage: BStage::CheckIn,
            c_view: CView::Today,
            c_cmd: None,
        }
    }

    fn entries(&self) -> usize { self.habits.len() + self.quests.len() }

    fn atk_total(&self) -> i32 { self.atk + self.items.iter().map(|i| i.atk + i.plus as i32).sum::<i32>() }
    fn def_total(&self) -> i32 { self.def + self.items.iter().map(|i| i.def).sum::<i32>() }

    fn next_layer(&self) -> Option<&Vec<Kind>> { self.layers.get(self.at) }

    fn toggle_entry(&mut self, i: usize) {
        if i < self.habits.len() {
            let h = &mut self.habits[i];
            if !h.due { self.msg = format!("{} is not due today", h.name); return; }
            if h.done {
                h.done = false;
                h.streak -= 1;
                let g = habit_gold(h.streak);
                self.gold -= g;
                self.gold_today -= g;
                self.msg = format!("Unmarked {} (−{}g)", h.name, g);
            } else {
                let g = habit_gold(h.streak);
                h.done = true;
                h.streak += 1;
                self.gold += g;
                self.gold_today += g;
                self.msg = format!("{} done · +{}g · Streak {}", h.name, g, h.streak);
            }
        } else {
            let q = &mut self.quests[i - self.habits.len()];
            let g = quest_gold(q);
            q.done = !q.done;
            let s = if q.done { 1 } else { -1 };
            self.gold += s * g;
            self.gold_today += s * g;
            self.msg = if q.matured || !q.done {
                format!("{} {} · {:+}g", q.name, if q.done { "done" } else { "reopened" }, s * g)
            } else {
                format!("{} done · 0g (created today, not matured)", q.name)
            };
        }
    }

    fn buy_freeze(&mut self) {
        if self.freezes >= FREEZE_CAP { self.msg = format!("Freeze cap reached ({FREEZE_CAP})"); return; }
        if self.gold < FREEZE_COST { self.msg = format!("A Freeze costs {FREEZE_COST}g"); return; }
        self.gold -= FREEZE_COST;
        self.freezes += 1;
        self.msg = format!("Bought a Freeze (−{FREEZE_COST}g) · {}/{FREEZE_CAP}", self.freezes);
    }

    fn move_map(&mut self, d: i32) {
        let n = self.next_layer().map_or(1, |l| l.len()) as i32;
        self.map_cursor = (self.map_cursor as i32 + d).rem_euclid(n) as usize;
    }

    fn play_node(&mut self) -> bool {
        if self.node.is_some() { return false; }
        if self.steps == 0 { self.msg = "No Steps left — one accrues tomorrow".into(); return false; }
        self.steps -= 1;
        self.nodes_today += 1;
        let kind = match self.next_layer() { Some(l) => l[self.map_cursor], None => Kind::Boss };
        let layer = self.at;
        self.path.push(self.map_cursor);
        self.at += 1;
        self.map_cursor = 0;
        self.node = Some(match kind {
            Kind::Fight | Kind::Boss => {
                let boss = kind == Kind::Boss;
                let enemy = if boss { boss_enemy() } else { enemy_for(layer) };
                Node::Fight { enemy, toggles: vec![false; self.cons.len() + 1], cursor: 0, boss }
            }
            Kind::Shop => Node::Shop { cursor: 0 },
            Kind::Rest => Node::Rest { cursor: 0 },
            Kind::Event => Node::Event { cursor: 0 },
        });
        self.msg = format!("Spent a Step · {} left", self.steps);
        true
    }

    fn fight_preview(&self) -> Option<(bool, i32, u32)> {
        let Some(Node::Fight { enemy, toggles, .. }) = &self.node else { return None };
        let n = self.cons.len();
        if toggles[n] { return None; }
        let mut atk = self.atk_total();
        let mut def = self.def_total();
        let mut e = enemy.clone();
        for (i, c) in self.cons.iter().enumerate() {
            if !toggles[i] { continue; }
            match c {
                Cons::Whetstone => atk += 3,
                Cons::Tonic => def += 3,
                Cons::Smoke => { if !e.traits.is_empty() { e.traits.remove(0); } }
            }
        }
        Some(simulate(self.hp, atk, def, &e))
    }

    fn node_options(&self) -> usize {
        match &self.node {
            Some(Node::Fight { toggles, .. }) => toggles.len(),
            Some(Node::Reward { .. }) => 3,
            Some(Node::Shop { .. }) => SHOP.len() + 1,
            Some(Node::Rest { .. }) => 2 + self.items.iter().filter(|i| !i.plus).count(),
            Some(Node::Event { .. }) => 3,
            _ => 0,
        }
    }

    fn node_key(&mut self, code: KeyCode) {
        let n = self.node_options().max(1);
        match code {
            KeyCode::Char('j') | KeyCode::Down => self.bump_cursor(1, n),
            KeyCode::Char('k') | KeyCode::Up => self.bump_cursor(-1, n),
            KeyCode::Char(' ') => {
                if let Some(Node::Fight { toggles, cursor, .. }) = &mut self.node {
                    if *cursor == toggles.len() - 1 && self.charges == 0 { self.msg = "No Ability charges left".into(); return; }
                    toggles[*cursor] = !toggles[*cursor];
                }
            }
            KeyCode::Enter => self.commit_node(),
            _ => {}
        }
    }

    fn bump_cursor(&mut self, d: i32, n: usize) {
        let c = match &mut self.node {
            Some(Node::Fight { cursor, .. }) | Some(Node::Reward { cursor })
            | Some(Node::Shop { cursor }) | Some(Node::Rest { cursor }) | Some(Node::Event { cursor }) => cursor,
            _ => return,
        };
        *c = (*c as i32 + d).rem_euclid(n as i32) as usize;
    }

    fn close_run(&mut self, won: bool) {
        let essence = 10 * self.at as i32 + if won { 50 } else { 0 } + self.gold / 5;
        self.essence += essence;
        self.node = Some(Node::Closed { won, essence });
    }

    fn commit_node(&mut self) {
        let Some(node) = self.node.take() else { return };
        match node {
            Node::Fight { enemy, toggles, cursor, boss } => {
                let n = self.cons.len();
                if toggles[n] {
                    self.charges -= 1;
                    self.msg = format!("Pet used Scout: skipped {} · {} charge left", enemy.name, self.charges);
                    return;
                }
                self.node = Some(Node::Fight { enemy: enemy.clone(), toggles: toggles.clone(), cursor, boss });
                let (won, lost, _) = self.fight_preview().unwrap();
                self.node = None;
                self.hp -= lost;
                let mut i = 0;
                self.cons.retain(|_| { let keep = !toggles[i]; i += 1; keep });
                if !won || self.hp <= 0 {
                    self.hp = 0;
                    self.msg = format!("Defeated by {}", enemy.name);
                    self.close_run(false);
                } else if boss {
                    self.msg = "The Boss falls!".into();
                    self.close_run(true);
                } else {
                    self.msg = format!("Beat {} · −{} HP", enemy.name, lost);
                    self.node = Some(Node::Reward { cursor: 0 });
                }
            }
            Node::Reward { cursor } => {
                match cursor {
                    0 if self.items.len() < SLOTS => self.items.push(Item { name: "Spiked buckler", atk: 1, def: 1, plus: false }),
                    1 => self.cons.push(Cons::Tonic),
                    2 if self.freezes < FREEZE_CAP => self.freezes += 1,
                    _ => { self.msg = "No room for that".into(); self.node = Some(Node::Reward { cursor }); return; }
                }
                self.msg = format!("Took: {}", REWARDS[cursor]);
            }
            Node::Shop { cursor } => {
                if cursor == SHOP.len() { self.msg = "Left the shop".into(); return; }
                let (name, price, _) = SHOP[cursor];
                if self.gold < price { self.msg = format!("Not enough Gold for {name}"); }
                else if cursor < 2 && self.items.len() >= SLOTS { self.msg = "Item slots full".into(); }
                else {
                    self.gold -= price;
                    match cursor {
                        0 => self.items.push(Item { name: "Iron sword", atk: 3, def: 0, plus: false }),
                        1 => self.items.push(Item { name: "Leather vest", atk: 0, def: 2, plus: false }),
                        2 => self.cons.push(Cons::Whetstone),
                        3 => self.cons.push(Cons::Tonic),
                        _ => self.cons.push(Cons::Smoke),
                    }
                    self.msg = format!("Bought {name} (−{price}g)");
                }
                self.node = Some(Node::Shop { cursor });
            }
            Node::Rest { cursor } => {
                let upgradable: Vec<usize> = (0..self.items.len()).filter(|&i| !self.items[i].plus).collect();
                if cursor == 0 {
                    let healed = 15.min(self.max_hp - self.hp);
                    self.hp += healed;
                    self.msg = format!("Rested · +{healed} HP");
                } else if cursor <= upgradable.len() {
                    let it = &mut self.items[upgradable[cursor - 1]];
                    it.plus = true;
                    self.msg = format!("Upgraded {}+", it.name);
                } else {
                    self.msg = "Left the rest".into();
                }
            }
            Node::Event { cursor } => {
                match cursor {
                    0 if self.gold >= 10 && self.items.len() < SLOTS => {
                        self.gold -= 10;
                        self.items.push(Item { name: "Shrine charm", atk: 1, def: 0, plus: false });
                        self.msg = "Offered 10g · got Shrine charm".into();
                    }
                    0 => { self.msg = "Can't: needs 10g and a free slot".into(); self.node = Some(Node::Event { cursor }); }
                    1 => { let h = 10.min(self.max_hp - self.hp); self.hp += h; self.msg = format!("Drank · +{h} HP"); }
                    _ => { self.gold += 12; self.hp = (self.hp - 6).max(1); self.msg = "Smashed the shrine · +12g, −6 HP".into(); }
                }
            }
            Node::Closed { .. } => { self.node = Some(node); }
        }
    }

    fn node_done(&self) -> bool { self.node.is_none() }
}

const SHOP: [(&str, i32, &str); 5] = [
    ("Iron sword", 20, "Item · +3 ATK"),
    ("Leather vest", 18, "Item · +2 DEF"),
    ("Whetstone", 8, "Consumable · +3 ATK one fight"),
    ("Ironbark tonic", 8, "Consumable · +3 DEF one fight"),
    ("Smoke bomb", 10, "Consumable · ignore one Trait"),
];
const REWARDS: [&str; 3] = ["Item: Spiked buckler (+1 ATK, +1 DEF)", "Consumable: Ironbark tonic", "Freeze"];

fn habit_gold(streak: u32) -> i32 { (5.0 * (1.0 + streak.min(20) as f32 / 10.0)).round() as i32 }
fn quest_gold(q: &Quest) -> i32 {
    if !q.matured { return 0; }
    (match q.size { 'S' => 6, 'M' => 14, _ => 25 }) + if q.due.is_some() { 3 } else { 0 }
}

fn enemy_for(layer: usize) -> Enemy {
    let (name, hp, atk, def, t) = [
        ("Rat swarm", 12, 4, 0, Trait::Fast),
        ("Mud imp", 14, 5, 1, Trait::Thorns),
        ("Bog goblin", 16, 6, 1, Trait::Fast),
        ("Shell beetle", 20, 5, 3, Trait::Armoured),
        ("Briar wolf", 22, 7, 2, Trait::Thorns),
        ("Grave knight", 26, 8, 3, Trait::Armoured),
    ][layer];
    Enemy { name, hp, atk, def, traits: vec![t] }
}
fn boss_enemy() -> Enemy {
    Enemy { name: "Hollow King", hp: 50, atk: 9, def: 3, traits: vec![Trait::Fast, Trait::Armoured] }
}

fn simulate(hp: i32, atk: i32, def: i32, e: &Enemy) -> (bool, i32, u32) {
    let has = |t| e.traits.contains(&t);
    let (mut h, mut eh) = (hp, e.hp);
    let mut hero_turn = !has(Trait::Fast);
    let mut armour = has(Trait::Armoured);
    let mut rounds = 0;
    while h > 0 && eh > 0 && rounds < 200 {
        if hero_turn {
            rounds += 1;
            if armour { armour = false } else { eh -= (atk - e.def).max(1) }
            if has(Trait::Thorns) { h -= 1 }
        } else {
            h -= (e.atk - def).max(1);
        }
        hero_turn = !hero_turn;
    }
    (eh <= 0 && h > 0, hp - h.max(0), rounds)
}

// ---------- shared content (not layout) ----------

fn kind_glyph(k: Kind) -> (&'static str, &'static str, Color) {
    match k {
        Kind::Fight => ("F", "Fight", Color::Red),
        Kind::Event => ("?", "Event", Color::Yellow),
        Kind::Shop => ("$", "Shop", Color::Green),
        Kind::Rest => ("R", "Rest", Color::Blue),
        Kind::Boss => ("B", "Boss", Color::Magenta),
    }
}

fn week_strip(h: &Habit) -> Vec<Span<'static>> {
    let mut v: Vec<Span> = h.week.iter().map(|d| match d {
        1 => "●".green(),
        2 => "*".cyan(),
        0 => "○".red(),
        _ => "·".dark_gray(),
    }).collect();
    v.push(if !h.due { "·".dark_gray() } else if h.done { "●".green().bold() } else { "◌".white() });
    v
}

fn today_lines(app: &App, cursor: Option<usize>, wide: bool) -> Vec<Line<'static>> {
    let mut out = vec![Line::from("Habits".bold().cyan())];
    for (i, h) in app.habits.iter().enumerate() {
        let mut s = vec![
            Span::raw(if h.done { " [x] " } else if h.due { " [ ] " } else { "  -  " }),
            Span::raw(format!("{:<14}", h.name)),
        ];
        if wide { s.push(format!("{:<12}", h.schedule).dark_gray()); }
        s.push(format!("×{:<3}", h.streak).yellow());
        s.extend(week_strip(h));
        s.push(Span::raw(" "));
        if h.due && !h.done { s.push(format!("+{}g", habit_gold(h.streak)).green()); }
        if h.streak == 0 && h.week.contains(&0) { s.push(" repair:1❄".cyan()); }
        let mut l = Line::from(s);
        if !h.due { l = l.patch_style(Style::new().add_modifier(Modifier::DIM)); }
        if cursor == Some(i) { l = l.patch_style(Style::new().add_modifier(Modifier::REVERSED)); }
        out.push(l);
    }
    out.push(Line::from("Quests".bold().cyan()));
    for (j, q) in app.quests.iter().enumerate() {
        let i = app.habits.len() + j;
        let mut s = vec![
            Span::raw(if q.done { " [x] " } else { " [ ] " }),
            Span::raw(format!("{:<14}", q.name)),
            format!("{} ", q.size).magenta().bold(),
        ];
        if let Some(d) = q.due { s.push(format!("due {d} ").dark_gray()); }
        if q.matured { s.push(format!("+{}g", quest_gold(q)).green()); } else { s.push("matures tomorrow".dark_gray()); }
        let mut l = Line::from(s);
        if cursor == Some(i) { l = l.patch_style(Style::new().add_modifier(Modifier::REVERSED)); }
        out.push(l);
    }
    out
}

fn map_lines(app: &App, selectable: bool, wide: bool) -> Vec<Line<'static>> {
    let mut out = vec![];
    let boss = boss_enemy();
    let boss_next = app.at == app.layers.len();
    let mut b = vec![
        "  ☠  ".magenta().bold(),
        Span::styled(" [B] ", Style::new().fg(Color::Magenta).add_modifier(if boss_next && selectable { Modifier::REVERSED } else { Modifier::BOLD })),
        format!(" {} ", boss.name).magenta().bold(),
    ];
    if wide { b.push(format!("{} HP {} ATK {} DEF · Fast, Armoured", boss.hp, boss.atk, boss.def).dark_gray()); }
    out.push(Line::from(b));
    out.push(Line::from(""));
    for r in (0..app.layers.len()).rev() {
        let mut s = vec![format!("  L{}  ", r + 1).dark_gray()];
        for (j, k) in app.layers[r].iter().enumerate() {
            let (g, name, c) = kind_glyph(*k);
            let label = if wide { format!(" [{g}] {name:<6}") } else { format!(" [{g}] ") };
            let mut st = Style::new().fg(c);
            if r < app.at {
                st = if app.path[r] == j { Style::new().fg(Color::White).add_modifier(Modifier::BOLD) } else { Style::new().fg(Color::DarkGray) };
            }
            if r == app.at && selectable && j == app.map_cursor { st = st.add_modifier(Modifier::REVERSED | Modifier::BOLD); }
            s.push(Span::styled(label, st));
            if r < app.at && app.path[r] == j { s.push("●".green()); } else { s.push(" ".into()); }
        }
        if r == app.at { s.push("  ← next".yellow()); }
        out.push(Line::from(s));
    }
    out.push(Line::from("  camp".dark_gray()));
    out
}

fn legend() -> Line<'static> {
    Line::from(vec!["F".red(), " fight  ".dark_gray(), "?".yellow(), " event  ".dark_gray(), "$".green(), " shop  ".dark_gray(), "R".blue(), " rest  ".dark_gray(), "B".magenta(), " boss  ".dark_gray(), "●".green(), " your path".dark_gray()])
}

fn sel(l: Line<'static>, on: bool) -> Line<'static> {
    if on { l.patch_style(Style::new().add_modifier(Modifier::REVERSED)) } else { l }
}

fn node_title(app: &App) -> String {
    match &app.node {
        Some(Node::Fight { enemy, boss, .. }) => format!(" {} · {} ", if *boss { "BOSS" } else { "Fight" }, enemy.name),
        Some(Node::Reward { .. }) => " Reward ".into(),
        Some(Node::Shop { .. }) => " Shop ".into(),
        Some(Node::Rest { .. }) => " Rest ".into(),
        Some(Node::Event { .. }) => " Event · The humming shrine ".into(),
        Some(Node::Closed { .. }) => " Run closed ".into(),
        None => String::new(),
    }
}

fn node_lines(app: &App) -> Vec<Line<'static>> {
    let mut out = vec![];
    match &app.node {
        Some(Node::Fight { enemy, toggles, cursor, .. }) => {
            out.push(Line::from(vec![
                "You   ".bold(), format!("{} HP  {} ATK  {} DEF", app.hp, app.atk_total(), app.def_total()).into(),
            ]));
            out.push(Line::from(vec![
                format!("{:<6}", "Foe").red().bold(), format!("{} HP  {} ATK  {} DEF", enemy.hp, enemy.atk, enemy.def).into(),
            ]));
            for t in &enemy.traits { out.push(Line::from(vec!["      Trait: ".dark_gray(), t.label().yellow()])); }
            out.push(Line::from(""));
            out.push(Line::from("Before you commit".bold()));
            for (i, c) in app.cons.iter().enumerate() {
                out.push(sel(Line::from(format!(" [{}] {}", if toggles[i] { "x" } else { " " }, c.label())), *cursor == i));
            }
            let n = app.cons.len();
            out.push(sel(Line::from(vec![
                format!(" [{}] ", if toggles[n] { "x" } else { " " }).into(),
                format!("Pet · Scout ({}/2): skip this fight, no reward", app.charges).cyan(),
            ]), *cursor == n));
            out.push(Line::from(""));
            let p = match app.fight_preview() {
                None => Line::from(" PREVIEW  skip · no HP lost · no reward ".black().on_cyan().bold()),
                Some((true, lost, r)) => Line::from(format!(" PREVIEW  WIN in {r} rounds · −{lost} HP ({} → {}) ", app.hp, app.hp - lost).black().on_green().bold()),
                Some((false, _, r)) => Line::from(format!(" PREVIEW  DEFEAT on round {r} · the Run closes ").white().on_red().bold()),
            };
            out.push(p);
        }
        Some(Node::Reward { cursor }) => {
            out.push(Line::from("Pick one".bold()));
            for (i, r) in REWARDS.iter().enumerate() { out.push(sel(Line::from(format!(" {r}")), *cursor == i)); }
        }
        Some(Node::Shop { cursor }) => {
            out.push(Line::from(vec!["Gold ".into(), format!("{}", app.gold).yellow().bold(), format!("  ·  slots {}/{}", app.items.len(), SLOTS).dark_gray()]));
            for (i, (name, price, what)) in SHOP.iter().enumerate() {
                let l = Line::from(vec![format!(" {:<15}", name).into(), format!("{:>3}g  ", price).yellow(), what.dark_gray()]);
                out.push(sel(l, *cursor == i));
            }
            out.push(sel(Line::from(" Leave"), *cursor == SHOP.len()));
        }
        Some(Node::Rest { cursor }) => {
            out.push(Line::from(format!("HP {}/{}", app.hp, app.max_hp)));
            out.push(sel(Line::from(format!(" Heal +{} HP", 15.min(app.max_hp - app.hp))), *cursor == 0));
            let mut i = 1;
            for it in app.items.iter().filter(|i| !i.plus) {
                out.push(sel(Line::from(format!(" Upgrade {} → {}+", it.name, it.name)), *cursor == i));
                i += 1;
            }
            out.push(sel(Line::from(" Leave"), *cursor == i));
        }
        Some(Node::Event { cursor }) => {
            out.push(Line::from("A cracked shrine hums in the reeds.".italic()));
            out.push(Line::from(""));
            let opts = ["Offer 10 Gold → Item: Shrine charm (+1 ATK)", "Drink from the font → +10 HP", "Smash it → +12 Gold, −6 HP"];
            for (i, o) in opts.iter().enumerate() { out.push(sel(Line::from(format!(" {o}")), *cursor == i)); }
        }
        Some(Node::Closed { won, essence }) => {
            out.push(Line::from(if *won { "You beat the Hollow King.".green().bold() } else { "The Hero fell. The Run is over.".red().bold() }));
            out.push(Line::from(format!("Essence earned: {essence} (total {})", app.essence)));
            out.push(Line::from("Items, Consumables and HP reset. Freezes, Pet and Unlocks stay.".dark_gray()));
            out.push(Line::from("R resets the prototype".dark_gray()));
        }
        None => {}
    }
    out
}

fn node_hint(app: &App) -> &'static str {
    match &app.node {
        Some(Node::Fight { .. }) => "j/k move · space toggle · enter fight",
        Some(Node::Closed { .. }) => "R reset",
        Some(_) => "j/k move · enter choose",
        None => "",
    }
}

fn pet_art() -> Vec<Line<'static>> {
    ["   ,_,  ", "  (o.o) ", "  /)_)~ ", "   \" \"  "].iter().map(|s| Line::from(s.green())).collect()
}

fn bar(v: i32, max: i32, w: usize, c: Color) -> Span<'static> {
    let f = ((v.max(0) as f32 / max as f32) * w as f32).round() as usize;
    Span::styled(format!("{}{}", "█".repeat(f), "░".repeat(w - f)), Style::new().fg(c))
}

fn hero_lines(app: &App) -> Vec<Line<'static>> {
    let mut out = vec![
        Line::from(vec!["HP ".into(), bar(app.hp, app.max_hp, 14, Color::Red), format!(" {}/{}", app.hp, app.max_hp).into()]),
        Line::from(format!("ATK {}  DEF {}", app.atk_total(), app.def_total())),
        Line::from(format!("Items {}/{}", app.items.len(), SLOTS).dark_gray()),
    ];
    for it in &app.items { out.push(Line::from(format!(" ▸ {}{}", it.name, if it.plus { "+" } else { "" }))); }
    out.push(Line::from("Consumables".dark_gray()));
    for c in &app.cons { out.push(Line::from(format!(" ▸ {}", c.label()))); }
    out
}

fn pet_lines(app: &App, wide: bool) -> Vec<Line<'static>> {
    let mut out = if wide { pet_art() } else { vec![] };
    out.push(Line::from(vec!["Moss ".green().bold(), "· Stage 2 Sprout".into()]));
    out.push(Line::from(vec!["XP ".into(), bar(340, 500, 14, Color::Green), " 340/500".into()]));
    out.push(Line::from("+30 XP today from 3 Pi tasks".dark_gray()));
    out.push(Line::from(vec!["Ability ".into(), "Scout".cyan().bold(), format!(" {}/2 charges", app.charges).into()]));
    if wide {
        out.push(Line::from(""));
        out.push(Line::from("Abilities".bold()));
        out.push(Line::from(" ✓ Scout     skip a fight (Stage 1)"));
        out.push(Line::from(" ✓ Burrow    turn a Node into a rest (Stage 2)"));
        out.push(Line::from(" · Haggle    reroll a shop (Stage 3, 500 XP)".dark_gray()));
        out.push(Line::from(" · ???       (Stage 4)".dark_gray()));
        out.push(Line::from(" · ???       (Stage 5)".dark_gray()));
    }
    out
}

fn status_spans(app: &App) -> Vec<Span<'static>> {
    vec![
        "Gold ".dark_gray(), format!("{}", app.gold).yellow().bold(),
        "  Steps ".dark_gray(), format!("{}/{}", app.steps, STEP_CAP).bold(),
        "  HP ".dark_gray(), format!("{}/{}", app.hp, app.max_hp).red().bold(),
        "  Freezes ".dark_gray(), format!("{}/{}", app.freezes, FREEZE_CAP).cyan().bold(),
        "  Essence ".dark_gray(), format!("{}", app.essence).magenta(),
    ]
}

fn centered(area: Rect, w: u16, h: u16) -> Rect {
    let (w, h) = (w.min(area.width), h.min(area.height));
    Rect::new(area.x + (area.width - w) / 2, area.y + (area.height - h) / 2, w, h)
}

fn hints(pairs: &[(&str, &str)]) -> Line<'static> {
    let mut s = vec![];
    for (k, d) in pairs { s.push(format!(" {k}").cyan().bold()); s.push(format!(" {d} ").dark_gray()); }
    Line::from(s)
}

// ---------- Variant A: lazygit panels ----------

fn draw_a(f: &mut Frame, app: &App, area: Rect) {
    let [top, body, keys] = Layout::vertical([Constraint::Length(1), Constraint::Min(0), Constraint::Length(1)]).areas(area);
    let mut st = vec![" habitos ".black().on_green().bold(), " Week 47 · Wed · Run day 3/7  ".into()];
    st.extend(status_spans(app));
    f.render_widget(Paragraph::new(Line::from(st)), top);

    let [left, main] = Layout::horizontal([Constraint::Percentage(42), Constraint::Min(0)]).areas(body);
    let [p1, p2, p3] = Layout::vertical([Constraint::Length(12), Constraint::Length(6), Constraint::Min(0)]).areas(left);
    let blk = |n: usize, t: &str| {
        let b = Block::bordered().border_type(BorderType::Rounded).title(format!("[{}] {}", n + 1, t));
        if app.a_focus == n { b.border_style(Style::new().fg(Color::Green).bold()) } else { b.border_style(Style::new().fg(Color::DarkGray)) }
    };
    let cur = (app.a_focus == 0).then_some(app.today_cursor);
    f.render_widget(Paragraph::new(today_lines(app, cur, false)).block(blk(0, "Today")), p1);
    f.render_widget(Paragraph::new(pet_lines(app, false)).block(blk(1, "Pet")), p2);
    f.render_widget(Paragraph::new(hero_lines(app)).block(blk(2, "Hero")), p3);

    let main_block = blk(3, &if app.node.is_some() { node_title(app) } else { "Run · Week 47".into() });
    let lines = if app.node.is_some() { node_lines(app) } else {
        let mut l = map_lines(app, app.a_focus == 3, true);
        l.push(Line::from(""));
        l.push(legend());
        l
    };
    f.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }).block(main_block), main);

    let ctx: Vec<(&str, &str)> = match (app.a_focus, app.node.is_some()) {
        (3, true) => vec![("j/k", "move"), ("space", "toggle"), ("enter", "commit")],
        (3, false) => vec![("h/l", "choose node"), ("enter", "play (1 Step)")],
        (0, _) => vec![("j/k", "move"), ("space", "mark done"), ("a", "add"), ("e", "edit")],
        _ => vec![],
    };
    let mut l = hints(&[("1-4/tab", "panel"), ("f", "buy Freeze")]);
    l.spans.extend(hints(&ctx).spans);
    l.spans.push(format!("  {}", app.msg).yellow());
    f.render_widget(Paragraph::new(l), keys);
}

fn key_a(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Tab => app.a_focus = (app.a_focus + 1) % 4,
        KeyCode::BackTab => app.a_focus = (app.a_focus + 3) % 4,
        KeyCode::Char(c @ '1'..='4') => app.a_focus = c as usize - '1' as usize,
        _ if app.a_focus == 0 => match code {
            KeyCode::Char('j') | KeyCode::Down => app.today_cursor = (app.today_cursor + 1) % app.entries(),
            KeyCode::Char('k') | KeyCode::Up => app.today_cursor = (app.today_cursor + app.entries() - 1) % app.entries(),
            KeyCode::Char(' ') | KeyCode::Enter => app.toggle_entry(app.today_cursor),
            _ => {}
        },
        _ if app.a_focus == 3 && app.node.is_some() => app.node_key(code),
        _ if app.a_focus == 3 => match code {
            KeyCode::Char('h') | KeyCode::Left => app.move_map(-1),
            KeyCode::Char('l') | KeyCode::Right => app.move_map(1),
            KeyCode::Enter => { app.play_node(); }
            _ => {}
        },
        _ => {}
    }
}

// ---------- Variant B: guided flow ----------

fn draw_b(f: &mut Frame, app: &App, area: Rect) {
    let [crumbs, body, foot] = Layout::vertical([Constraint::Length(2), Constraint::Min(0), Constraint::Length(2)]).areas(area);
    let stages = [(BStage::CheckIn, "Check-in"), (BStage::Map, "Map"), (BStage::Node, "Node"), (BStage::Done, "Done")];
    let idx = stages.iter().position(|s| s.0 == app.b_stage).unwrap();
    let mut c = vec![];
    for (i, (_, n)) in stages.iter().enumerate() {
        if i > 0 { c.push(" ── ".dark_gray()); }
        c.push(if i < idx { format!("● {n}").green() } else if i == idx { format!("◉ {n}").bold().white() } else { format!("○ {n}").dark_gray() });
    }
    f.render_widget(Paragraph::new(Line::from(c)).alignment(Alignment::Center), crumbs);

    let card = centered(body, 76, body.height);
    let (title, mut lines, hint): (String, Vec<Line>, &str) = match app.b_stage {
        BStage::CheckIn => {
            let mut l = vec![Line::from("What did you do today?".bold()), Line::from("")];
            l.extend(today_lines(app, Some(app.today_cursor), true));
            l.push(Line::from(""));
            l.push(Line::from(vec!["Earned today ".dark_gray(), format!("+{}g", app.gold_today).yellow().bold(), "   Freezes ".dark_gray(), format!("{}/{}", app.freezes, FREEZE_CAP).cyan()]));
            (" Wednesday ".into(), l, "j/k move · space mark · f buy Freeze (15g) · enter continue →")
        }
        BStage::Map => {
            let mut l = vec![Line::from(vec!["Steps ".dark_gray(), format!("{}/{}", app.steps, STEP_CAP).bold(), "   ".into()]).patch_style(Style::new())];
            l[0].spans.extend(status_spans(app).into_iter().skip(4));
            l.push(Line::from(""));
            l.extend(map_lines(app, true, true));
            l.push(Line::from(""));
            l.push(legend());
            (" Run · day 3/7 ".into(), l, "h/l choose · enter play (1 Step) · esc finish for today")
        }
        BStage::Node => {
            let mut l = node_lines(app);
            if app.node.is_none() { l = vec![Line::from(app.msg.clone().bold())]; }
            (node_title(app), l, if app.node.is_none() { "enter continue →" } else { node_hint(app) })
        }
        BStage::Done => {
            let done = app.habits.iter().filter(|h| h.done).count();
            let due = app.habits.iter().filter(|h| h.due).count();
            let l = vec![
                Line::from("That's today.".bold()),
                Line::from(""),
                Line::from(format!("Habits      {done}/{due} done")),
                Line::from(vec!["Gold        ".into(), format!("+{}g", app.gold_today).yellow()]),
                Line::from(format!("Nodes       {} played · {} Steps left", app.nodes_today, app.steps)),
                Line::from(vec!["Pet         ".into(), "Moss +30 XP (3 Pi tasks)".green()]),
                Line::from(format!("Run         layer {}/6 · Boss in {} layers", app.at, 6usize.saturating_sub(app.at) + 1)),
            ];
            (" See you tomorrow ".into(), l, "q quit · enter start over (prototype)")
        }
    };
    if app.b_stage != BStage::Node && app.b_stage != BStage::Done {
        lines.push(Line::from(""));
        lines.push(Line::from(app.msg.clone().yellow()));
    }
    let h = (lines.len() as u16 + 2).min(card.height);
    let card = centered(card, card.width, h);
    f.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }).block(Block::bordered().border_type(BorderType::Double).title(title).title_alignment(Alignment::Center)), card);
    f.render_widget(Paragraph::new(Line::from(hint.dark_gray())).alignment(Alignment::Center), foot);
}

fn key_b(app: &mut App, code: KeyCode) {
    match app.b_stage {
        BStage::CheckIn => match code {
            KeyCode::Char('j') | KeyCode::Down => app.today_cursor = (app.today_cursor + 1) % app.entries(),
            KeyCode::Char('k') | KeyCode::Up => app.today_cursor = (app.today_cursor + app.entries() - 1) % app.entries(),
            KeyCode::Char(' ') => app.toggle_entry(app.today_cursor),
            KeyCode::Enter => app.b_stage = if app.node.is_some() { BStage::Node } else { BStage::Map },
            _ => {}
        },
        BStage::Map => match code {
            KeyCode::Char('h') | KeyCode::Left => app.move_map(-1),
            KeyCode::Char('l') | KeyCode::Right => app.move_map(1),
            KeyCode::Enter => { if app.play_node() { app.b_stage = BStage::Node } else if app.steps == 0 { app.b_stage = BStage::Done } }
            KeyCode::Esc => app.b_stage = BStage::Done,
            _ => {}
        },
        BStage::Node => {
            if app.node_done() {
                if code == KeyCode::Enter { app.b_stage = if app.steps > 0 { BStage::Map } else { BStage::Done } }
            } else { app.node_key(code) }
        }
        BStage::Done => if code == KeyCode::Enter { app.b_stage = BStage::CheckIn },
    }
}

// ---------- Variant C: k9s views ----------

fn draw_c(f: &mut Frame, app: &App, area: Rect) {
    let [head, cmd, body, crumb] = Layout::vertical([Constraint::Length(6), Constraint::Length(if app.c_cmd.is_some() { 3 } else { 0 }), Constraint::Min(0), Constraint::Length(1)]).areas(area);
    let [info, keys, logo] = Layout::horizontal([Constraint::Length(34), Constraint::Min(0), Constraint::Length(26)]).areas(head);
    let kv = |k: &str, v: Span<'static>| Line::from(vec![format!("{k:<9}").yellow(), v]);
    f.render_widget(Paragraph::new(vec![
        kv("Run:", "Week 47 · day 3/7".into()),
        kv("Steps:", format!("{}/{}", app.steps, STEP_CAP).bold()),
        kv("Gold:", format!("{}", app.gold).yellow().bold()),
        kv("HP:", format!("{}/{}", app.hp, app.max_hp).red().bold()),
        kv("Freezes:", format!("{}/{}", app.freezes, FREEZE_CAP).cyan().bold()),
        kv("Pet:", "Moss · Sprout · Scout 2/2".green()),
    ]), info);
    let k = |a: &str, b: &str| Line::from(vec![format!("<{a}>").magenta().bold(), format!(" {b}").dark_gray()]);
    let mut kl = vec![k("1", "today"), k("2", "run"), k("3", "pet"), k("4", "habits"), k(":", "command"), k("f", "buy Freeze")];
    match app.c_view {
        CView::Today => kl.push(k("space", "mark")),
        CView::Run => kl.push(k("enter", "play node")),
        CView::Habits => kl.push(k("a/e/d", "add/edit/del")),
        _ => {}
    }
    let [k1, k2] = Layout::horizontal([Constraint::Length(18), Constraint::Min(0)]).areas(keys);
    f.render_widget(Paragraph::new(kl[..4].to_vec()), k1);
    f.render_widget(Paragraph::new(kl[4..].to_vec()), k2);
    f.render_widget(Paragraph::new(vec![
        Line::from(" _       _    _ _       ".cyan()),
        Line::from("| |_  __| |__(_) |_ ___ ".cyan()),
        Line::from("| ' \\/ _` | '_ \\ |  _/ _ \\".cyan()),
        Line::from("|_||_\\__,_|_.__/_|\\__\\___/".cyan()),
    ]), logo);

    if let Some(c) = &app.c_cmd {
        f.render_widget(Paragraph::new(Line::from(vec!["🐾> ".into(), c.clone().bold(), "█".into()])).block(Block::bordered().border_style(Style::new().fg(Color::Cyan))), cmd);
    }

    let (name, count) = match app.c_view {
        CView::Today => ("today", app.entries()),
        CView::Run => ("run", 7),
        CView::Pet => ("pet", 5),
        CView::Habits => ("habits", app.habits.len() + app.quests.len()),
    };
    let block = Block::bordered().border_style(Style::new().fg(Color::Cyan))
        .title(Line::from(vec![format!(" {name}").cyan().bold(), format!("[{count}] ").magenta()]).alignment(Alignment::Center));
    match app.c_view {
        CView::Today | CView::Habits => {
            let manage = app.c_view == CView::Habits;
            let header = if manage { vec!["NAME", "KIND", "SCHEDULE / SIZE", "STREAK", "LAST 7", "CREATED"] } else { vec!["", "NAME", "KIND", "STREAK", "LAST 7", "GOLD"] };
            let mut rows = vec![];
            for (i, h) in app.habits.iter().enumerate() {
                let mark = if h.done { "✓" } else if h.due { "·" } else { "-" };
                let cells: Vec<Cell> = if manage {
                    vec![h.name.into(), "habit".into(), h.schedule.into(), format!("{}", h.streak).into(), Line::from(week_strip(h)).into(), "3w ago".into()]
                } else {
                    vec![mark.into(), h.name.into(), "habit".into(), format!("×{}", h.streak).into(), Line::from(week_strip(h)).into(),
                         if h.due && !h.done { format!("+{}", habit_gold(h.streak)).into() } else { "".into() }]
                };
                let mut row = Row::new(cells);
                if !h.due { row = row.style(Style::new().fg(Color::DarkGray)); } else if h.done { row = row.style(Style::new().fg(Color::Green)); }
                if i == app.today_cursor { row = row.style(Style::new().bg(Color::Cyan).fg(Color::Black)); }
                rows.push(row);
            }
            for (j, q) in app.quests.iter().enumerate() {
                let i = app.habits.len() + j;
                let cells: Vec<Cell> = if manage {
                    vec![q.name.into(), "quest".into(), format!("{}{}", q.size, q.due.map(|d| format!(" · due {d}")).unwrap_or_default()).into(), "".into(), "".into(), if q.matured { "2d ago" } else { "today" }.into()]
                } else {
                    vec![if q.done { "✓" } else { "·" }.into(), q.name.into(), format!("quest {}", q.size).into(), "".into(), "".into(),
                         if q.matured { format!("+{}", quest_gold(q)) } else { "0 (new)".into() }.into()]
                };
                let mut row = Row::new(cells);
                if q.done { row = row.style(Style::new().fg(Color::Green)); }
                if i == app.today_cursor { row = row.style(Style::new().bg(Color::Cyan).fg(Color::Black)); }
                rows.push(row);
            }
            let widths = if manage {
                vec![Constraint::Length(16), Constraint::Length(7), Constraint::Length(18), Constraint::Length(8), Constraint::Length(9), Constraint::Min(8)]
            } else {
                vec![Constraint::Length(2), Constraint::Length(16), Constraint::Length(9), Constraint::Length(8), Constraint::Length(9), Constraint::Min(6)]
            };
            let t = Table::new(rows, widths).header(Row::new(header).style(Style::new().fg(Color::White).bold())).block(block).column_spacing(2);
            f.render_widget(t, body);
        }
        CView::Run => {
            let [m, side] = Layout::horizontal([Constraint::Min(0), Constraint::Length(44)]).areas(block.inner(body));
            f.render_widget(block, body);
            let mut l = map_lines(app, app.node.is_none(), true);
            l.push(Line::from(""));
            l.push(legend());
            f.render_widget(Paragraph::new(l), m);
            f.render_widget(Paragraph::new(hero_lines(app)).wrap(Wrap { trim: false }).block(Block::new().borders(ratatui::widgets::Borders::LEFT).title(" Hero ")), side);
        }
        CView::Pet => {
            let [a, b] = Layout::horizontal([Constraint::Length(12), Constraint::Min(0)]).areas(block.inner(body));
            f.render_widget(block, body);
            f.render_widget(Paragraph::new(pet_art()), a);
            f.render_widget(Paragraph::new(pet_lines(app, false).into_iter().chain(pet_lines(app, true).into_iter().skip(8)).collect::<Vec<_>>()), b);
        }
    }

    if app.node.is_some() {
        let lines = node_lines(app);
        let pop = centered(body, 64, lines.len() as u16 + 4);
        f.render_widget(Clear, pop);
        f.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }).block(
            Block::bordered().border_type(BorderType::Thick).border_style(Style::new().fg(Color::Yellow)).title(node_title(app)).title_bottom(Line::from(format!(" {} ", node_hint(app))).dark_gray())), pop);
    }

    let mut cr = vec![format!(" <{name}> ").black().on_magenta().bold()];
    if app.node.is_some() { cr.push(" <node> ".black().on_yellow().bold()); }
    cr.push(format!("  {}", app.msg).yellow());
    f.render_widget(Paragraph::new(Line::from(cr)), crumb);
}

fn key_c(app: &mut App, code: KeyCode) {
    if let Some(cmd) = &mut app.c_cmd {
        match code {
            KeyCode::Char(c) => cmd.push(c),
            KeyCode::Backspace => { cmd.pop(); }
            KeyCode::Esc => app.c_cmd = None,
            KeyCode::Enter => {
                let v = match cmd.trim() { "today" | "t" => Some(CView::Today), "run" | "r" | "map" => Some(CView::Run), "pet" | "p" => Some(CView::Pet), "habits" | "h" | "quests" => Some(CView::Habits), _ => None };
                match v { Some(v) => app.c_view = v, None => app.msg = format!("unknown view :{}", cmd.trim()) }
                app.c_cmd = None;
            }
            _ => {}
        }
        return;
    }
    if app.node.is_some() { app.node_key(code); return; }
    match code {
        KeyCode::Char(':') => app.c_cmd = Some(String::new()),
        KeyCode::Char('1') => app.c_view = CView::Today,
        KeyCode::Char('2') => app.c_view = CView::Run,
        KeyCode::Char('3') => app.c_view = CView::Pet,
        KeyCode::Char('4') => app.c_view = CView::Habits,
        _ => match app.c_view {
            CView::Today | CView::Habits => match code {
                KeyCode::Char('j') | KeyCode::Down => app.today_cursor = (app.today_cursor + 1) % app.entries(),
                KeyCode::Char('k') | KeyCode::Up => app.today_cursor = (app.today_cursor + app.entries() - 1) % app.entries(),
                KeyCode::Char(' ') if app.c_view == CView::Today => app.toggle_entry(app.today_cursor),
                KeyCode::Enter if app.c_view == CView::Today => app.toggle_entry(app.today_cursor),
                _ => {}
            },
            CView::Run => match code {
                KeyCode::Char('h') | KeyCode::Left => app.move_map(-1),
                KeyCode::Char('l') | KeyCode::Right => app.move_map(1),
                KeyCode::Enter => { app.play_node(); }
                _ => {}
            },
            _ => {}
        },
    }
}

// ---------- switcher + loop ----------

fn draw(f: &mut Frame, app: &App) {
    let [main, sw] = Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).areas(f.area());
    match app.variant { 0 => draw_a(f, app, main), 1 => draw_b(f, app, main), _ => draw_c(f, app, main) }
    let label = Line::from(vec![
        " ◀ [ ".black().on_light_yellow(),
        format!(" {} ", VARIANTS[app.variant]).black().on_light_yellow().bold(),
        " ] ▶ ".black().on_light_yellow(),
        "  PROTOTYPE · R reset · q quit".dark_gray(),
    ]);
    f.render_widget(Paragraph::new(label).alignment(Alignment::Center), sw);
}

fn run(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut app = App::new(0);
    loop {
        terminal.draw(|f| draw(f, &app))?;
        let Event::Key(key) = event::read()? else { continue };
        if key.kind != KeyEventKind::Press { continue; }
        if app.c_cmd.is_none() {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('[') => { app.variant = (app.variant + 2) % 3; continue; }
                KeyCode::Char(']') => { app.variant = (app.variant + 1) % 3; continue; }
                KeyCode::Char('R') => { app = App::new(app.variant); continue; }
                KeyCode::Char('f') => { app.buy_freeze(); continue; }
                _ => {}
            }
        }
        match app.variant { 0 => key_a(&mut app, key.code), 1 => key_b(&mut app, key.code), _ => key_c(&mut app, key.code) }
    }
}

fn main() -> std::io::Result<()> {
    if let Ok(keys) = std::env::var("DUMP") {
        let mut app = App::new(0);
        for (i, part) in keys.split(',').enumerate() {
            if i > 0 {
                let code = match part { "enter" => KeyCode::Enter, "tab" => KeyCode::Tab, "esc" => KeyCode::Esc, "space" => KeyCode::Char(' '), s => KeyCode::Char(s.chars().next().unwrap()) };
                match code {
                    KeyCode::Char('[') => app.variant = (app.variant + 2) % 3,
                    KeyCode::Char(']') => app.variant = (app.variant + 1) % 3,
                    KeyCode::Char('f') => app.buy_freeze(),
                    _ => match app.variant { 0 => key_a(&mut app, code), 1 => key_b(&mut app, code), _ => key_c(&mut app, code) },
                }
            }
        }
        let mut t = ratatui::Terminal::new(ratatui::backend::TestBackend::new(110, 30)).unwrap();
        let _ = 0;
        t.draw(|f| draw(f, &app)).unwrap();
        let b = t.backend().buffer();
        for y in 0..b.area.height { let row: String = (0..b.area.width).map(|x| b[(x, y)].symbol().to_string()).collect(); println!("{row}"); }
        return Ok(());
    }
    let mut terminal = ratatui::init();
    let r = run(&mut terminal);
    ratatui::restore();
    r
}
