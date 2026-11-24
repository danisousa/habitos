---
name: code-review
description: Review the changes since a fixed point (commit, branch, tag, or merge-base) along two axes — Standards (does the code follow this repo's documented coding standards?) and Spec (does the code match what the originating issue/PRD asked for?). Runs the reviews in parallel sub-agents and reports them side by side. Use when the user wants to review a branch, a PR, work-in-progress changes, or asks to "review since X".
metadata:
  upstream:
    repo: mattpocock/skills
    path: skills/engineering/code-review
    commit: abeeaf9
---

Two-axis review of the diff between `HEAD` and a fixed point the user supplies:

- **Standards** — does the code conform to this repo's documented coding standards?
- **Spec** — does the code faithfully implement the originating issue / PRD / spec?

The axes run as **parallel sub-agents** so they don't pollute each other's context, then this skill verifies each card against the code and aggregates the findings.

The issue tracker should have been provided to you — if `docs/agents/issue-tracker.md` is missing, tell the user to run `/skill:setup-matt-pocock-skills`.

## Process

### 1. Pin the fixed point

Whatever the user said is the fixed point — a commit SHA, branch name, tag, `main`, `HEAD~5`, etc. If they didn't specify one, ask for it.

Capture the diff command once: `git diff <fixed-point>...HEAD` (three-dot, so the comparison is against the merge-base). Also note the list of commits via `git log <fixed-point>..HEAD --oneline` and the merge-base via `git merge-base <fixed-point> HEAD` — both go in the report's header.

Before going further, confirm the fixed point resolves (`git rev-parse <fixed-point>`) and the diff is non-empty. A bad ref or empty diff should fail here — not inside the parallel sub-agents.

### 2. Identify the spec source

Look for the originating spec, in this order:

1. Issue references in the commit messages (`#123`, `Closes #45`, GitLab `!67`, etc.) — fetch via the workflow in `docs/agents/issue-tracker.md`.
2. A path the user passed as an argument.
3. A PRD/spec file under `docs/`, `specs/`, or `.scratch/` matching the branch name or feature.
4. If nothing is found, ask the user where the spec is. If they say there isn't one, the **Spec** sub-agent will skip and report "no spec available".

### 3. Identify the standards sources

Anything in the repo that documents how code should be written — `CODING_STANDARDS.md`, `CONTRIBUTING.md`, or whatever the repo actually keeps. Here that is `AGENTS.md` and the topic files it points at: pass the ones the diff touches.

On top of whatever the repo documents, the Standards axis always carries the **smell baseline** — a fixed set of Fowler code smells that applies even when a repo documents nothing. It lives in [`smell-baseline.md`](./smell-baseline.md), with the rules that bind it.

### 4. Spawn the sub-agents in parallel

If only `subagents_enable` is visible, call it first. Then launch one `subagent({ workflow: "parallel", args: { tasks: [...] } })` with one task per axis, each with `agent: "code-reviewer"` and an explicit `cwd`. That agent pins its model and a read-only `tools` allowlist in `.pi/agents/code-reviewer.md`.

Every axis writes its findings as the **finding card** in [`reference.md`](./reference.md); the `code-reviewer` agent reads that file itself in its first turn, so no prompt carries its text. Each sub-agent writes its own cards — it is the one that read the code, so it is the one that can name the fix. Each brief says which of its findings take which tag.

Each prompt is **closed**: it holds exactly the items listed for its axis below. The implementer's decisions reach an axis through the diff and the commit messages; a focus request the developer made when invoking this skill goes to step 5.

**Standards sub-agent prompt** — exactly:

- The diff command, the merge-base SHA and the commit list.
- The list of standards-source files you found in step 3, plus `.agents/skills/code-review/smell-baseline.md` to read and apply.
- The brief: "Report (a) every place the diff violates a documented standard, tagged `Incumple`, quoting the standard (file + the rule); and (b) any baseline smell you spot, tagged `Criterio`, naming the smell and quoting the hunk. A documented repo standard overrides the baseline. Skip anything tooling enforces."

**Spec sub-agent prompt** — exactly:

- The diff command, the merge-base SHA and the commit list.
- The path or fetched contents of the spec.
- The brief: "Report: (a) requirements the spec asked for that are missing or partial, tagged `Incumple`; (b) behaviour in the diff that wasn't asked for (scope creep), tagged `Criterio`; (c) requirements that look implemented but where the implementation looks wrong, tagged `Incumple`. Quote the spec line for each finding."

If the spec is missing, skip the Spec sub-agent and note this in the final report.

### 5. Verify

Each axis hands back in its own turn: verify its cards in that turn, and close the turn with one status line
that holds no findings — `<axis> verificado; faltan <axes>`. The findings reach the developer in the step 6
report, written once every axis is in.

Check every card against the code before it reaches the developer. An axis can be wrong about the code, and a
false claim costs more than the card is worth. For each card, find the factual claim its proposal rests
on and open the cited `path:line`. For a claim that behaviour changed, also open the code before it with
`git show <merge-base>:<path>`, and add any side the card leaves out — a card citing one side only cannot be
`CONFIRMED`. Then tag the card with one verdict:

- **`CONFIRMED`** — the code shows the claim holds.
- **`PLAUSIBLE`** — the code is consistent with the claim, but reading it cannot settle it (runtime data, a
  judgement call, a path you cannot follow to the end).
- **`REFUTED`** — the code contradicts the claim. Rewrite the card on what the code does show, and verify the
  rewrite. If nothing stands, withdraw the card.

Add the verdict to the card's first line: `**<n> · <acción> · <Incumple|Criterio> · <CONFIRMED|PLAUSIBLE>** · \`<path>:<line>\``,
or `**<n> · Dejar · <CONFIRMED|PLAUSIBLE>**` on a Dejar line. A rewritten card keeps its number and opens its
paragraph with `(reescrita: <what was wrong>)`. Every standing card ends on a trailing line,
`_Verificado: <what you opened>_`, as `path:line` or `path@<sha>:line`; on a `PLAUSIBLE` card it also says what
stayed unchecked. It does not count towards the card's word cap. A Dejar line carries its verdict alone. A withdrawn card keeps its number as one line,
`**<n> · retirada** — <the claim> · <what the code shows>`, and never appears in _Qué te toca_.

If the developer named something to look at when invoking this skill, check it here, against the code, as you
check a card. When it shows a finding, write it as a card of your own under the axis it belongs to, numbered
after that axis's cards, with `· orquestador` after the verdict.

### 6. Aggregate

The report is for the developer, who decides what happens to each finding. Write it in Spanish, in three parts.

**1. Revisado contra** — what steps 1–3 and 5 gathered, so the reader knows the yardstick before any finding:

```markdown
**Revisado contra**
- **Base:** `<fixed point>` @ `<merge-base short SHA>` · <n> commits
- **Spec:** #<N> "<title>" — or the path, or `ninguna`
- **Normas:** <standards files passed> + smell baseline
- **Ejes:** Standards ✓ · Spec ✓ — a skipped axis reads `no corre: <reason>`
- **Verificación:** <n> CONFIRMED · <n> PLAUSIBLE · <n> reescritas · <n> retiradas
```

**2. The axes** — `## Standards` and `## Spec`, each holding that axis's cards as verified in step 5, in its own numbering. Keep the axes apart, in that order, with no merging or reranking of findings — the axes are deliberately separate (see _Why two axes_). A skipped axis keeps its heading, with the reason it was skipped.

**3. Qué te toca** — an index grouped by action, pointing at cards as `<axis> <n>`, axes always in the order Standards, Spec. Every group carries **one proposed action**, which is what the developer approves.

First, **link** the standing cards across axes: two cards from different axes are linked when they point at the same underlying problem. A shared `path` is a hint to compare them; the problem is the criterion.

Each card's priority is the one its proposal names. Between linked cards, the higher priority in the order of _Prioridades_ in [`reference.md`](./reference.md) wins, and on a tie the first card in axis order.

- **Duplicate** — linked cards with the same action. One entry, under that action, joined with `=`, proposing the winning card's fix (or, for Decidir, its preferred option): `Standards 3 = Spec 1`.
- **Conflict** — linked cards with different actions. One Decidir entry, joined with `⇄`; its options are the cards' actions, numbered in entry order, and the winning card's is preferred: `Standards 1 ⇄ Spec 1 (→ la 2, prioridad: contrato)`.

A linked card appears only in its linked entry, and a reply naming any of its cards answers the whole entry. Linking lives in the index alone: the axis sections and the cards stay as step 5 left them.

```markdown
## Qué te toca
- **Arreglar (1):** Standards 2
- **Decidir (2):** Spec 2 (→ la 1) · Standards 1 ⇄ Spec 1 (→ la 2, prioridad: contrato)
- **Issue aparte (1):** Standards 3 = Spec 1
- **Dejar (3)**

`adelante` aprueba esto tal cual · o p. ej. `adelante salvo Spec 2: la 2`
```

A Decidir card shows its preferred option as `(→ la <n>)`. Dejar is a count only. List only the actions that have cards. With no findings on any axis, the index reads `Nada que hacer.`

Then stop and wait for the developer's reply; nothing is applied before it. `adelante` approves the index as written. Any other reply ("Spec 3: la 1", "adelante salvo Spec 2: la 2", "Standards 4 no") changes only the entries it names, and the rest stand as proposed. Dejar needs no approval. Apply what was approved as new commits, and file each approved Issue aparte on the tracker, per `docs/agents/issue-tracker.md`.

## Why two axes

A change can pass one axis and fail another:

- Code that follows every standard but implements the wrong thing → **Standards pass, Spec fail.**
- Code that does exactly what the issue asked but breaks the project's conventions → **Spec pass, Standards fail.**

Reporting them separately stops one axis from masking another.
