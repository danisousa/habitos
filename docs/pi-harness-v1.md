# Pi harness v1

Decisions from the grilling session that defined the first version of the Pi harness.

## Goal

Minimal base to learn Pi's primitives while being usable day to day. Habitos is both a real habits app and the
harness test bed. The app's stack is decided by the pipeline itself, so v1 ships no stack skills.

**Done when:** `/skill:grill-with-docs` on the Habitos app runs in Pi without missing anything.

## Global (`~/.pi/agent/`)

- `AGENTS.md` adapted from `~/.claude/CLAUDE.md` (which stays untouched): no cavecrew, bash search guidance
  (`rg`/`fd`), delegation policy.
- Delegation policy: `scout` authorised by default; any other subagent only on explicit request or when a skill says so.
- `pi-web-access` installed, zero-config (Exa MCP). API keys later in `~/.pi/agent/web-search.json` if needed.
- `~/.agents/skills/{to-prd,to-issues}` retired. Global `grill-with-docs` (devrag/code-search) kept: project
  `.agents/skills` loads before `~/.agents/skills`, so inside Habitos the project version wins.
- Extension `extensions/git-guard.ts` on bash tool calls:

  | Level | Commands | Action |
  |---|---|---|
  | Always block | `git push` to `main`/`master` (explicit or current branch), `git push --force`/`-f`/`--force-with-lease` | Block with reason |
  | Confirm | `git commit`, other `git push`, `gh pr create`, `gh pr merge`, `git reset --hard`, `git rebase` | `ctx.ui.confirm`; no UI → block |
  | Free | everything else (`git status/diff/log/add/checkout/branch`, `gh issue …`, `gh pr view`) | — |

- Extension `extensions/git-status.ts`: appends to the default footer via `ctx.ui.setStatus` the branch and
  `+added −removed` lines since `HEAD`, including untracked files (respecting `.gitignore`). Context % stays in
  Pi's own footer. No output outside a git repo; repo without commits counts everything as untracked.

## Habitos (project)

- Pipeline skills copied into `.agents/skills/` from `Fintrackr/fintrackr-monorepo/.agents/skills/`:
  grilling, domain-modeling, grill-with-docs, to-spec, to-tickets, implement, tdd, setup-matt-pocock-skills,
  handoff, research, wayfinder, prototype, code-review — plus `writing-great-skills` from `~/.claude/skills/`.
- Minimal port only:
  - "Call the Skill tool with X" → read the X skill (grill-with-docs, implement ×2, tdd).
  - code-review: `Agent` tool / `subagent_type` → `pi-subagents` delegation to `code-reviewer`; drop the
    finance and E2E axes; fix `.claude/skills/...` paths.
  - Remove Fintrackr specifics (implement `apps/web` paths, hard-coded package labels in to-spec/to-tickets →
    defer to `docs/agents/`).
  - Invocation syntax is `/skill:<name>`.
- Subagents via `pi-subagents`:

  | Agent | Model | Where |
  |---|---|---|
  | `scout` | Sonnet 5.5, without `write` | override in `.pi/settings.json` |
  | `researcher` | Sonnet 5.5 | override in `.pi/settings.json` |
  | `code-reviewer` (ported from Fintrackr `.claude/agents/`) | Sonnet 5.5, read-only tools | `.pi/agents/code-reviewer.md` |
  | rest | session model | — |

- Tracker: GitHub Issues on `danisousa/habitos` (currently **public**).
- Repo bootstrap: minimal base commit on `main` (`.gitignore` + `README.md`); the harness goes through branch
  `chore/pi-harness-v1` → PR.

## Build order

Session 1 (global):

1. `~/.pi/agent/AGENTS.md`
2. `pi install npm:pi-web-access`
3. Retire `~/.agents/skills/{to-prd,to-issues}`
4. `git-guard.ts` — test in a scratch repo under `/tmp`
5. `git-status.ts`

Session 2 (opened in Habitos, fresh context):

6. `git init`, remote, base commit; the user pushes `main` by hand (the guard blocks it)
7. Branch `chore/pi-harness-v1`; copy and port the skills; diff against Fintrackr shows only the agreed changes
8. `.pi/agents/code-reviewer.md` + `.pi/settings.json`; verify scout runs on Sonnet without `write`
9. `/skill:setup-matt-pocock-skills` → `docs/agents/*`, project `AGENTS.md`, GitHub labels
10. Commit + `gh pr create` → merge
11. Acceptance: `/skill:grill-with-docs` on the Habitos app

## Later (v2 candidates)

- Promote what works in Habitos to global.
- Wider delegation (worker/reviewer) once trusted.
- Replace the footer entirely if the default one gets noisy.
- Ports of `create-pr`, `create-issue`, `update-repo-context`, stack skills.
