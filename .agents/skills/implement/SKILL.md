---
name: implement
description: "Implement a piece of work based on a spec or set of tickets."
disable-model-invocation: true
metadata:
  upstream:
    repo: mattpocock/skills
    path: skills/engineering/implement
    commit: abeeaf9
---

Implement the work described by the user in the spec or tickets.

## Re-anchor before any edit

The issue is a contract written against an older tree (`create-issue` stamps `Verified against <sha>` as the
first Context line). The plan is built here, on the tree that will actually be edited — never earlier, never
persisted to the issue.

1. `gh issue view <N> --json body` — take the stamped `<sha>`, every cited path, and every anchor (symbol,
   test name, route, table/column, quoted phrase).
2. `git log --oneline <sha>..HEAD -- <cited paths>` — what landed on those files since the issue was verified.
   Read any commit that touches a cited path.
3. One grep per anchor: `grep -rn '<anchor>' <path>`. Every anchor must resolve.
4. A missing or moved anchor, or a Context claim the commits from step 2 falsified, updates the issue body
   (`gh issue edit <N> --body`) **before** the first edit — the contract has to be true on the tree the PR
   will be reviewed against. If the fix reopens a product question, stop and hand back instead.
5. Write the plan in-session: the seams `/skill:tdd` will test first, the order of edits. It stays in the
   conversation. The PR reports the delta from the DoD (`create-pr`), not the plan.

An initiative ticket or spec with no cited paths has nothing to re-anchor — go straight to the plan.

## Build

If the work touches UI-rendering paths and `docs/agents/frontend-design-workflow.md` exists, follow it before
writing UI code.

Read the `tdd` skill (`.agents/skills/tdd/SKILL.md`) and follow it where possible, at pre-agreed seams.

Run typechecking regularly, single test files regularly, and the full test suite once at the end.

A deviation decided while building — a rule the issue left open or stated differently — updates `/GLOSSARY.md` when it
changes a domain rule, and every open ticket that consumes it: search the initiative's Tickets table, or
`gh issue list --search`, for the rule's term.

Once done, commit your work to the current branch, then read the `code-review` skill (`.agents/skills/code-review/SKILL.md`) and follow it to review it.
Review fixes land as new commits.
