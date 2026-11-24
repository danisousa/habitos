---
name: retro
description: "Conduct a retrospective on a coding session."
disable-model-invocation: true
metadata:
  upstream:
    repo: mattpocock/skills
    path: skills/engineering/retro
    commit: abeeaf9
---

The user has asked for a **retrospective**. You are suggesting improvements to the coding agent's **environment** to improve future runs.

## Steps

1. Read `.agents/skills/writing-for-agents/SKILL.md` for the writing style guide.

2. Review the current session only. If its context was compacted, read the compacted part from its log: the newest `.jsonl` in `~/.pi/agent/sessions/`, under the folder named after this repo's path.

3. Look for candidates for improvement in these categories.

- **Navigation**: how easy was it for the agent to find the right files? Are there hidden dependencies between files? Would a **navigation pointer** make it easier? _Use when_ the session took a long time to find a piece of information.
- **Automated checks**: are there automated checks that could catch errors the agent made? Linting, typing, tests, filesystem linters? Read the repo's own check command first (its `package.json`/build-tool `lint`/`check` scripts, its CI workflow), so a check that already exists but sits unwired or silently broken is the finding, not a reinvention. A repo with no **guardrail** (no pre-commit hook and no CI job running its lint/typecheck/test command) is itself a finding: an un-linted repo is a standing missed opportunity, not a neutral default. _Use when_ the agent made a mistake an automated check could have caught, or the repo has no guardrail at all.
- **Coding standards**: should the **reviewer agent** be given a new rule to enforce? Should an existing rule be removed or clarified? Classify the violation first: a **mechanical** one (a fixed syntactic pattern, a banned API, an import shape, a file-location rule) gets a deterministic check, full stop: a custom rule in the repo's own linter, a new pre-commit hook, or a new CI job, whichever the repo's language and existing guardrail make cheapest. Default to building the check over writing the rule. Reserve `AGENTS.md` and the topic files it points at, which `/skill:code-review` reads, for genuine **judgement calls** (cross-file consistency, "matches the surrounding style," anything no guardrail could ever substitute for). _Use when_ the reviewer agent failed to catch a mistake.
- **Global AGENTS.md**: are there any steering instructions that should be moved to coding standards (or automated checks) instead? _Use when_ the AGENTS.md file is particularly large - in the repo OR the user's global scope (`~/.pi/agent/AGENTS.md`).
- **Harness**: would a change to Pi itself have avoided the friction? Settings (`~/.pi/agent/settings.json`, `.pi/settings.json`), extensions (`~/.pi/agent/extensions/`), subagent definitions and overrides, installed packages. _Use when_ the friction came from the harness rather than the repo: a guard that blocked too much or too little, a subagent on the wrong model or tools, a missing command.
- **Communication**: did the agent follow the acting-vs-asking and response rules in `~/.pi/agent/AGENTS.md`? Asked when it should have acted, acted when it should have asked, padded or unclear replies. _Use when_ the user corrected, repeated or clarified a question or a reply.
- **Tool economy**: did the agent make expensive tool calls that could be streamlined? Is there any custom tooling (CLI's, MCP's) that is particularly token-inefficient? _Use when_ the agent made an expensive tool call.
- **No-ops**: look for instructions in steering files that don't modify the agent's behavior. _Use when_ the steering files are large and unwieldy.
- **Information access**: look for opportunities to increase the agent's access to information. Teeing dev server logs, readonly access to third-party services. _Use when_ a crucial piece of information was not available to the agent.

4. Present these candidates to the user, in order of severity.

5. File each candidate the user accepts as an issue labelled `harness`, following `docs/agents/issue-tracker.md`. When an open `harness` issue already covers it, comment on that issue instead. Done when every accepted candidate has an issue or a comment.

## Reference

### Implementation vs Review

Remember that all work goes through two stages: implementation and review. The implementation agent has the most **context pressure**. They are responsible for exploration, writing code, and debugging failures.

The review agent has the least context pressure - it receives a diff, so no exploration needed. It often does not need to write code or debug.

This means that the review agent should be responsible for imposing coding standards, not the implementation agent.

### Files

You have access to several files in the repo:

- `AGENTS.md`: this file is pushed to the context window of any agent working in this repo. It should be used incredibly sparingly, usually only for **navigation pointers** to other files.
- `docs/agents/*.md`: the topic files `AGENTS.md` points at. Add **navigation pointers** to docs folders if one gets more than 1,000 lines long.
- Docs: use docs as references files, pointed to by other files. Look for existing docs before writing new ones.
- Skills: use skills for docs (since their description goes into the agent's context window), or for user-invoked commands. Follow the advice in the `writing-for-agents` skill.
