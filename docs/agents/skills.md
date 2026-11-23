# Skills

Habitos owns every skill it uses, in `.agents/skills/` (see `docs/adr/0001-per-project-skills.md`).

## Classes

Each skill has one class, which fixes how it is updated:

- **Adapted**: a copy of an upstream skill that this repo edits. Its `SKILL.md` frontmatter ends with `metadata.upstream` (repo, path, last reviewed commit). Updated with `/skill:upstream-skills`, which also adopts new ones.
- **Own**: written here, no upstream. Updated by editing it.
- **Third-party**: installed unmodified with `npx skills add`, tracked in `skills-lock.json`, updated with `npx skills update`. Editing one makes it adapted: adopt it with `/skill:upstream-skills adopt <name>`.
- **Project**: specific to Habitos's stack or domain. An own skill that would make no sense in another repo.

## Adapting a skill to this repo

- **Extension point**: when the repo only supplies data the skill already expects (labels, paths, extra review axes), put it in `docs/agents/<skill>.md` and leave the skill as it is.
- **Composed skill**: when the approach changes, write a new skill under a new name that invokes the original. Where they conflict, the composed skill wins.
- Name every skill uniquely: a second skill with an existing name shadows it, and Pi keeps whichever it finds first.

To invoke another skill from a skill, read `.agents/skills/<name>/SKILL.md` and follow it. This works for user-invoked skills too, which the agent cannot fire on its own.
