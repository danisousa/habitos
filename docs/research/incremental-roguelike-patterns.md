# Research: Incremental and roguelike patterns for a habit-driven terminal game (Habitos)

Labels: **[Src]** = stated by the source. **[Interp]** = my interpretation of a source. **[Inf]** = my inference for Habitos, not stated by any source.
Evidence limits: the search tool was rate-limited partway through. I read Kongregate Part III and the Berlin Interpretation in full. Other claims rest on search snippets. Gaps are listed under Sources.

## Summary
- Prestige is a deliberate reset in exchange for a lasting multiplier. Its purposes are the "ladder climbing" feeling and keeping numbers manageable [Src: Kongregate III]. It needs a long grind to feel earned, and a habit game has few actions per day. Prestige therefore fits only if the unit of progress is *days or weeks*, not minutes. [Inf]
- Offline progress is the idle pattern that fits best. A habit game is "offline" almost all the time. Egg, Inc. caps offline play at 2 hours and compensates by making prestige currency independent of previous runs, which reduces dependence on idle time [Src: Kongregate III]. [Inf] Habitos wants the opposite: a pet that keeps living while you are away, with no cap that punishes absence.
- Permadeath is the clearest clash. The Berlin Interpretation says permadeath is enjoyable because "the random environment makes this enjoyable rather than punishing" [Src]. A habit game has no random environment to cushion the loss. Habitica's own issue tracker has users saying death "makes you want to quit all together" [Src: HabitRPG #223]. A peer-reviewed study of Habitica reports counterproductive gamification effects [Src: Diefenbach & Müssig]. I read only the abstract snippet for that study.
- Runs, procedural generation and meta-progression fit if a "run" is a calendar unit (week or month). Real-life habits are the input, and the roguelite shell (a run that ends, a persistent unlock layer) makes lapses cheap. A bad week then costs one run, not the whole save. [Inf]
- Daily seeds are low-friction. Slay the Spire's Daily Climb computes the day's run locally from one integer, so no server is needed [Src: Warden]. The same seed gives the same content to everyone. That suits a once-a-day check-in, but a single-player personal game gets no leaderboard value from it. [Inf]
- Terminal games sustain very deep systems, and visual appeal is a craft problem, not a ceiling. Dwarf Fortress sold over 1M copies on Steam in about 3 years [Src: Game Developer]. Pokete shows a full creature-collecting game in a terminal [Src: Pokete repo]. [Inf] The pet is a natural fit for ASCII or animated-glyph art.
- The strongest overall fit is a **slow-clock, never-lose-everything** design. Offline accrual is the baseline. Prestige and runs operate on weekly or monthly clocks. Failure is represented as lost *momentum*, not lost *assets*. The pet should be forgiving.

## Incremental patterns (fit / clash for a habit-driven loop)

1. **Generators, multipliers and exponential growth.**
   - **[Src]** Multipliers temporarily bring production closer to costs. Prestige resets generators and multipliers in exchange for persistent multipliers ([Kongregate I](https://www.kongregate.com/en/pages/the-math-of-idle-games-part-i)).
   - **Clash (partial):** these curves are tuned for a player who acts every few seconds and sees feedback within minutes. [Inf] With about 3–10 inputs per day, exponential curves would need re-tuning to days or weeks. Otherwise the numbers jump too fast, or flatline.
   - **Fit:** a habit completion could be the "click" that triggers a multiplier, and the engine produces passively between check-ins. [Inf]

2. **Offline progress.**
   - **Fit (strong).** The player is away from the terminal most of the time.
   - **[Src]** Egg, Inc. caps offline gains at 2 hours. Clicker Heroes has no cap, but a steep difficulty wall compensates ([Kongregate III](https://www.kongregate.com/en/pages/the-math-of-idle-games-part-iii)).
   - **[Inf]** Habitos should likely not cap offline gains. A cap is a hidden penalty on real life. A cap would also work against "must not punish real life".
   - **[Inf]** The risk is that passive accrual makes the habit input optional. Habits should therefore *multiply or unlock*, not just add.

3. **Prestige / ascension.**
   - **[Src]** Prestige currency can be computed from lifetime earnings (Cookie Clicker, AdVenture Capitalist), max earnings (Realm Grinder), earnings since reset (Egg, Inc.) or upgrades bought (Clicker Heroes). These fall into two families: lifetime stats and since-reset stats ([Kongregate III](https://www.kongregate.com/en/pages/the-math-of-idle-games-part-iii)).
   - **[Src]** Cookie Clicker uses a cube root of lifetime earnings, so doubling prestige currency needs about 8× the earnings. Realm Grinder's square-root formula needs about 4×. Egg, Inc. needs about 128×, which nudges players toward more active play ([Kongregate III](https://www.kongregate.com/en/pages/the-math-of-idle-games-part-iii)).
   - **Fit:** a "lifetime"-based prestige means a missed week never erases the accrued base. [Inf]
   - **Clash:** prestige explicitly resets progress. A habit user who resets and then has a bad week may feel punished twice. Prestige should be opt-in and tied to a milestone, such as a streak or a completed season. [Inf]
   - **[Src]** Kongregate's closing advice: decide where the "fun" is (unfolding features, achievements, or optimizing the prestige loop) and focus on that. [Interp] A habit game has less room for a "number optimization" fun than a pure idle game does.

4. **Big-number novelty.**
   - **[Src]** Pecorella says "the novelty of big numbers on their own is largely gone".
   - **Clash:** [Inf] don't make the number the reward. The reward should be pet state, unlocks and narrative.

## Roguelike patterns (fit / clash)

1. **Permadeath.**
   - **[Src]** The Berlin Interpretation lists permadeath as a high-value factor: "You are not expected to win the game with your first character. You start over from the first level when you die... The random environment makes this enjoyable rather than punishing" ([RogueBasin](https://www.roguebasin.com/index.php/Berlin_Interpretation)).
   - **Clash (strong):** habits are not randomized, so a missed day is the player's real-life failure, not a dice outcome. HabitRPG issue #223 includes a user saying death "makes you want to quit all together" and proposing a level loss instead ([GitHub](https://github.com/HabitRPG/habitica/issues/223)). A Habitica analysis from the Octalysis community says the health-loss mechanism is easy to avoid or cheat ([SlideShare](https://www.slideshare.net/slideshow/habitica-design-challenge-finalist-for-octalysis-jacob-bender/75311389)).
   - **Soft substitute:** the *pet*'s run could end (retire, or "evolve and leave a legacy") without any real loss of progress. Death is rendered as a story beat. [Inf]

2. **Runs.**
   - **Fit:** a run bounded by a week or a month gives a reset point and a clear "finale" that lapses can't permanently scar. [Inf]
   - **Clash:** the genre's runs are 30 minutes to 3 hours, with many decisions each. Habitos has few decisions per day. A run would have to be a *slow, low-APM* run with several meaningful choices per day at most. [Inf]

3. **Procedural generation.**
   - **[Src]** Berlin: "random environment generation" increases replayability. Appearance and placement of items is random, monster appearance is fixed, and fixed content (plots, puzzles, vaults) removes randomness ([RogueBasin](https://www.roguebasin.com/index.php/Berlin_Interpretation)).
   - **Fit:** procedural generation is cheap content for a solo developer. Pet traits, events and map nodes could be generated from a seed. [Inf]
   - **Clash:** the "exploration" it enables is a high-APM activity. In Habitos, exploration should be *revealed over time*, such as a daily event or a next-node reveal. [Inf]

4. **Meta-progression (roguelite).**
   - **Fit (strong).** Persistent unlocks mean a bad run is not a wasted run. Slay the Spire unlocks new cards when a run ends [Src: Game Design Roundtable snippet, [link](https://thegamedesignroundtable.com/211-slay-the-spire-a-designers-deep-dive-with-anthony-giovannetti/)]. I did not read the transcript, so I only know that the topic is discussed.
   - **[Src]** A Design Club talk says the "roguelike vs roguelite" distinction is not simply about meta-progression, and that the term needs narrowing ([Design Club](https://gameindustrylibrary.com/documents/roguelike-genre-exploration-design-club)). [Interp] Meta-progression covers both character power-ups and knowledge or unlocks. Unlocks and codex entries are safer for habit tracking than permanent power (which makes failure feel worse).

5. **Daily seeds and challenges.**
   - **[Src]** Slay the Spire's Daily Climb: "Once a day, a difficult challenge is created by combining several rule-bending modifiers. The same seed is utilized by everyone" ([Steam patch 14 notes](https://store.steampowered.com/news/posts/?appids=646570&enddate=1520089948)). It has one preselected character, a fixed seed and modifiers, with a scoreboard per day ([wiki](https://slaythespire-archive.fandom.com/wiki/Daily_Climb)). It is computed locally from a single integer ([Warden](https://mattwarden.com/projects/spire-daily/)).
   - **Fit:** a date-seeded daily gives a once-a-day reason to open the terminal with no server. [Inf]
   - **Clash:** "difficult challenge" framing is wrong for habits. A daily modifier should be a *bonus or flavor* (e.g. "today, exercise counts double"), not an obstacle. [Inf]

6. **Turn-based, non-modal, resource management.**
   - **[Src]** Berlin lists turn-based, non-modal, resource management and complexity as high-value factors ([RogueBasin](https://www.roguebasin.com/index.php/Berlin_Interpretation)).
   - **Fit:** turn-based is a perfect match for TUIs. [Inf] The game doesn't need real-time updates.

## Hybrids worth noting
- **Roguelite + idle:** Kongregate's idle-genre overview notes short prestige loops appearing in launch games, shooters and RPGs, and offline progress being used to entice players to return ([Kongregate](https://www.kongregate.com/en/pages/idle-games)). [Inf] "Prestige = run" is a natural mapping: a run ends, you keep meta-currency, and the next run starts stronger.
- **Habit tracker + RPG:** Habitica is the main example, and it is mixed. The HCI study "Counterproductive effects of gamification" analyzed it ([ACM](https://dl.acm.org/doi/10.1016/j.ijhcs.2018.09.004)). I saw only the abstract snippet, which says gamification's effectiveness is a critical question. The GitHub issue above and the Octalysis critique show that health-loss punishment is contested in user feedback.
- **Idle + pet / creature care:** [Inf] I found no sourced examples in this pass. The pet-growing loop here is closer to an idle "generator" with a strong emotional hook than a roguelike.
- **Prestige as ascension tree:** the Cookie Clicker "heavenly" tree is documented in the Game Design Lab analysis ([teemo.dev](https://teemo.dev/game-design/cookie-clicker/design/)). That site is a third-party fan analysis, so treat it as secondary.

## Terminal-game lessons
- **Dwarf Fortress:** [Src] ASCII by default, a 20-year development effort, procedurally generated ([Wikipedia](https://en.wikipedia.org/wiki/Dwarf_Fortress)). Steam launch lifted monthly revenue from $15,635 to $7,230,123 ([Ars Technica](https://arstechnica.com/gaming/2023/02/after-16-years-of-freeware-dwarf-fortress-creators-get-their-7m-payday/)). It topped 1M Steam sales in just over 3 years ([Game Developer](https://www.gamedeveloper.com/business/dwarf-fortress-has-topped-1-million-sales-on-steam)). [Interp] Depth sells, and the Steam version added graphics. The source does not isolate the effect of tiles vs ASCII on sales. Tarn Adams has been interviewed about why ASCII was chosen ([Stanford](https://dhs.stanford.edu/algorithmic-literacy/an-interview-with-tarn-adams)); I did not read it.
- **NetHack / Angband / Crawl / ADOM / Rogue:** [Src] these form the canon the Berlin Interpretation was built from ([RogueBasin](https://www.roguebasin.com/index.php/Berlin_Interpretation)). [Interp] Their depth comes from item and monster interaction ("complexity"), not graphics.
- **Cataclysm: DDA:** I could not verify specifics (my search was rate-limited). The only evidence is a Hacker News comment listing it with NetHack and DCSS as successful FOSS games ([HN](https://news.ycombinator.com/item?id=25506062)). Treat as weak.
- **Pokete:** [Src] "a small terminal based Pokemon like game", ~3.1k GitHub stars, GPL-3.0, built on the author's `scrap_engine`, with a Python/Go codebase ([GitHub](https://github.com/lxgr-linux/pokete/), [How to play](https://github.com/lxgr-linux/pokete/blob/master/docs/HowToPlay.md)). [Interp] This is the closest analogue to Habitos: collection, creatures and an overworld in a terminal. It shows a creature-game can fit in a terminal. I did not examine its art quality.
- **ttyper:** [Src] a terminal typing test in Rust with Ratatui, ~1.6k stars ([GitHub](https://github.com/max-niederman/ttyper)). [Interp] It shows a single-purpose TUI can be popular with a small, clean interface. It has no progression layer.
- **[Inf] Takeaways for Habitos:** (a) depth lives in systems and content, not pixels. (b) One-purpose TUIs with a clean interface are viable. (c) The pet's animation and color are the main visual levers. (d) Turn-based and non-modal designs work naturally in a TUI.
- **Not covered:** lazygit/k9s UX conventions (panels, keybindings), and which terminals support true color or Unicode. I did not research these.

## Candidate loop shapes for Habitos (sketches, no decision)

**A. "Slow idle pet" (offline-first, no resets).**
Habit check-ins feed a multiplier on the pet's passive growth. Offline accrual is uncapped. Progress is cumulative. Lapses only slow the multiplier, which decays softly toward a floor. No prestige. Risk: could feel flat after months. [Inf]

**B. "Season runs" (roguelite on a calendar clock).**
A run is a week or month. Each day's habits move the pet through procedurally generated nodes (events, encounters). At the end, the run closes (the pet "retires" or evolves) and grants meta-currency or codex unlocks that persist. A failed run is weaker, not fatal. Daily seed (from the date) picks the day's event and a flavor modifier. Risk: the decision count per day is low. [Inf]

**C. "Prestige by milestone" (idle with optional ascension).**
Lifetime-based currency is computed from total habit completions, using a root curve so effort has diminishing returns, like the Cookie Clicker or AdVenture Capitalist family ([Kongregate III](https://www.kongregate.com/en/pages/the-math-of-idle-games-part-iii)). The player may ascend at a milestone, trading the current pet for a permanent bonus and a new egg. Opt-in only. Risk: a reset can read as a loss. [Inf]

**D. "Daily puzzle-event" (daily seed, no long reset).**
Each day's seed generates one small event or decision (a mini roguelike node). Habit completion supplies resources used in that event, and the outcome feeds the pet. No run structure. Cheap to build and a clear reason to open the app daily. Risk: shallow without a long-term layer. [Inf]

## Sources
- Kept: The Math of Idle Games, Part III (https://www.kongregate.com/en/pages/the-math-of-idle-games-part-iii). Read in full. Primary source on prestige formulas and offline caps.
- Kept: The Math of Idle Games, Part I (https://www.kongregate.com/en/pages/the-math-of-idle-games-part-i). Snippet only. Definitions of multiplier and prestige.
- Kept: A Brief Look at the Idle Games Genre (https://www.kongregate.com/en/pages/idle-games). Snippet only. Hybrid examples.
- Kept: Berlin Interpretation, RogueBasin (https://www.roguebasin.com/index.php/Berlin_Interpretation). Read most of it. Primary text from IRDC 2008.
- Kept: Slay the Spire Daily Climb: Steam patch notes (https://store.steampowered.com/news/posts/?appids=646570&enddate=1520089948), wiki (https://slaythespire-archive.fandom.com/wiki/Daily_Climb), Matt Warden (https://mattwarden.com/projects/spire-daily/). Snippets only.
- Kept: HabitRPG issue #223 (https://github.com/HabitRPG/habitica/issues/223). User feedback on death. Snippet only.
- Kept: Diefenbach & Müssig, Habitica study (https://dl.acm.org/doi/10.1016/j.ijhcs.2018.09.004). Snippet only. I did not read the findings.
- Kept: Pokete repo (https://github.com/lxgr-linux/pokete/), ttyper repo (https://github.com/max-niederman/ttyper). Snippets only.
- Kept: Dwarf Fortress sales and revenue (https://www.gamedeveloper.com/business/dwarf-fortress-has-topped-1-million-sales-on-steam, https://arstechnica.com/gaming/2023/02/after-16-years-of-freeware-dwarf-fortress-creators-get-their-7m-payday/).
- Deprioritized: teemo.dev, frameadvance.net, incrementalgames.org. These are third-party analysis sites, not primary.
- Deprioritized: Hacker News comment on FOSS games. Weak evidence.
- Not found or not verified:
  - Primary designer talks for Hades, Balatro, Universal Paperclips and Kittens Game. The search was rate-limited. Only the GDC Vault listing for the Slay the Spire metrics talk (https://www.gdcvault.com/play/1025731/) surfaced, and I did not read it.
  - Cataclysm: DDA primary sources.
  - Any source for offline progress in habit or pet apps (Finch, Tamagotchi-style).

## Next steps
- Read the Tarn Adams / Stanford interview for the ASCII rationale. Fetch the Cataclysm: DDA docs and Pokete's art approach.
- Find primary designer sources on gentle-failure pet design (Tamagotchi, Finch) and on Hades or Balatro meta-progression.
- Read the Diefenbach & Müssig findings in full before relying on the Habitica critique.
