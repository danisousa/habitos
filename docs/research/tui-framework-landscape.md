# TUI frameworks for Habitos (game-like, lazygit/k9s/btop-style TUI, solo dev, TypeScript companion)

Evidence labels: **[D]** = stated directly by a primary source I fetched. **[I]** = my inference, not stated by a source. **[U]** = not verified in this run.
Repo stats and release dates come from the GitHub REST API, fetched during this run. Treat them as a snapshot. The API returned dates in 2026.
Web search was rate-limited (Exa 429), so I used direct fetches of repos and official docs only. Coverage is uneven; see Open risks.

## Summary
- **Textual (Python)** is the fastest for a solo dev to get a polished, panel-heavy app. It has a CSS-like layout model, built-in themes, a command palette, a dev console, `textual serve` for the browser, and a Pilot test API plus SVG snapshot tests [D]. Its weak spots are the Python runtime and single-binary distribution [I]. It also has no TypeScript affinity.
- **Ratatui (Rust)** is the most proven "btop/gitui-class" stack. Its showcase includes gitui, bottom, bandwhich and Atuin, and it is MIT-licensed with 23k stars [D]. It has Canvas, charts and gauges, plus insta snapshot testing [D]. Rust compile and iteration cost is the main drag for a solo, game-feel project [I].
- **Bubble Tea v2 + Lip Gloss (Go)** has a mature ecosystem: the highest star count here (45.8k) and 18,000+ dependent apps [D]. v2 has a cell-based renderer, colour downsampling, and mouse and keyboard handling [D]. Harmonica (spring animation) and ntcharts (charts) are named companions [D]. It builds to a single binary [I] and has a golden-file test example [D].
- **Ink (React/TypeScript)** is the most widely deployed TS option. Claude Code, Gemini CLI, Copilot CLI and Wrangler use it [D]. It has `useAnimation`, a `maxFps` default of 30, incremental rendering and `ink-testing-library` [D]. It has no first-class mouse API in the docs I read: `useInput` drops mouse reports [D]. It is better suited to inline/CLI UIs than to dense multi-panel dashboards [I].
- **OpenTUI (TypeScript on a Zig core)** is the best TS-native fit for a rich, animated, mouse-aware TUI. It has flexbox/Yoga layout, a Timeline animation module, React and Solid bindings, and an in-memory test renderer [D]. OpenCode uses it in production [D]. It is young and pre-1.0 (v0.9.9, repo created July 2025) [D]. It requires Bun ≥1.3.14, or Node ≥26.4 with `--experimental-ffi` [D].
- **TypeScript affinity is relevant but not decisive.** The companion pi extension only needs to feed data, for example through a file, SQLite or a socket [I]. A non-TS TUI can read that. OpenTUI is the only option where both halves share a language and could share types.
- **Provisional ranking for this game-like personal TUI** [I]: OpenTUI if you want TS and game feel and accept Bun and pre-1.0 churn. Otherwise Textual for iteration speed, then Ratatui or Bubble Tea if you want a compiled single binary. Ink is the weakest for panels, mouse and animation.

## Comparison table

| Dimension | Textual (Py) | Ratatui (Rust) | Bubble Tea v2 + Lip Gloss (Go) | Ink (TS/React) | OpenTUI (TS/Zig) |
|---|---|---|---|---|---|
| Stars / issues (snapshot) | 37.7k / 419 [D] | 23.4k / 174 [D] | 45.9k / 228 [D] | 40.3k / 29 [D] | 13.9k / 79 [D] |
| Latest release (API) | v8.2.8, 2026-06-30 [D] | ratatui-v0.30.2, 2026-06-19 [D] | v2.1.2, 2026-11-18 [D] | v8.0.0, 2026-10-03 [D] | v0.9.9, 2026-11-23 [D] |
| Pre-1.0? | No (v8) | Yes (0.30) | No (v2) | No (v8) | **Yes (0.9.x)** |
| Visual richness | Rich-based; truecolor/graphs not confirmed in docs I fetched [U]. Sparkline blog asset exists in repo [D] | Canvas (Line, Circle, Points, Rectangle, Map, custom shapes) [D]; chart and gauge examples [D]; braille [U] | Cell-based renderer, colour downsampling [D]; Lip Gloss borders, tables, lists, trees, blending [D]; ntcharts [D] | Colour/borders via `<Text>`/`<Box>`; braille spinner example [D]; no chart widget in core [I] | Text, flexbox, images, video, 3D [D]; charts [U] |
| Animation | Smooth-scroll, easing docs/blog assets in repo [D]; timers [I] | Manual: you redraw per tick [I] | Harmonica springs [D]; doom-fire, space, progress-animated examples [D] | `useAnimation`; `maxFps` 30 default [D] | `animation/Timeline.ts` [D]; "updates the cells that changed" [D] |
| Mouse | Pilot simulates click/hover [D]; real mouse is built in [I] | mouse-drawing example [D] | Built-in; BubbleZone for hit-testing [D] | Not documented as supported; mouse reports dropped by `useInput` [D] | Mouse and focus built in [D] |
| Theming | Built-in themes (nord, gruvbox, tokyo-night, solarized, atom-one…), CSS variables, `^t` cycle [D] | Manual `Style`/palette; no theme system [I] | Lip Gloss styles; no theme system [I] | Manual props [I] | Manual; theme-mode dev script exists [D] |
| Layout | CSS-like (TCSS) [D] | Constraint/flex layout (constraints, flex, constraint-explorer examples) [D] | Lip Gloss join/align; manual composition [D] | Flexbox (Yoga) [D] | Flexbox (Yoga) [D] |
| Widget ecosystem | Large built-in set plus command palette [D] | Core widgets plus third-party (awesome-ratatui) [D] | Bubbles (inputs, viewport, spinner…) [D] | Many third-party components [D] | selects, inputs, scroll boxes [D]; component matrix per binding [D] |
| Testing | `run_test` + Pilot; `pytest-textual-snapshot` SVG snapshots [D] | `TestBackend` + insta snapshots [D] | Golden file in `examples/simple` [D] | `ink-testing-library` (`lastFrame()`) [D] | `createTestRenderer`, `captureCharFrame`, keymap testing [D] |
| Hot reload / dev tools | `textual-dev` console [D]; `textual serve` to browser [D] | `bacon.toml` in repo [D]; hot reload [U] | `tea.LogToFile` + `tail -f` [D]; hot reload [U] | React DevTools supported [D] | Dev scripts (keypress debug) [D]; Bun-based dev [D] |
| Distribution | `uvx`/pip; needs Python [D]; single binary [U] | cargo binary [I] | Go static binary; goreleaser config in repo [D/I] | `npm install ink react` [D]; needs Node [I] | Bun or Node 26.4+ with FFI flag; Node SEA tested on Linux x64 only [D]; prebuilt examples for macOS/Linux x64/Windows [D] |
| Notable apps | [U] (Posting and others not verified here) | gitui, bottom, bandwhich, Atuin, Codex, chess-tui, minesweep-rs [D] | chezmoi, gh-dash, circumflex, Glow, Mods, Tetrigo [D] | Claude Code, Gemini CLI, Copilot CLI, Wrangler, Shopify CLI; games: Sudoku, Sea Trader, Wordle [D] | OpenCode [D] |

## Per-framework notes

### Textual
- **Strengths [D]:** It is a Python framework that runs in the terminal or the browser. It has a dev console, a fuzzy command palette (`ctrl+p`) and built-in themes. `textual serve` shares apps on the web. https://github.com/Textualize/textual, https://textual.textualize.io/guide/design/
- **Testing [D]:** `run_test()` runs headless, and `Pilot.press`/`click`/`hover` simulate input. `pytest-textual-snapshot` stores SVG screenshots with a visual diff report. https://textual.textualize.io/guide/testing/
- **Maintenance [D]:** v8.2.8 (2026-06-30). The repo was last pushed 2026-11-09. It has 419 open issues and 37.7k stars. https://api.github.com/repos/Textualize/textual
- **Gaps:** I did not verify an FPS ceiling, braille/graph widgets, or single-binary packaging (for example PyInstaller). All are [U].
- **Fit [I]:** Likely the quickest path to a lazygit/Posting-style layout. Python and TS need a data bridge.

### Ratatui
- **Model [D]:** It is a Rust library with a `no_std` option. The demo apps cover canvas, chart, gauge, popup, scrollbar, mouse-drawing and constraint layout. https://github.com/ratatui/ratatui
- **Canvas [D]:** Predefined shapes plus custom `Shape`s. https://docs.rs/ratatui/latest/ratatui/widgets/canvas/index.html
- **Testing [D]:** Snapshot testing with insta. https://ratatui.rs/recipes/testing/snapshots/
- **Showcase [D]:** https://ratatui.rs/showcase/apps/
- **Maintenance [D]:** v0.30.2 (2026-06-19), pushed 2026-11-23. It is still 0.x, and `BREAKING-CHANGES.md` is tracked in the repo. https://github.com/ratatui/ratatui
- **Animation [I]:** There is no built-in animation system. You drive ticks and redraw yourself, which gives control but more boilerplate.

### Bubble Tea v2 + Lip Gloss
- **Model [D]:** Elm-style model/update/view. `View()` returns a `tea.View` that can declare alt-screen, mouse tracking and cursor. It has a cell-based renderer, colour downsampling, and keyboard and mouse handling. https://github.com/charmbracelet/bubbletea
- **Ecosystem [D]:** Bubbles, Lip Gloss, Harmonica, BubbleZone and ntcharts. Lip Gloss has layout, tables, lists, trees and canvas/layer code in its tree. https://github.com/charmbracelet/lipgloss
- **Maintenance [D]:** v2.1.2 released 2026-11-18, pushed 2026-11-24. The release notes mention idle programs now sit idle. https://api.github.com/repos/charmbracelet/bubbletea/releases/latest
- **Apps [D]:** chezmoi, gh-dash, Tetrigo (a game), and others. https://github.com/charmbracelet/bubbletea
- **Gap [I]:** There is no CSS-like theming or ready-made panel layout, so lazygit-style panes need manual composition.

### Ink
- **Model [D]:** React renderer with flexbox. `useAnimation` shares one timer, `maxFps` defaults to 30, `incrementalRendering` is opt-in, and `alternateScreen` is opt-in. https://github.com/vadimdemedes/ink
- **Mouse [D]:** `useInput` ignores mouse reports, and the README only advises enabling SGR mode if your app enables mouse tracking. No first-class mouse API was found. https://github.com/vadimdemedes/ink
- **Testing [D]:** `ink-testing-library`. https://github.com/vadimdemedes/ink
- **Apps [D]:** Claude Code, Gemini CLI, Copilot CLI, Wrangler, plus hobby games. https://github.com/vadimdemedes/ink
- **Maintenance [D]:** v8.0.0 (2026-10-03), requires React 19.3+, only 29 open issues. The README notes strict rules for AI-generated PRs. https://api.github.com/repos/vadimdemedes/ink/releases/latest
- **Fit [I]:** Easy if you know React. It is better for inline, streaming UIs than for dense dashboards.

### OpenTUI
- **Model [D]:** Zig native core with TypeScript bindings (core, React, Solid) and a Rust binding. It uses Yoga flexbox layout and updates only the changed cells. https://opentui.com/docs, https://github.com/anomalyco/opentui
- **Capabilities [D]:** selects, inputs, scroll boxes, keyboard and mouse controls, video, sound, images and 3D. https://github.com/anomalyco/opentui
- **Animation [D]:** `packages/core/src/animation/Timeline.ts` exists, with tests. https://github.com/anomalyco/opentui
- **Testing [D]:** `createTestRenderer` (in-memory), `captureCharFrame`, and mouse/input/clock helpers. https://opentui.com/docs/core-concepts/testing
- **Runtime [D]:** Bun ≥1.3.14, or Node ≥26.4 with `--experimental-ffi`. Node SEA tests run on Linux x64 only. https://opentui.com/docs/getting-started/runtime-support
- **Maintenance [D]:** v0.9.9 on 2026-11-23, with per-release API snapshots (`api/0.x.txt`). It has 79 open issues and 13.9k stars. https://api.github.com/repos/anomalyco/opentui/releases/latest
- **Production use [D]:** "OpenCode uses OpenTUI in production for millions of users." https://github.com/anomalyco/opentui
- **Risk [I]:** Pre-1.0, so expect API churn. Bun or experimental Node FFI is a nonstandard runtime dependency.

### Other contenders
Not researched in depth because search was rate-limited. These are [U]: Go's tview and gocui (lazygit's library), Python's urwid, Rust's cursive, and Node's blessed. I did not verify any of them.

## Fit for a game-like TUI
- **Frame-driven animation and live bars [I]:** OpenTUI (Timeline plus cell diffing), Bubble Tea (tick commands, Harmonica springs, doom-fire example) and Ratatui (immediate-mode redraw per tick) are the natural fits. Textual and Ink are workable. Ink's `maxFps` defaults to 30 [D].
- **Panels, shortcuts footer, vim keys:** Textual has bindings and a CSS layout, which is the least custom code [I]. In OpenTUI, Ratatui and Bubble Tea you build the footer and pane focus yourself, though OpenTUI has a keymap package and focus handling [D].
- **Mouse:** Textual, OpenTUI, Bubble Tea and Ratatui have documented or example support. Ink does not [D/I].
- **Game feel from libraries and examples [D]:** Bubble Tea ships doom-fire, space and cellbuffer examples. Ratatui ships a destroy/demo2 effect. OpenTUI advertises 3D, video and sound.
- **Snapshot-testing game state [D]:** All five have some form of text or snapshot testing. Textual's SVG snapshots are the most polished.
- **TS bridge [I]:** If the pi extension writes JSON or SQLite that the TUI reads, TS affinity matters little. A shared TS domain model (habits, XP rules) favours OpenTUI or Ink.

## Open risks
- OpenTUI is 0.x with fast releases, and it needs Bun or Node 26.4+ `--experimental-ffi`. Distribution beyond Linux x64 SEA is untested by the project [D].
- Ratatui is 0.x. Ink and Textual have had major version bumps (Ink 8 requires React 19.3+ [D]).
- Textual's single-binary story, FPS ceiling and graph widgets were not verified [U].
- Hot reload was not confirmed for any framework except Textual's dev tooling and OpenTUI's dev scripts. For Ratatui, Bubble Tea and Ink it is [U]. For Ink, the README mentions React DevTools only [D].
- Braille chart support (Ratatui, ntcharts) was not confirmed in the text I fetched [U].
- Lip Gloss theming and layering details are inferred from file names in the repo tree, not from reading the API docs [I].
- Notable-app lists are vendor-curated showcases, so they are biased towards successes.
- Stars and issue counts are a point-in-time snapshot from the GitHub API.

## Sources
- https://github.com/Textualize/textual and https://api.github.com/repos/Textualize/textual (plus `/releases/latest`)
- https://textual.textualize.io/guide/testing/
- https://textual.textualize.io/guide/design/
- https://github.com/ratatui/ratatui and https://api.github.com/repos/ratatui/ratatui (plus `/releases/latest`)
- https://docs.rs/ratatui/latest/ratatui/widgets/canvas/index.html
- https://ratatui.rs/recipes/testing/snapshots/
- https://ratatui.rs/showcase/apps/
- https://github.com/charmbracelet/bubbletea and https://api.github.com/repos/charmbracelet/bubbletea (plus `/releases/latest`)
- https://github.com/charmbracelet/lipgloss
- https://github.com/vadimdemedes/ink and https://api.github.com/repos/vadimdemedes/ink (plus `/releases/latest`)
- https://github.com/anomalyco/opentui (also reachable as sst/opentui) and https://api.github.com/repos/anomalyco/opentui (plus `/releases/latest`)
- https://opentui.com/docs
- https://opentui.com/docs/getting-started/runtime-support
- https://opentui.com/docs/core-concepts/testing
