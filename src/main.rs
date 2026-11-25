mod tui;

use std::error::Error;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, FixedOffset, Local};
use habitos::{Clock, Game, SeedSource};
use rusqlite::Connection;

struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> DateTime<FixedOffset> {
        Local::now().fixed_offset()
    }
}

struct TimeSeeds;

impl SeedSource for TimeSeeds {
    fn next_seed(&mut self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let dir = PathBuf::from(std::env::var("HOME")?).join(".local/share/habitos");
    std::fs::create_dir_all(&dir)?;
    let game = Game::open(
        Connection::open(dir.join("habitos.db"))?,
        Box::new(SystemClock),
        Box::new(TimeSeeds),
    )?;
    tui::run(game)
}
