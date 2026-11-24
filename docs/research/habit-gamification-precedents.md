# Habit-gamification prior art and what transfers to Habitos

Scope note: web search was rate-limited (Exa 429) and several sites returned 403 (Habitica wiki, Finch help center). Evidence is thinner for Habitica, Finch and Forest than for Duolingo, Beeminder, Loop, Harsh and Dijo. Gaps are listed explicitly. Labels: **[direct]** = stated by the source; **[interp]** = my reading of a source; **[inference]** = my own reasoning; **[anecdotal]** = community report.

## Summary
- **Make the streak cheap to extend, and forgive misses.** Duolingo decoupled the streak from the daily goal so one lesson keeps it alive. Its A/B test showed +3.3% day-14 retention and +10.5% of daily learners on a streak. It also found streak freezes raise activity ([improving the streak](https://blog.duolingo.com/improving-the-streak/), [streak research](https://blog.duolingo.com/how-duolingo-streak-builds-habit/)).
- **Loss aversion works but has a cost.** Duolingo says loss aversion drives long streaks, and that a broken streak can demotivate and deter people from starting one. Loop's habit score is built to avoid the "one miss wipes everything" failure ([Duolingo](https://blog.duolingo.com/how-duolingo-streak-builds-habit/), [Loop](https://github.com/iSoron/uhabits)).
- **Every self-report system is an honor system.** Habitica says outright that it cannot verify completion ([Habitica FAQ](https://habitica.fandom.com/wiki/FAQ)). Anecdotally, gaming the checkbox is the most common way trackers die ([Reddit, anecdotal](https://www.reddit.com/r/getdisciplined/comments/1rfbuvr/been_through_like_15_habit_tracking_apps_whats/)). For a single-user game, cheating is only "you cheat yourself", so design for honesty-friendliness rather than anti-cheat.
- **Real stakes and hard commitment suit a niche.** Beeminder uses money and a one-week-horizon commitment ([about](https://www.beeminder.com/about)). Forest uses a withering tree for leaving the app ([Play Store](https://play.google.com/store/apps/details?hl=en&id=cc.forestapp)). Both work because failure has a visible cost.
- **Terminal habit tools succeed by being low-friction, plain-text and scriptable.** Harsh offers y/n/skip, urgency warnings and text files. Dijo offers vim motions and scripted auto-habits ([Harsh](https://github.com/wakatara/harsh), [Dijo](https://github.com/NerdyPepper/dijo)).
- **Pet and RPG layers pay out in a separate currency.** Finch ties daily energy and rainbow stones to a pet. Habitica ties HP/XP/gold to an avatar and loses HP when dailies are missed. This gives the game depth without changing what counts as a habit ([Finch](https://finchcare.com/about-finch), [Habitica case study, secondary](https://gamificationexplained.com/product-review/case-study-habitica-turning-your-life-into-a-role-playing-game/)).
- **Early novelty fades and the motivation shifts.** Duolingo says the streak is exciting at 2→3 days (+50%), and by 200→201 (+0.5%) it works through loss aversion instead. Habitos therefore needs mid-game content, not only early-game flash ([Duolingo](https://blog.duolingo.com/how-duolingo-streak-builds-habit/)).

## Per-product findings

### Duolingo (streaks)
- **[direct]** A streak counts consecutive days with a completed lesson. Learners at a 7-day streak are 2.4× more likely to return the next day ([improving the streak](https://blog.duolingo.com/improving-the-streak/)). Reaching a 7-day streak makes learners 3.6× more likely to complete the course ([streak research](https://blog.duolingo.com/how-duolingo-streak-builds-habit/)). Both figures are correlational, and the sources state them that way.
- **[direct]** The old rule required hitting the daily goal to extend the streak. Learners with the "intense" goal were least likely to be on a streak. Decoupling (one lesson extends the streak, goal progress shown separately) gave +3.3% day-14 retention, +1% DAU and +19% on-streak among new learners. Fewer people hit their goal, but more built a daily habit. Duolingo concluded that lowering the barrier matters more than how much is learned per day ([improving the streak](https://blog.duolingo.com/improving-the-streak/)).
- **[direct]** A streak-extend animation raised new-learner 7-day retention by +1.7%. Allowing two equipped Streak Freezes raised daily active learners by +0.38%. Duolingo cites a Penn/UCLA study that "slack" can motivate more than rigid rules ([streak research](https://blog.duolingo.com/how-duolingo-streak-builds-habit/)).
- **[direct]** Duolingo says a broken streak can be demotivating, and fear of losing one can stop people from starting ([streak research](https://blog.duolingo.com/how-duolingo-streak-builds-habit/)).
- **Caveat:** these are the vendor's own A/B results, with no independent replication. The freeze can be bought with gems, as the blog screenshot shows.

### Habitica (RPG to-do)
- **[direct, maker]** Habitica says rewards give instant gratification (XP, gold, random drops such as eggs) ([press kit](https://habitica.com/static/press-kit)).
- **[secondary]** Missing dailies or doing bad habits costs HP. At 0 HP the avatar dies and loses a level and a piece of equipment ([case study](https://gamificationexplained.com/product-review/case-study-habitica-turning-your-life-into-a-role-playing-game/)). I could not fetch the official wiki page on death, so this is not confirmed from Habitica itself.
- **[direct, maker]** There is no verification: "we believe in the honor system because... you only hurt yourself if you lie" ([FAQ](https://habitica.fandom.com/wiki/FAQ), [Obstacles](https://habitica.fandom.com/wiki/Obstacles)). This is wiki-hosted text, and I treat it as the community-maintained official line.
- **[anecdotal]** Some players say cheaters lose interest because the game loses its point ([r/habitica](https://www.reddit.com/r/habitica/comments/h8vvh9/do_people_take_the_game_seriously/)). Others report gaming the checkbox in Habitica-style apps until the data "becomes fiction" ([r/getdisciplined](https://www.reddit.com/r/getdisciplined/comments/1rfbuvr/been_through_like_15_habit_tracking_apps_whats/)).
- **Gap:** I found no primary Habitica source on burnout, dropout rates, or the pre-2017 death-spiral and party-damage design discussions.

### Finch (self-care pet)
- **[direct]** The pet grows as you care for yourself. Goals and self-care activities earn Energy, which the pet spends on adventures. Rainbow Stones are a separate currency ([about](https://finchcare.com/about-finch), [help center via search snippet](https://help.finchcare.com/hc/en-us/articles/37780134479757-Energy-vs-Rainbow-Stones)).
- **[fan wiki]** Energy resets to zero at the start of the pet's day. Stones buy clothing, furniture, colors and travel, and can also pay to repair a streak ([Energy](https://finch.fandom.com/wiki/Energy), [Rainbow Stones](https://finch.fandom.com/wiki/Rainbow_Stones)). This is search-snippet level only, because the pages returned 403 when fetched.
- **[inference]** A positive-only pet (no death) plus an optional streak-repair sink is the soft-loss end of the design spectrum. I have no primary data on Finch's retention.

### Forest (focus timer)
- **[direct, maker/store]** Start a timer and a seed grows into a tree. Leaving the app makes the tree wither ([Play Store](https://play.google.com/store/apps/details?hl=en&id=cc.forestapp)). Real trees are funded through Trees for the Future ([trees.org](https://trees.org/support/corporate-sponsors/forest-app)).
- **[inference]** It works because the loss is immediate, visible and tied to one short session. It is a focus tool rather than a daily-habit tool, so it transfers to Habitos mainly as a "session" mechanic.
- **Gap:** no primary data on retention or burnout.

### Beeminder (commitment contracts)
- **[direct]** Keep your datapoints on the good side of the "Bright Red Line" or pay a pledge. Derailing resets the line at your current value. You choose the respite period and the pledge, which usually rises after each derail ([about](https://www.beeminder.com/about)).
- **[direct]** "Flexible self-control": commitments apply only on a one-week horizon, so you can always quit or change a goal, but only a week from now ([about, linking to flexbind post](https://www.beeminder.com/about)).
- **[direct]** Data smoothing and a trend "aura" are used so day-to-day noise doesn't discourage you ([about](https://www.beeminder.com/about)).
- **Gap:** I could not fetch Beeminder's own cheating/fudging-data policy (the /cheating URL returned 404).

### Streaks (iOS, Crunchy Bagel)
- **[direct]** Up to 24 daily tasks, "don't break the chain". It has no accounts and no ads, and data stays on-device or in iCloud. Task sharing lets others see your progress in real time ([press](https://streaks.app/press/), [Streaks 7](https://crunchybagel.com/now-available-streaks-7/), [privacy](https://streaks.app/privacy/)).
- **[interp]** The hard cap of 24 tasks is a deliberate anti-bloat constraint. It is not documented as a design rationale.

### Loop Habit Tracker
- **[direct]** The "habit score" works like a strength meter: every repetition strengthens it, every miss weakens it, and "a few missed days after a long streak... will not completely destroy your progress, unlike many other don't-break-the-chain apps". It supports flexible schedules such as 3 times per week ([repo README](https://github.com/iSoron/uhabits)).
- **[interp]** This is an explicit, shipped alternative to the binary streak.

### Harsh (terminal)
- **[direct]** Three commands (`ask`, `log`, `todo`). Per-day answers are y/n/skip. Skip "doesn't break streak" and `n` does. Frequencies can be rolling windows like `3/7`. Habits can be marked track-only or retired with an end date. Data lives in plain text files. It renders a Seinfeld-style consistency graph with a sparkline and a score that excludes skipped days. `todo` shows urgency such as "2 days before streak breaks" ([README](https://github.com/wakatara/harsh), [launch post](http://daryl.wakatara.com/harsh-a-minimalist-cli-habit-tracker/)).

### Dijo (terminal)
- **[direct]** A curses habit tracker with vim-like motions, modal day/week views and a command mode (`:add`, `:delete`). It is scriptable: external programs can report habits automatically, for example git commits ([README](https://github.com/NerdyPepper/dijo)).

### Research
- **[direct]** Lally et al. 2010 followed 96 volunteers doing a daily behaviour in a consistent context, modelling how automaticity develops ([Wiley abstract](https://onlinelibrary.wiley.com/doi/10.1002/ejsp.674?538fcd4b_page=)). The BPS digest and a secondary summary describe a wide range of times to automaticity and a fitted curve for only some participants ([BPS](https://www.bps.org.uk/research-digest/how-form-habit), [summary, secondary](https://sorushi.com/learn/research/lally-2010)). I could not confirm the exact "66 days" or "missing one day doesn't matter" figures in the text I retrieved. Verify them before citing.
- **[direct]** Gardner, Lally and Wardle describe habits as actions triggered automatically by contextual cues ([PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC3505409/)).

## Cross-cutting patterns

### What kept people engaged
1. **Low-effort daily action with a visible counter.** Duolingo's one-lesson streak and Harsh's y/n/skip prompt both follow this ([Duolingo](https://blog.duolingo.com/improving-the-streak/), [Harsh](https://github.com/wakatara/harsh)).
2. **Satisfying feedback at the moment of completion**, especially early on and at milestones ([Duolingo](https://blog.duolingo.com/how-duolingo-streak-builds-habit/)).
3. **Slack mechanisms.** Duolingo freezes, Harsh skip, Loop's decaying score and Beeminder's respite all let a miss be absorbed.
4. **A progression layer separate from the habit itself.** Habitica XP/gold/drops and Finch energy/stones/pet fall here ([Habitica](https://habitica.com/static/press-kit), [Finch](https://finchcare.com/about-finch)).
5. **Frictionless, owned data and automation.** Harsh's text files, Dijo's scripting hooks and Streaks' no-account design ([Dijo](https://github.com/NerdyPepper/dijo), [Streaks](https://streaks.app/privacy/)).

### What burned users out
- **Rigid all-or-nothing streaks.** Duolingo says the break itself demotivates, and the fear of losing one deters starting. Loop's README names this as the problem with don't-break-the-chain apps. **[direct]** ([Duolingo](https://blog.duolingo.com/how-duolingo-streak-builds-habit/), [Loop](https://github.com/iSoron/uhabits))
- **Goals set too high.** Duolingo found high daily goals linked to fewer streaks. **[direct]** ([improving the streak](https://blog.duolingo.com/improving-the-streak/))
- **Punitive loss (HP loss, death, level loss).** Habitica's design is documented only through a secondary source, and I found no primary burnout data. **[inference]** The risk is a death spiral where falling behind makes the game feel worse, so people quit.
- **Novelty decay.** The Duolingo post says the streak's reward shifts from excitement to avoidance as it grows. **[direct]**
- **[anecdotal]** Users report trackers working for about two weeks, after which the data degrades into fiction ([r/getdisciplined](https://www.reddit.com/r/getdisciplined/comments/1rfbuvr/been_through_like_15_habit_tracking_apps_whats/)). This is one thread and not representative.

### What invited cheating
- **Self-report with no verification.** Habitica acknowledges this openly ([FAQ](https://habitica.fandom.com/wiki/FAQ)).
- **Binary checkboxes with "close enough" completion**, plus streak-loss pressure. **[inference]** Combining the Duolingo loss-aversion finding with the anecdotal report above suggests the pressure is what pushes people to fudge.
- **Mitigations seen in the wild:** automatic data sources (Dijo scripts, Beeminder device integrations), quantified amounts rather than yes/no (Harsh `@ amount`, Beeminder graphable data), and an explicit skip button so honesty isn't punished (Harsh) ([Beeminder](https://www.beeminder.com/about), [Harsh](https://github.com/wakatara/harsh), [Dijo](https://github.com/NerdyPepper/dijo)).

## Transferable to Habitos
All items below are **[inference]** applied to the Habitos brief, drawing on the cited evidence.

1. **Keep a minimum-effort "did it" path.** Separate "the habit is done" from "how well or how much". Reward the minimum with the streak and the extras with game currency. This follows the Duolingo decoupling result.
2. **Use a decaying strength score rather than a pure streak for the core meter.** Loop-style, with an explicit skip/rest token (Harsh skip, Duolingo freeze) that costs a game resource, not shame. Let the pet and the roguelike run read this score, so a single miss dents progress without wiping it.
3. **Soft-loss pet.** Finch-style positive growth, with neglect causing a slow, recoverable decline. Death is optional and easy to leave out. For a personal game, recoverability is more important than stakes. If you want stakes, make them a voluntary "hardcore run" mode, as Beeminder's pledge is opt-in.
4. **Gameplay depth fed by habit output, not gated behind streak length.** Habit completions mint resources (energy/gold-like) that feed incremental or roguelike decisions. Adding mid-game systems (builds, unlocks, events) addresses novelty decay. Avoid making the roguelike's run-ending condition equal to "broke a streak".
5. **Reduce cheating incentive by design, not enforcement.** Offer quantity logging (`@ amount`), honest skip, and optional auto-habits via scripts or git hooks (Dijo, Beeminder). Don't escalate rewards for self-reported days beyond a cap. This is a single-user game with no leaderboard, so cheating costs only the player. Consider a cap on daily game income so fudging gains little.
6. **Terminal ergonomics.** Logging should take seconds, offer a urgency/todo view (Harsh), a modal view (Dijo), keep data in plain files, and keep the habit list small (Streaks' 24-task cap).
7. **Session mechanic (Forest-style).** An optional timed "focus run" whose pet or tree reward is forfeited if abandoned. It is a roguelike-run analogue and uses immediate, visible loss.
8. **Milestone feedback.** Short animated or ASCII celebration on milestone days, which Duolingo showed to lift early retention.

## Contradictions / disputed
- Loss-aversion streaks are credited with long-term engagement (Duolingo) while the same source says broken streaks demotivate and Loop calls streak-wipe a flaw. These are not contradictory so much as a tradeoff. The Duolingo post resolves it with freezes ([Duolingo](https://blog.duolingo.com/how-duolingo-streak-builds-habit/), [Loop](https://github.com/iSoron/uhabits)).
- Habitica community claims that cheaters lose interest versus claims that people game every tracker. Both are anecdotal and unresolved ([r/habitica](https://www.reddit.com/r/habitica/comments/h8vvh9/do_people_take_the_game_seriously/), [r/getdisciplined](https://www.reddit.com/r/getdisciplined/comments/1rfbuvr/been_through_like_15_habit_tracking_apps_whats/)).

## Missing evidence
- Official Habitica documentation of the death, party-damage and boss-quest rules, plus any retention data. Not fetched (403/DNS failures).
- Finch's and Forest's own retention or design rationale. Beeminder's own policy on cheating and data fudging (404).
- Loop's exact score formula (the README says "advanced formula"). Check the source repo before copying the algorithm.
- Peer-reviewed gamification-effectiveness literature (for example Hamari et al.). The search was rate-limited, so this is not covered.
- Lally's exact quantitative results (verify against the paper).
- Duolingo's results are vendor-reported A/B tests only.

## Sources
- Kept: Duolingo, [Improving the streak](https://blog.duolingo.com/improving-the-streak/) and [Streak uses habit research](https://blog.duolingo.com/how-duolingo-streak-builds-habit/). Primary, with experiment numbers.
- Kept: [Beeminder about](https://www.beeminder.com/about). Primary.
- Kept: [Loop Habit Tracker repo](https://github.com/iSoron/uhabits). Primary README.
- Kept: [Harsh repo](https://github.com/wakatara/harsh) and [launch post](http://daryl.wakatara.com/harsh-a-minimalist-cli-habit-tracker/). Primary.
- Kept: [Dijo repo](https://github.com/NerdyPepper/dijo). Primary.
- Kept: [Habitica press kit](https://habitica.com/static/press-kit), [FAQ](https://habitica.fandom.com/wiki/FAQ) and [Obstacles](https://habitica.fandom.com/wiki/Obstacles). Maker or wiki, honor-system statement. Seen via search snippets only.
- Kept: [Finch about](https://finchcare.com/about-finch), [help center](https://help.finchcare.com/hc/en-us/articles/37780134479757-Energy-vs-Rainbow-Stones), fan wiki [Energy](https://finch.fandom.com/wiki/Energy) and [Rainbow Stones](https://finch.fandom.com/wiki/Rainbow_Stones). Snippets only.
- Kept: [Forest Play Store](https://play.google.com/store/apps/details?hl=en&id=cc.forestapp) and [Trees for the Future](https://trees.org/support/corporate-sponsors/forest-app). Snippets only.
- Kept: [Streaks press](https://streaks.app/press/), [Streaks 7](https://crunchybagel.com/now-available-streaks-7/) and [privacy](https://streaks.app/privacy/). Primary, via snippets.
- Kept: [Lally 2010 abstract](https://onlinelibrary.wiley.com/doi/10.1002/ejsp.674?538fcd4b_page=), [BPS digest](https://www.bps.org.uk/research-digest/how-form-habit) and [Gardner et al.](https://pmc.ncbi.nlm.nih.gov/articles/PMC3505409/). Research.
- Anecdotal only: [r/getdisciplined thread](https://www.reddit.com/r/getdisciplined/comments/1rfbuvr/been_through_like_15_habit_tracking_apps_whats/) and [r/habitica thread](https://www.reddit.com/r/habitica/comments/h8vvh9/do_people_take_the_game_seriously/).
- Deprioritized: [gamificationexplained.com Habitica case study](https://gamificationexplained.com/product-review/case-study-habitica-turning-your-life-into-a-role-playing-game/) (secondary, used only for the HP/death description and flagged). [sorushi.com Lally summary](https://sorushi.com/learn/research/lally-2010) (secondary). Canvas8 and a WordPress interview blog on Habitica (secondary, not used).
