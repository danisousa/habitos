# Code review reference

Read by every `code-reviewer` axis in its first turn.

## Finding card

Write in Spanish. Paths, identifiers, quotes from the spec, `/GLOSSARY.md` or a standard, and domain terms
stay verbatim, in their original language.

Number your findings 1, 2, 3… and write each one as a card of about 100 words, or about 150 for a **Decidir**
card; the number of cards has no cap. A card is markdown, in this shape:

```markdown
**<n> · <acción> · <Incumple|Criterio>** · `<path>:<line>`
<one paragraph of plain prose>
```

The paragraph is **plain**: written for a reader who has not seen the diff. Say what the code does before
naming any identifier, and tell the consequence in what the app or the data would show. Quote the standard,
spec line or glossary text the finding rests on, when there is one, and name the smell for a smell. A
technical term absent from `/GLOSSARY.md` gets half a sentence of explanation. End on what you propose and the
priority behind it, in the same prose.

A card that claims behaviour changed — "antes hacía X, ahora no" — cites both sides in the paragraph: the code
before as `path@<merge-base>:line`, read with `git show <merge-base>:<path>`, and the code after as
`path:line`. Read the old code before writing the claim, and report a change only when you can point at both
sides.

The action is one of four:

- **Arreglar** — a concrete change on this branch; name it.
- **Decidir** — two or more reasonable paths, in the shape below.
- **Issue aparte** — real, but outside this change; say what the issue would ask for.
- **Dejar** — leave it as it is. A Dejar is one line instead of a card:
  `**<n> · Dejar** · \`<path>:<line>\` — <the finding> · <why touching it is not worth it> · prioridad: <prioridad>`.

An axis with nothing to report writes only `Sin hallazgos.`

## Prioridades

Every proposal names one priority, from this list. When two priorities pull apart, the higher one wins:

1. **dominio** — the domain rules are right (`/GLOSSARY.md`, `docs/adr/`).
2. **reversibilidad** — what it costs to undo: migrations, production data, anything published.
3. **contrato** — what the issue, spec or an ADR says.
4. **alcance** — the PR stays one atomic change; what lies outside it goes to an Issue aparte.
5. **mantenibilidad** — documented standards, one source of truth, fewer moving parts.
6. **coste** — tokens, CI minutes, run time.

A priority that fits none of these is written `otro: <name>`. To propose against the order, name the
priority it overrides and why: `prioridad: coste (salta contrato: <por qué>)`.

## Decidir

After the paragraph, number the options. Under each, one line per priority on which the options differ — leave
out a priority that reads the same for all of them. Then the preferred option, and why the call is the
developer's:

```markdown
1. **<option>.**
   - <prioridad>: <what this option means for it>.
2. **<option>.**
   - <prioridad>: <what this option means for it>.

**Me inclino por la <n> — prioridad: <prioridad>** (salta <prioridad>: <por qué>, only when it overrides the order).
**Por qué decides tú:** <why the call is not the reviewer's>.
```
