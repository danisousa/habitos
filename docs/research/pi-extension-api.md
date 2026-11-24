# Research: pi extension API as a signal source for Habitos (pet XP)

Root (R) = `/Users/danielgarciasousa/.pi/agent/install/releases/1.6.0/node_modules/@earendil-works/pi-coding-agent`
Docs cited by section heading (line numbers not captured). `.d.ts` cited by symbol.

## Summary
- Pi extensions are TypeScript modules (loaded via jiti, no build step) that run **inside the pi process with full OS permissions**. There is no sandbox. They can use Node built-ins (`node:fs`, `node:net`, `fetch`, `child_process`) and `pi.exec()`. (R/docs/extensions.md: intro, "Available Imports", "Writing an Extension")
- The best "a task finished" hook is `agent_settled`. It fires once, after retries, compaction retries and queued follow-ups are done. `agent_end` fires per low-level run and is explicitly *not* a final-idle signal. (extensions.md "Lifecycle Overview", "agent_start / agent_end / agent_before_settle / agent_settled")
- Richer data comes from `turn_end`, `agent_end.messages`, `message_end` (assistant `usage` tokens and cost), `tool_execution_end` / `tool_result` (name, args, isError, durationMs), `input` and `before_agent_start` (user prompt), and `session_start` / `session_shutdown`.
- **There is no git-commit event.** A commit can only be inferred from `bash` tool calls or results (for example, a command containing `git commit`), or detected by running `git` through `pi.exec` after a run.
- Token and cost data are exposed. Each assistant message has `usage {input, output, cacheRead, cacheWrite, totalTokens, cost{…total}}`. `ctx.getContextUsage()` gives context-window usage. Session-wide totals are not exposed as a ready-made API; sum over `ctx.sessionManager.getEntries()`. (message-types.md "Usage"; session-format.md "UsageEntry")
- To talk to a local app, the simplest options are: append JSON lines to a file, spawn a CLI with `pi.exec`, POST via `fetch`, or write to a Unix socket or SQLite. Handlers are `async` and awaited in sequence, so slow work delays pi. Keep them quick or fire-and-forget.
- Install: global `~/.pi/agent/extensions/` or project `.pi/extensions/` (project-local loads only after the project is trusted). You can also use `settings.json` `packages`/`extensions` paths, `pi install npm:|git:|./path`, or `pi -e` for one run. (extensions.md "Extension Locations"; packages.md)
- Extensions only run when pi runs, in all modes (TUI, RPC, JSON, print). Without pi running, Habitos sees nothing. Pi's session JSONL files (`~/.pi/agent/sessions/...`) are an independent, extension-free data source. (extensions.md "Mode Behavior"; session-format.md)

## Extension model (runtime, install, lifecycle)
- **Runtime/language:** default-export factory `export default function (pi: ExtensionAPI) {…}`, sync or async. TypeScript is loaded through jiti. `.js` also works. Host-provided packages are `@earendil-works/pi-coding-agent`, `pi-agent-core`, `pi-ai`, `pi-tui` and `typebox`. npm deps work via a `package.json` next to the extension. (extensions.md "Writing an Extension", "Available Imports")
- **Placement:**

  | Location | Scope |
  |---|---|
  | `~/.pi/agent/extensions/*.ts`, `*/index.ts`, or `*/package.json` with a `pi.extensions` field | Global |
  | `.pi/extensions/...` | Project-local, loads only after project trust |
  | `settings.json` `"extensions": [paths]` and `"packages": ["npm:…","git:…"]` | Either (global settings, or `.pi/settings.json` with `--local`) |
  | `pi -e ./x.ts` | One-off; no hot reload |

  Auto-discovered extensions hot-reload with `/reload`. (extensions.md "Extension Locations"; packages.md "Install and manage packages")
- **Packaging:** a pi package is a directory or npm/git package with `package.json` `"pi": {"extensions": [...]}` or conventional `extensions/` dir. Install with `pi install npm:…`, `git:…` or `./local`. `pi install -l` writes to `.pi/settings.json`. Host packages go in `peerDependencies` (`"*"`), not `dependencies`. (packages.md "Create a package", "Declare dependencies")
- **Lifecycle:** the factory runs at load (an async factory is awaited before `session_start`). The docs say not to start processes, sockets, watchers or timers from the factory, because it may run in invocations with no session. Start them in `session_start` and clean up in an idempotent `session_shutdown`. (extensions.md "Long-lived resources and shutdown")
- **Session replacement** (`/new`, `/resume`, `/fork`, `/reload`) emits `session_shutdown` on the old runtime, then reloads extensions and emits `session_start` on a fresh instance. In-memory state is lost, so re-initialize it. (extensions.md "Lifecycle Overview", "Session Events")
- **Handler order and errors:** handlers run in extension load order, then registration order. Extension errors are logged and the agent continues. A `tool_call` handler error blocks the tool. (extensions.md "Handler Composition", "Error Handling")

## Events catalogue
Sources: extensions.md "Events" sections; types.d.ts `R/dist/core/extensions/types.d.ts` (`ExtensionEvent` union).

| Event | When it fires | Payload / data relevant to "task completed" |
|---|---|---|
| `session_start` | Startup, reload, new, resume, fork | `reason`, `previousSessionFile?`; `ctx.sessionManager.getSessionId()`, `.getSessionFile()` (can be ephemeral/undefined), `ctx.cwd` |
| `session_shutdown` | Quit (Ctrl+C/D, SIGHUP, SIGTERM), reload, new, resume, fork | `reason`, `targetSessionFile?` |
| `input` | User input received, after extension commands, before skill/template expansion. Can transform or handle. | `text`, `images?`, `source` ("interactive" \| "rpc" \| "extension"), `streamingBehavior?` |
| `before_agent_start` | Before a run, after input expansion | `messageType` "user" → `prompt`, `images`; "custom" → `message`; `systemPromptOptions` (cwd, skills, context files) |
| `agent_start` | A low-level agent run begins | none |
| `turn_start` | Each turn (one model request plus its tool calls) | `turnIndex`, `timestamp` |
| `message_start` / `message_update` / `message_end` | Per message (user, assistant, toolResult, custom). `message_update` is the streaming token-level event. | `message`; assistant `message_end` has final `usage`, `stopReason`, `provider`, `model`, `content` (including `toolCall` blocks). Handlers can replace the message, so treat as read-only. |
| `tool_call` | After `tool_execution_start`, before execution. Can block or mutate input. | `toolName`, `toolCallId`, `input` (bash: `{command, timeout?}`; edit/write/read: `path`…), `parentToolCallId?` |
| `tool_execution_start` / `_update` / `_end` | Tool lifecycle | `toolCallId`, `toolName`, `args`; end adds `result`, `isError`, `durationMs?`. End fires for every finalized call, including blocked and failed ones. |
| `tool_result` | After execution, before `tool_execution_end`. Can patch the result. | `toolName`, `toolCallId`, `input`, `content[]`, `isError`, `details` (bash: `{truncation?, fullOutputPath?}`; no exit-code field in `BashToolDetails`), `usage?` (nested-model usage) |
| `turn_end` | After the assistant and tool-result messages are persisted | `turnIndex`, `message` (assistant), `toolResults[]`, `outcome` ("completed" \| "aborted" \| "error"), `entries`, `continue`. It can append entries or request a continuation. Guard against loops. |
| `agent_end` | A low-level run ends. Pi may still retry, compact or process queued follow-ups. | `messages` (messages from this run) |
| `agent_before_settle` | Final actionable boundary. Skipped on abort. | same boundary state as `turn_end` (`outcome`, `entries`, `continue`, `context`) |
| **`agent_settled`** | Final: pi is idle and no auto-continuation will occur. Notification-only. | `aborted: boolean`. Use `ctx.sessionManager` to inspect the run. `ctx.isIdle()` is true. |
| `session_compact` / `_failed` / `session_before_compact` | Compaction | `reason`, `willRetry`, `compactionEntry`. Compaction usage counts toward session totals. |
| `model_select`, `thinking_level_select` | Model or thinking change | model / level |
| `user_bash` | User's `!` / `!!` commands (not LLM tool calls) | `command`, `cwd`, `excludeFromContext`. This is the only hook for a user-typed `git commit`. |
| `before_provider_request`, `after_provider_response`, `provider_stream_event` | Provider debugging hooks | payload, status/headers, raw stream. Not needed for XP; stream hook delays consumption. |
| `project_trust`, `resources_discover`, `ui_prompt_start/_end`, `session_info_changed`, `session_tree`, `cache_warming_decision`, `mcp_servers_change` | Misc | not relevant |

**Pull-style data (not events):**
- `ctx.sessionManager.getEntries()` / `getBranch()` return the full JSONL-equivalent history with per-entry ISO timestamps. (extensions.md "ctx.sessionManager")
- `UsageEntry` (`type:"usage"`) records non-message usage such as `cache_warm`. Compaction and branch-summary entries carry `usage`. (session-format.md)
- `ctx.getContextUsage()` returns `{tokens,…}` for the context window. (extensions.md "ctx.getContextUsage()")
- `ctx.mode` ("tui" \| "rpc" \| "json" \| "print") and `ctx.hasUI`. `ctx.cwd`.
- `pi.getSessionName()` and `pi.setSessionName()` are available.

**Usage shape** (message-types.md "Usage"): `{input, output, cacheRead, cacheWrite, cacheWrite1h?, reasoning?, totalTokens, cost{input,output,cacheRead,cacheWrite,total}}`. `reasoning` is already inside `output`. Cost depends on the model's configured prices, which can be 0 for custom or proxy providers. Handlers can overwrite `usage.cost` via `message_end`.

**Environment seen by LLM bash commands:** `PI_SESSION_ID`, `PI_SESSION_FILE`, `PI_PROVIDER`, `PI_MODEL`, `PI_REASONING_LEVEL`, plus `AI_AGENT=pi` and `PI_CODING_AGENT=true` process markers. These are not injected into user `!` commands. (R/docs/environment-variables.md)

## Talking to an external app (options and trade-offs)
All of these run with the user's permissions inside the pi process. (extensions.md intro)

1. **Append JSONL to a file** (for example `~/.habitos/events.jsonl`) with `node:fs` `appendFile`.
   - Pros: decoupled; no Habitos process needed at write time; Habitos can tail or poll later; robust offline; trivial in code.
   - Cons: needs a dedupe/ID scheme (use `sessionId` + `turnIndex` or `toolCallId`) and a reader/watcher; concurrent pi instances append to the same file (small `O_APPEND` writes are usually safe).
2. **Spawn a CLI**: `pi.exec("habitos", ["award", ...], {timeout, signal})` runs without a shell and returns `{stdout, stderr, code, killed, truncated}`. `timeout` default behavior is none; set one. Alternatively use `node:child_process` `spawn(..., {detached:true, stdio:"ignore"}).unref()` for fire-and-forget.
   - Pros: Habitos owns its schema and storage, so there is a single writer; `habitos` need not be running as a server.
   - Cons: process start cost per event; `pi.exec` is awaited, so it blocks the handler. Needs `habitos` on PATH, or an absolute path.
3. **HTTP to a local server**: global `fetch("http://127.0.0.1:PORT/…", {signal: ctx.signal})`. The docs show `fetch` in handlers, including with `ctx.signal`.
   - Pros: structured request/response; easy for a long-running Habitos TUI daemon.
   - Cons: the app must be listening, so you need a retry or queue fallback; awaiting adds latency.
4. **Unix domain socket or named pipe** (`node:net`): low overhead, no port conflicts. Same listener requirement as HTTP; the docs warn to open sockets in `session_start`, not in the factory, and close them in `session_shutdown`.
5. **Shared SQLite**: Node built-ins are available. The docs list `node:*` imports as available but show no SQLite example. Whether `node:sqlite` exists depends on the Node version running pi, and I did not verify it. A third-party driver would need an extension-local `npm install` (a native module like `better-sqlite3` complicates packaging).
   - Pros: transactional, queryable.
   - Cons: two writers need WAL and busy-timeout handling; schema coupling between pi extension and Habitos.
6. **Inverse direction (no extension at all):** Habitos reads pi's session JSONL files at `~/.pi/agent/sessions/--<path>--/<timestamp>_<id>.jsonl` (format in session-format.md), which contain user, assistant (with `usage`), toolResult and `custom` entries. Pros: zero install, history replay. Cons: file format is versioned (currently v3), no real-time "settled" marker, session entries can be branched (`/tree`) so naive counting may double-count.
7. **Persisting extension-side state:** `pi.appendEntry("habitos", data)` writes custom entries into the pi session (not seen by the LLM). Useful for an idempotency cursor ("already awarded turn N") that survives resume. Rebuild from `getBranch()` rather than all entries. (extensions.md "State Management")
8. **Inter-extension bus** `pi.events.emit/on` exists, but only between extensions inside pi, not an external app.
9. **Notify the user** with `ctx.ui.notify(msg, level)` / `ctx.ui.setStatus` / `setWidget` (guard with `ctx.hasUI`). The `notify.ts` example writes OSC terminal escapes to `process.stdout` (R/examples/extensions/notify.ts), which is possible in TUI mode but could interfere with the TUI; treat with care.

## Constraints and gotchas
- **No sandbox:** the extension has full permissions and can read prompts, credentials and history. Installing is a trust decision. Project-local `.pi/extensions` and `.pi/settings.json` packages load only after project trust. (extensions.md "Extension Locations"; packages.md)
- **Async and blocking:** handlers are awaited in load order. The docs say notification handlers "should not perform slow work on a streaming event". `provider_stream_event` handlers are awaited in stream order and delay consumption. Prefer `agent_settled` or `turn_end` and use short timeouts or fire-and-forget I/O. Wrap every handler in try/catch; errors are logged, not fatal (except `tool_call` errors, which block the tool).
- **`agent_end` is not "done":** it can fire several times per user prompt (retry, compaction retry, queued follow-ups). Use `agent_settled`, or dedupe on a per-prompt key. `agent_settled.aborted` flags Escape-cancelled runs, so award nothing if desired.
- **Idle semantics:** work requested from `agent_settled` (such as `sendUserMessage`) is deferred until all settled handlers finish, so dispatch is non-reentrant.
- **Parallel tools:** tool calls from one assistant message run in parallel by default; `tool_execution_end` fires in completion order, `tool_execution_start` in source order. Do not assume ordering.
- **Nested calls:** events carry `parentToolCallId` when a tool (for example a codemode script or subagent) made the call. Filter or count accordingly.
- **Subagents:** the `subagent/` example spawns sub-agents with `exec` (R/examples/extensions/subagent). Child pi processes would load the same extensions, so a Habitos extension could fire twice (parent + child). Not verified in the docs; check `AI_AGENT`/`PI_CODING_AGENT` env or `ctx.mode`.
- **Modes:** `print` (`-p`) and `json` have `hasUI=false`; extensions still run, but you cannot prompt. `ctx.shutdown()` is a no-op in print mode. In print mode the process exits when prompts finish, so use `session_shutdown` or `agent_settled` to flush and avoid unflushed async work (researcher inference; not stated in the docs).
- **Session replacement:** after `/new`, `/resume`, `/fork`, `/reload`, state is re-created; captured old `pi`/`ctx` objects become stale and throw. Do not cache `ctx` across session replacement. (extensions.md "Session replacement lifecycle and footguns")
- **Ephemeral sessions:** `getSessionFile()` may be undefined. Do not key idempotency on file path alone; use `getSessionId()`.
- **Message mutability:** `message_end`, `tool_result` and `before_provider_request` handlers can modify what other handlers see. Another extension loaded earlier can alter data. Observe-only handlers should not return values.
- **Usage/cost caveats:** `usage.cost` is derived from model config and may be zero or edited. Tool-nested model usage is in `tool_result.usage` / `toolResult.usage`, separate from main assistant usage. Compaction and cache-warming usage is in separate entries.
- **Git commits:** no first-class event. `bash` `tool_result.details` lacks an exit code; use `isError`, parse `content` text, or verify with `pi.exec("git", ["log", "-1", ...])`. User `!git commit` is visible only via `user_bash`. Commits made by external processes are invisible to pi.
- **Packaging:** do not list `@earendil-works/*` or `typebox` in `dependencies`. Declare them as `peerDependencies`. Extensions in `~/.pi/agent/extensions` need `npm install` in their own folder for third-party deps. (packages.md)
- **Version:** all of the above is from pi release 1.6.0 docs on this machine. The extension API is evolving (for example `agent_settled`, `agent_before_settle` and `turn_end` boundary semantics are recent), so pin against the installed version.

## Candidate signals for "a pi task is done" (no decision)
| Signal | What it means | Strengths | Weaknesses |
|---|---|---|---|
| `agent_settled` (`aborted:false`) | One user request fully finished and pi is idle | Single fire per prompt-run; official "final" signal; simple | Not an accuracy-of-success signal; fires for any prompt, even a trivial chat; no payload beyond `aborted`, so read `ctx.sessionManager` |
| `agent_end` | One low-level run ended | Gives `messages` directly | Can fire multiple times per prompt; not final |
| `turn_end` (`outcome:"completed"`, `toolResults.length>0`) | A turn with tool work finished | Per-turn granularity, has `message.usage` and `toolResults` | Many per task; mid-task |
| `tool_execution_end` / `tool_result` on `edit`/`write` (not `isError`) | File actually changed | Concrete work evidence; `durationMs` | Per-call noise; blocked/failed calls also fire end (`isError`) |
| `tool_result` on `bash` whose `input.command` matches `git commit` and not `isError` | A commit was made by the agent | Closest to "git commit" | Heuristic parsing of the command; no exit-code field; misses `!`-commands and external commits (see `user_bash`) |
| `pi.exec("git", ["rev-parse","HEAD"])` compare in `agent_settled` | A commit landed during the run | Reliable, independent of how it was made | Extra process; not valid outside a git repo; slight latency |
| `input` / `before_agent_start` (`messageType:"user"`) | A user task began | Task start timestamp and prompt text | Not completion |
| Assistant `message_end.usage` totals (tokens/cost) | Effort spent | Direct token/cost measure | Cost may be 0 for some providers; not a completion signal |
| `session_shutdown` (`reason:"quit"`) | Session ended | Good flush point (cf. `auto-commit-on-exit.ts`) | Only once per session; may be unclean on SIGKILL |
| Slash command or custom tool (`pi.registerCommand` / `registerTool`) | Explicit "task done" the user or LLM invokes | Explicit intent, no heuristics | Requires user/LLM discipline or prompt guidance; LLM may not call it |
| Reading session JSONL externally | Post-hoc analysis | No extension needed | No realtime settled marker |

## Sources
- Kept: `R/docs/extensions.md` — events, lifecycle, ctx API, `pi.exec`, modes, error handling, install locations.
- Kept: `R/docs/packages.md` — install, scope, package manifest, dependencies.
- Kept: `R/docs/message-types.md` — `Usage`, `AssistantMessage`, `ToolResultMessage` shapes.
- Kept: `R/docs/session-format.md` — session JSONL entry types, file location, `UsageEntry`.
- Kept: `R/docs/environment-variables.md` — `PI_*` session variables and process markers.
- Kept: `R/dist/core/extensions/types.d.ts` — exact event interfaces and the `ExtensionEvent` union.
- Kept: `R/dist/core/tools/bash.d.ts` — `BashToolInput`, `BashToolDetails` (no exit code field).
- Kept: `R/examples/extensions/notify.ts` — `agent_settled` as the done signal; OSC notification.
- Kept: `R/examples/extensions/auto-commit-on-exit.ts` — `session_shutdown` plus `pi.exec` git pattern.
- Not fully read (listed in extensions.md examples table only): `git-checkpoint.ts`, `git-merge-and-resolve.ts`, `subagent/`, `file-trigger.ts`, `event-bus.ts`.
- Not read: `sdk.md`, `rpc*.md`, `settings.md`, the README (only used indirectly). No web sources used.

## Missing evidence / uncertainties
- Line numbers were not captured; citations are by file and section.
- `node:sqlite` or any SQLite driver availability in pi's runtime: not verified.
- Whether subagent child processes load the same extensions (double-firing): not verified.
- Behavior of unflushed async I/O at process exit in print mode: not verified (inference only).
- Whether `ctx.sessionManager` or any API exposes aggregate session cost/tokens: not found; `/session` and the footer show totals but no extension API for them was found in what I read. Summing entries is the likely approach (inference).
- No contradictions found between the docs and `.d.ts` files in the parts read.

## Next steps
- Prototype a ~30-line extension: `agent_settled` → append one JSON line (sessionId, timestamp, aborted, tool counts, summed usage) to a Habitos-owned file, then verify it in TUI and `-p` modes.
- Read `sdk.md` and the RPC docs if Habitos might embed or drive pi rather than observe it.
- Check `examples/extensions/git-checkpoint.ts` and `subagent/` for the commit-detection and child-process behavior.

```acceptance-report
{
  "criteriaSatisfied": [
    {"id": "criterion-1", "status": "satisfied", "evidence": "Markdown brief with required sections written to the authoritative output path; claims cite local pi 1.6.0 docs, .d.ts files and examples; uncertainties listed."}
  ],
  "changedFiles": ["/Users/danielgarciasousa/.pi/agent/sessions/--Users-danielgarciasousa-Desktop-Proyectos-Habitos--/subagent-artifacts/outputs/38dbedac-78ac-4fbe-a37f-c230d9ded9d4/task-3.md"],
  "testsAddedOrUpdated": [],
  "commandsRun": [],
  "validationOutput": ["Read extensions.md, packages.md, message-types.md, session-format.md, environment-variables.md, types.d.ts, bash.d.ts, two examples; no source_check used (local primary docs only)."],
  "residualRisks": ["Line numbers not captured", "SQLite availability, subagent double-firing, print-mode flush behavior unverified", "sdk.md, rpc docs and README not read"],
  "noStagedFiles": true,
  "diffSummary": "Created one research markdown file.",
  "reviewFindings": ["no blockers"],
  "manualNotes": "Read-only research apart from the single requested output file. The Write tool was used to create it."
}
```
