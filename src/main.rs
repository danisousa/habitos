mod tui;

use std::error::Error;
use std::path::PathBuf;

use chrono::{DateTime, FixedOffset, Local};
use habitos::{Clock, Game};
use rusqlite::Connection;

struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> DateTime<FixedOffset> {
        Local::now().fixed_offset()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let dir = PathBuf::from(std::env::var("HOME")?).join(".local/share/habitos");
    std::fs::create_dir_all(&dir)?;
    let game = Game::open(
        Connection::open(dir.join("habitos.db"))?,
        Box::new(SystemClock),
    )?;
    tui::run(game)
}
