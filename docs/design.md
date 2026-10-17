# Tact — Game Design Document

> **Status:** early draft. Most of this is a starting point for discussion,
> not settled decisions. Items marked **(open)** need a call.

## 1. Vision

Tact is an isometric, pixel-art, turn-based tactics game. Its main
inspiration is *Final Fantasy Tactics Advance* (FFTA): small squads on
compact, hand-built maps with lots of height variation, a deep job system
that rewards experimenting with builds, and a light, adventurous tone.

The goal is not to clone FFTA but to keep what makes it great:

- **Readable battles.** You can see the whole fight at a glance. Maps are
  small, units are distinct, and the rules are legible from the UI.
- **Build-crafting as the long game.** Most of the depth lives in choosing
  jobs and abilities for your units between battles.
- **Every map is a puzzle box.** Height, chokepoints, and terrain matter as
  much as stats.
- **Rules that change the game.** FFTA's *laws* made each battle feel
  different. We want an equivalent, but one that's more fun than punishing
  (see §4.6).

### Design pillars

1. **Clarity over complexity.** If a mechanic can't be shown clearly in the
   UI, simplify it.
2. **Meaningful positioning.** Facing, height, and terrain should affect
   most decisions.
3. **Rewarding experimentation.** Learning new abilities should feel like
   unlocking new toys, and mixing jobs should produce surprising combos.
4. **Short sessions.** A battle should take roughly 10–20 minutes.

## 2. Presentation

### Camera and projection

- Fixed 2:1 dimetric ("isometric") projection, like FFTA.
- Tile top faces are **32×16 px**; each height level is **8 px** tall.
- The map can be rotated in 90° steps **(open — FFTA did not allow this;
  it helps readability on tall maps but quadruples some art)**.
- The camera follows the cursor and active unit, with smooth panning.

### Resolution and pixel art

- Target a low internal resolution and scale up by an integer factor, so
  pixels stay square and crisp. Candidates: **320×180** (16:9, scales
  cleanly to 720p/1080p/1440p/4K) or FFTA-style 240×160 **(open)**.
- Restricted palette. The placeholder art uses colors from DawnBringer's
  DB32 palette; the final palette is **(open)**.
- Unit sprites around **24×32 px**, with idle, walk, attack, cast, hurt, and
  KO animations, in two facings (front and back), mirrored for the others.

### UI

- Grid cursor driven by keyboard/gamepad first; mouse support second.
- Unit panel showing HP/MP, job, and status effects.
- Turn order timeline always visible.
- Move/attack ranges drawn as tinted tile overlays.
- Damage and hit chance previewed before confirming an action.

## 3. Core loop

```
 World map ──► Choose mission ──► Prepare party ──► Battle
     ▲                                                │
     └──── Rewards (gil, items, AP, story) ◄──────────┘
```

- **World map:** pick missions, visit shops, and trigger story events.
- **Party prep:** pick up to 6 units, set jobs, equipment, and abilities.
- **Battle:** turn-based tactical fight on an isometric map.
- **Rewards:** money, equipment, and ability points (AP) used to learn
  abilities.

Whether we include FFTA-style "dispatch" missions (sending a unit away to
complete a mission off-screen) is **(open)**.

## 4. Battle systems

### 4.1 Maps

- Grids of roughly 8×8 to 16×16 tiles.
- Each tile has a **terrain type** and an integer **height**.
- Terrain affects movement cost and sometimes abilities (e.g. water halves
  movement and blocks some actions; lava deals damage).
- Maps are authored by hand. The early prototype uses a text format; a
  proper format (likely RON, or Tiled/LDtk import) is **(open)**.

### 4.2 Turn order

Use a **charge time (CT)** system, like FFTA:

- Every tick, each unit gains CT equal to its **Speed**.
- When a unit reaches 100 CT it takes a turn.
- What a unit does on its turn sets how much CT it keeps: moving *and*
  acting costs the most; waiting without doing either costs the least. This
  rewards patience and makes Speed a strong stat.
- The upcoming turn order is shown on screen and updates when you preview
  an action.

### 4.3 A turn

On its turn a unit may **Move** once and **Act** once, in either order, then
chooses a **facing** to end on.

- **Move:** up to *Move* tiles. A unit can step up or down at most *Jump*
  height levels per tile.
- **Act:** attack, use an ability, or use an item.
- **Facing:** attacks from the side or back are more likely to hit and
  harder to counter, so where you end your turn matters.

### 4.4 Combat

- Stats: HP, MP, Attack, Defense, Magic, Resistance, Speed, Move, Jump,
  Evasion.
- **Hit chance** depends on the attacker's accuracy, the target's evasion,
  and the attack's direction (front < side < back).
- **Height:** ranged attacks reach farther from high ground; melee attacks
  can only reach targets within a height difference limit.
- Damage formula **(open)**. Prefer formulas a player can roughly predict
  over ones with lots of hidden modifiers.
- Status effects: poison, sleep, slow/haste, silence, and so on. Keep the
  list short and make each one visibly distinct.

### 4.5 Victory and defeat

Most battles are "defeat all enemies", with variations: defeat a leader,
survive N turns, reach a tile, protect a unit. Losing all your units ends
the battle; whether KOed units can be permanently lost is **(open)**.

### 4.6 Battle rules (our take on *laws*)

FFTA's judges enforced laws ("no fire magic", "no healing") that penalized
breaking them. They added variety but often felt like arbitrary
punishments.

Our version: each battle has one or two visible **rules** that change how
the fight plays, and they apply to *both* sides. Some ideas:

- *Highground:* units on higher tiles deal extra damage.
- *Quickened:* everyone gains CT faster, but MP costs double.
- *Fragile:* all damage doubled.
- *Pact of iron:* no magic; physical damage reduced.

Optional **bonus objectives** (e.g. "don't use items") could reward extra
loot without punishing the player. Whether rules ever penalize the player
directly, FFTA-style, is **(open)**.

## 5. Units and progression

### 5.1 Races and jobs

Like FFTA, each unit belongs to a **race**, and each race can take a
specific set of **jobs**. Races give flavor and constrain builds, so that
two units with the same job still play differently.

The races and their jobs are **(open)**. For the first prototype we'll use
a few generic jobs:

| Job     | Role                          | Example abilities         |
| ------- | ----------------------------- | ------------------------- |
| Soldier | Front-line melee              | Power Strike, Provoke     |
| Archer  | Ranged physical, uses height  | Aim, Pin Down             |
| Mage    | Area damage magic             | Fire, Frost, Bolt         |
| Healer  | Support and recovery          | Cure, Protect, Revive     |
| Thief   | Mobility, steals and debuffs  | Steal, Backstab           |

### 5.2 Learning abilities

FFTA tied abilities to equipment: a weapon teaches a skill while equipped,
and you master it by earning AP. This made loot exciting. Options:

- **Equipment teaches abilities** (FFTA style).
- **Jobs teach abilities** directly by spending AP earned in that job.

**(open)** — the FFTA approach ties together loot and progression, which
is a big part of its appeal.

### 5.3 Ability slots

A unit has:

- A **primary** ability set from its current job.
- A **secondary** ability set from any other job it has learned.
- A **reaction** (triggers when hit, e.g. counterattack).
- A **support** (passive, e.g. +1 Move).

Mixing these is the core of build-crafting.

## 6. Story and setting

**(open)** FFTA framed its story around a child pulled into a storybook
world. Tone should stay light and adventurous with some heart. Setting,
characters, and premise to be decided.

## 7. Technical design

- **Engine:** Bevy (Rust), using its ECS throughout.
- **Rendering:** 2D sprites placed in isometric screen space. Draw order is
  computed from grid position and height (see `src/iso.rs`). Nearest
  filtering for pixel art.
- **Game logic separated from presentation.** Battle rules (turn order,
  movement, damage) should be plain Rust that doesn't depend on rendering,
  so they can be unit-tested and eventually used by AI and replays.
- **Data-driven content.** Jobs, abilities, items, and maps defined in data
  files, not code.
- **AI:** score-based utility AI to start (evaluate each move/act/facing
  option and pick the best), with per-unit personalities.

### Current code layout

| Module          | Responsibility                                  |
| --------------- | ----------------------------------------------- |
| `src/iso.rs`    | Grid ↔ screen projection and draw-order depth   |
| `src/tiles.rs`  | Generated placeholder tile art                  |
| `src/map.rs`    | Map data, parsing, and spawning terrain sprites |
| `src/cursor.rs` | Tile cursor and its keyboard controls           |
| `src/camera.rs` | Pixel-scaled camera that follows the cursor     |

## 8. Roadmap

1. **Scaffold** ✅ — Bevy app, isometric test map with heights, cursor,
   camera.
2. **Units on the map** — spawn units, show them on tiles, select one.
3. **Movement** — pathfinding with Move/Jump limits, range overlay,
   walking animation.
4. **Basic combat** — attack, HP, KO, facing, hit chance.
5. **Turn order** — CT system and timeline UI.
6. **Enemy AI** — simple utility AI.
7. **Jobs and abilities** — data-driven ability definitions, a few jobs.
8. **Battle rules** — the system from §4.6.
9. **Meta loop** — party prep, rewards, world map.

## 9. Open questions

- Internal resolution: 320×180 or 240×160?
- Map rotation?
- Permanent unit loss?
- Equipment-taught or job-taught abilities?
- Races, setting, and story.
- How punishing should battle rules be?
- Mouse and touch support priority?
