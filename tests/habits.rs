mod support;

use habitos::Schedule;
use rusqlite::Connection;
use support::{clock_at, open, open_at};

#[test]
fn an_every_day_habit_marked_done_shows_as_done_today() {
    let (mut game, _clock) = open_at("2025-03-05T10:00:00+01:00");
    let read = game.add_habit("Read", Schedule::EveryDay).unwrap();
    game.mark_habit_done_today(read).unwrap();

    let habits = game.habits().unwrap();
    assert_eq!(habits.len(), 1);
    assert_eq!(habits[0].name, "Read");
    assert_eq!(habits[0].schedule, Schedule::EveryDay);
    assert!(habits[0].done_today);
}

#[test]
fn a_mark_at_three_fifty_nine_does_not_count_for_the_game_date_starting_at_four() {
    let (mut game, clock) = open_at("2025-03-05T03:59:00+01:00");
    let read = game.add_habit("Read", Schedule::EveryDay).unwrap();
    game.mark_habit_done_today(read).unwrap();

    clock.set("2025-03-05T04:00:00+01:00");
    assert!(!game.habits().unwrap()[0].done_today);
}

#[test]
fn habits_and_marks_persist_across_restarts() {
    let path = std::env::temp_dir().join(format!("habitos-test-{}.db", std::process::id()));
    let _ = std::fs::remove_file(&path);
    let clock = clock_at("2025-03-05T10:00:00+01:00");

    let mut game = open(Connection::open(&path).unwrap(), &clock);
    let read = game.add_habit("Read", Schedule::EveryDay).unwrap();
    game.mark_habit_done_today(read).unwrap();
    drop(game);

    let mut game = open(Connection::open(&path).unwrap(), &clock);
    let habits = game.habits().unwrap();
    std::fs::remove_file(&path).unwrap();
    assert_eq!(habits[0].name, "Read");
    assert!(habits[0].done_today);
}
