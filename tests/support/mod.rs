#![allow(dead_code)]

use std::cell::Cell;
use std::rc::Rc;

use chrono::{DateTime, FixedOffset, NaiveDate};
use habitos::{Clock, Game};
use rusqlite::Connection;

#[derive(Clone)]
pub struct FakeClock(Rc<Cell<DateTime<FixedOffset>>>);

impl FakeClock {
    pub fn set(&self, at: &str) {
        self.0.set(instant(at));
    }
}

impl Clock for FakeClock {
    fn now(&self) -> DateTime<FixedOffset> {
        self.0.get()
    }
}

pub fn instant(at: &str) -> DateTime<FixedOffset> {
    DateTime::parse_from_rfc3339(at).unwrap()
}

pub fn date(d: &str) -> NaiveDate {
    d.parse().unwrap()
}

pub fn clock_at(at: &str) -> FakeClock {
    FakeClock(Rc::new(Cell::new(instant(at))))
}

pub fn open_at(at: &str) -> (Game, FakeClock) {
    let clock = clock_at(at);
    let game = Game::open(
        Connection::open_in_memory().unwrap(),
        Box::new(clock.clone()),
    )
    .unwrap();
    (game, clock)
}
