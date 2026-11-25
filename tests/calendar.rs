mod support;

use rusqlite::Connection;
use support::{clock_at, date, open, open_at};

#[test]
fn one_minute_before_four_counts_for_the_previous_game_date() {
    let (mut game, _clock) = open_at("2025-03-05T03:59:00+01:00");
    assert_eq!(game.today().unwrap(), date("2025-03-04"));
}

#[test]
fn moving_the_clock_back_keeps_the_last_seen_game_date() {
    let (mut game, clock) = open_at("2025-03-05T10:00:00+01:00");
    game.today().unwrap();
    clock.set("2025-03-04T10:00:00+01:00");
    assert_eq!(game.today().unwrap(), date("2025-03-05"));
}

#[test]
fn the_clock_returning_after_moving_back_does_not_skip_a_game_date() {
    let (mut game, clock) = open_at("2025-03-05T10:00:00+01:00");
    game.today().unwrap();
    clock.set("2025-03-04T10:00:00+01:00");
    game.today().unwrap();
    clock.set("2025-03-06T10:00:00+01:00");
    assert_eq!(game.today().unwrap(), date("2025-03-06"));
}

#[test]
fn the_last_seen_game_date_survives_a_restart_with_the_clock_moved_back() {
    let path = std::env::temp_dir().join(format!("habitos-calendar-{}.db", std::process::id()));
    let _ = std::fs::remove_file(&path);
    let clock = clock_at("2025-03-05T10:00:00+01:00");
    open(Connection::open(&path).unwrap(), &clock)
        .today()
        .unwrap();

    clock.set("2025-03-04T10:00:00+01:00");
    let mut game = open(Connection::open(&path).unwrap(), &clock);
    let today = game.today().unwrap();
    std::fs::remove_file(&path).unwrap();
    assert_eq!(today, date("2025-03-05"));
}

#[test]
fn four_oclock_starts_the_new_game_date() {
    let (mut game, _clock) = open_at("2025-03-05T04:00:00+01:00");
    assert_eq!(game.today().unwrap(), date("2025-03-05"));
}
