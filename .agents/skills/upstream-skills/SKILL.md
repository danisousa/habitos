---
name: upstream-skills
description: Review upstream changes to the adapted skills, or adopt a skill into that regime.
disable-model-invocation: true
---

An **adapted skill** is a copy in `.agents/skills/` that this repo owns and may edit. It records its origin in its `SKILL.md` frontmatter and has no entry in `skills-lock.json`, so `npx skills update` leaves it alone:

```yaml
metadata:
  upstream:
    repo: mattpocock/skills
    path: skills/productivity/grilling
    commit: 697d4ce
```

`commit` is the last upstream commit reviewed. Every adapted skill comes from `mattpocock/skills`.

No arguments: run **Review**. `adopt <skill>`: run **Adopt**.

Start both from a temporary clone. Shell state does not persist between calls, so set `U`, and any other variable a snippet reads (`name`, `src`, `commit`), in each call:

```bash
U=$(mktemp -d)/skills
git clone --quiet --filter=blob:none https://github.com/mattpocock/skills "$U" && echo "$U"
```

## Review

Compare upstream against itself, from `commit` to HEAD. The local copy differs on purpose, so a diff against it shows adaptations, not news.

1. List what moved:

   ```bash
   for f in $(grep -l '^  upstream:' .agents/skills/*/SKILL.md); do
     src=$(awk '/^    path:/{print $2; exit}' "$f")
     commit=$(awk '/^    commit:/{print $2; exit}' "$f")
     echo "== $f $commit"
     git -C "$U" log --oneline "$commit"..HEAD -- "$src"
   done
   ```

   A `path` missing at HEAD means the skill was renamed or deleted: read the commit that removed it and put that to the user. On a rename, set `path` to the new location before diffing.

2. For each skill that moved, read `git -C "$U" diff "$commit"..HEAD -- "$src"` beside the local copy of each file it touches. Classify every change:
   - **noise**: wording or punctuation, same behaviour. Left unapplied.
   - **adopt**: improves behaviour and applies as written.
   - **adapt**: worth having, but must be rewritten for this repo, such as a file name, a label or a workflow rule that differs here.
   - **reject**: conflicts with a local adaptation or a repo rule, or adds nothing here.

   Done when every hunk of every diff has a class.

3. Group noise into one line per skill, stated once. Then put every adopt, adapt and reject to the user one decision at a time: what the skill does today, what upstream changes, your recommendation and its reason. Wait for the answer before the next.

4. Apply the accepted changes to the local copies, keeping their adaptations. Done when every adopt and adapt decision is in the local file.

5. Set `commit` to the short upstream HEAD on every skill with `metadata.upstream`, moved or not, including those with only rejected or noise changes, so a settled change never comes back.

6. Report each rejected change with its reason, ready for the PR body.

## Adopt

If the skill already has `metadata.upstream`, say so and stop. Done when the skill has `metadata.upstream` as its last frontmatter key, and no lock entry.

### Installed skill

1. Take `src`, the value for `path`, from the skill's `skillPath` in `skills-lock.json`, without `/SKILL.md`.

2. Find the baseline, the upstream commit whose directory equals the first version committed here:

   ```bash
   first=$(git log --diff-filter=A --format=%h -- ".agents/skills/$name/SKILL.md" | tail -1)
   tree=$(git rev-parse "$first:.agents/skills/$name")
   for c in $(git -C "$U" log --format=%h -- "$src"); do
     [ "$(git -C "$U" rev-parse "$c:$src" 2>/dev/null)" = "$tree" ] && { echo "$c"; break; }
   done
   ```

   No output means no upstream version matches: put that to the user rather than pick a commit.

3. Add `metadata.upstream` with that commit.

4. Remove the lock entry:

   ```bash
   node -e 'const f="skills-lock.json",l=JSON.parse(fs.readFileSync(f));delete l.skills[process.argv[1]];fs.writeFileSync(f,JSON.stringify(l,null,2)+"\n")' "$name"
   ```

### Skill not yet installed

1. Find it with `find "$U/skills" -type d -name "$name"`. Its path relative to `$U` is `src`, the value for `path`. Anything but one match goes to the user.

2. Copy it:

   ```bash
   cp -R "$U/$src" ".agents/skills/$name"
   ```

3. Add `metadata.upstream`, with `commit` from `git -C "$U" rev-parse --short HEAD`.
