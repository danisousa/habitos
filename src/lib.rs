mod calendar;
mod store;

use chrono::{DateTime, FixedOffset, NaiveDate};
use rusqlite::{Connection, Result};
use store::Store;

pub trait Clock {
    fn now(&self) -> DateTime<FixedOffset>;
}

pub trait SeedSource {
    fn next_seed(&mut self) -> u64;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Schedule {
    EveryDay,
}

impl Schedule {
    fn key(self) -> &'static str {
        match self {
            Schedule::EveryDay => "every_day",
        }
    }

    fn from_key(key: &str) -> Schedule {
        match key {
            "every_day" => Schedule::EveryDay,
            _ => unreachable!("unknown schedule {key}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HabitId(i64);

pub struct HabitRow {
    pub id: HabitId,
    pub name: String,
    pub schedule: Schedule,
    pub done_today: bool,
}

pub struct Game {
    store: Store,
    clock: Box<dyn Clock>,
    #[expect(dead_code, reason = "first read by the Run engine in #36")]
    seeds: Box<dyn SeedSource>,
}

impl Game {
    pub fn open(
        conn: Connection,
        clock: Box<dyn Clock>,
        seeds: Box<dyn SeedSource>,
    ) -> Result<Game> {
        Ok(Game {
            store: Store::open(conn)?,
            clock,
            seeds,
        })
    }

    pub fn add_habit(&mut self, name: &str, schedule: Schedule) -> Result<HabitId> {
        let today = self.today()?;
        self.store
            .insert_habit(name, schedule.key(), today)
            .map(HabitId)
    }

    pub fn mark_habit_done_today(&mut self, habit: HabitId) -> Result<()> {
        let today = self.today()?;
        let recorded_at = self.clock.now().to_utc();
        self.store.insert_mark(habit.0, today, today, recorded_at)
    }

    pub fn habits(&mut self) -> Result<Vec<HabitRow>> {
        let today = self.today()?;
        self.store
            .habits()?
            .into_iter()
            .map(|h| {
                Ok(HabitRow {
                    id: HabitId(h.id),
                    done_today: self.store.is_marked(h.id, today)?,
                    name: h.name,
                    schedule: Schedule::from_key(&h.schedule),
                })
            })
            .collect()
    }

    pub fn today(&mut self) -> Result<NaiveDate> {
        let now = calendar::game_date(self.clock.now());
        match self.store.last_seen_game_date()? {
            Some(seen) if seen >= now => Ok(seen),
            _ => {
                self.store.set_last_seen_game_date(now)?;
                Ok(now)
            }
        }
    }
}
