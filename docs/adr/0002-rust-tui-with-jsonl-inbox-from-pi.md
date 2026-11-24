# Rust TUI on SQLite, fed by a report-only pi extension through a JSONL inbox

Habitos is a standalone Rust + Ratatui binary storing raw facts in SQLite (`rusqlite`, bundled), while the pi extension is TypeScript that only appends events to `~/.local/share/habitos/inbox.jsonl`, which the app ingests and deduplicates by `id`. Rust was chosen to learn it, over Go + Bubble Tea (faster iteration) and OpenTUI (TS-native, would share a language with the extension, but pre-1.0 and Bun-only); sharing a language mattered little because the extension only feeds data. The append-only inbox keeps all game logic in the app, works with the TUI closed, and means pi never blocks or fails because of Habitos.

## Consequences

The Pi task rule and XP amounts live in the app, so they can be retuned without touching the extension; the extension's payload must stay raw facts rather than decisions.
