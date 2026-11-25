use chrono::{DateTime, NaiveDate, Utc};
use rusqlite::{Connection, OptionalExtension, Result};

const MIGRATIONS: &[&str] = &["
    CREATE TABLE last_seen (
        id INTEGER PRIMARY KEY CHECK (id = 1),
        game_date TEXT NOT NULL
    );
    CREATE TABLE habits (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL
    );
    CREATE TABLE habit_schedules (
        habit_id INTEGER NOT NULL REFERENCES habits (id),
        from_game_date TEXT NOT NULL,
        schedule TEXT NOT NULL,
        PRIMARY KEY (habit_id, from_game_date)
    );
    CREATE TABLE habit_marks (
        habit_id INTEGER NOT NULL REFERENCES habits (id),
        counts_for TEXT NOT NULL,
        recorded_on TEXT NOT NULL,
        recorded_at TEXT NOT NULL,
        PRIMARY KEY (habit_id, counts_for)
    );
"];

pub struct StoredHabit {
    pub id: i64,
    pub name: String,
    pub schedule: String,
}

pub struct Store {
    conn: Connection,
}

impl Store {
    pub fn open(conn: Connection) -> Result<Store> {
        let version: i64 = conn.pragma_query_value(None, "user_version", |r| r.get(0))?;
        for (i, sql) in MIGRATIONS.iter().enumerate().skip(version as usize) {
            conn.execute_batch(sql)?;
            conn.pragma_update(None, "user_version", i as i64 + 1)?;
        }
        Ok(Store { conn })
    }

    pub fn last_seen_game_date(&self) -> Result<Option<NaiveDate>> {
        self.conn
            .query_row("SELECT game_date FROM last_seen", [], |r| r.get(0))
            .optional()
    }

    pub fn insert_habit(&self, name: &str, schedule: &str, from: NaiveDate) -> Result<i64> {
        self.conn
            .execute("INSERT INTO habits (name) VALUES (?1)", [name])?;
        let id = self.conn.last_insert_rowid();
        self.conn.execute(
            "INSERT INTO habit_schedules (habit_id, from_game_date, schedule) VALUES (?1, ?2, ?3)",
            (id, from, schedule),
        )?;
        Ok(id)
    }

    pub fn habits(&self) -> Result<Vec<StoredHabit>> {
        let mut stmt = self.conn.prepare(
            "SELECT h.id, h.name, s.schedule FROM habits h
             JOIN habit_schedules s ON s.habit_id = h.id
             AND s.from_game_date = (SELECT MAX(from_game_date) FROM habit_schedules WHERE habit_id = h.id)
             ORDER BY h.id",
        )?;
        stmt.query_map([], |r| {
            Ok(StoredHabit {
                id: r.get(0)?,
                name: r.get(1)?,
                schedule: r.get(2)?,
            })
        })?
        .collect()
    }

    pub fn insert_mark(
        &self,
        habit_id: i64,
        counts_for: NaiveDate,
        recorded_on: NaiveDate,
        recorded_at: DateTime<Utc>,
    ) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO habit_marks (habit_id, counts_for, recorded_on, recorded_at) VALUES (?1, ?2, ?3, ?4)",
            (habit_id, counts_for, recorded_on, recorded_at),
        )?;
        Ok(())
    }

    pub fn is_marked(&self, habit_id: i64, counts_for: NaiveDate) -> Result<bool> {
        self.conn.query_row(
            "SELECT EXISTS (SELECT 1 FROM habit_marks WHERE habit_id = ?1 AND counts_for = ?2)",
            (habit_id, counts_for),
            |r| r.get(0),
        )
    }

    pub fn set_last_seen_game_date(&self, date: NaiveDate) -> Result<()> {
        self.conn.execute(
            "INSERT INTO last_seen (id, game_date) VALUES (1, ?1)
             ON CONFLICT (id) DO UPDATE SET game_date = excluded.game_date",
            [date],
        )?;
        Ok(())
    }
}
