# Generalize the Engine to Support Pong + Arkanoid (both kept working)

## Summary
Refactor the current single-game (Pong) simulation into an **ECS-lite** engine that runs a configurable `World` (Entities + Components + Resources) through a configured list of `Systems`. The only difference between “Pong” and “Arkanoid” becomes the **world blueprint** (what entities/components exist) and the **system schedule** (which systems run, and with what resource/config values).

Decisions locked:
- Game switching: **in-app menu**
- Snapshot: **per-game snapshots**
- Input: **bits-only** `u32`

---

## Architecture

### Rust module layout
Reshape `src/engine/` into:
- `src/engine/mod.rs`
  - `pub use Engine` (used by WASM) + input bit constants.
  - Contains no “Pong vs Arkanoid” branching beyond selecting a `GameDefinition` from a registry.
- `src/engine/core.rs` (shared “engine stuff”)
  - Input bit constants, dt clamp, shared math helpers.
  - `GameId` newtype + `GameDefinition` metadata (id/name).
- `src/engine/ecs/` (ECS-lite implementation)
  - `entity.rs`: `EntityId` allocator/reuse (simple `u32`).
  - `world.rs`: component storages + resources + spawn/despawn.
  - `query.rs`: tiny iteration helpers (no macros, keep it simple).
  - `schedule.rs`: ordered list of systems (`fn(&mut World, f32)`).
  - `components.rs`: shared component types (Transform/Velocity/Collider/Render/etc).
  - `resources.rs`: shared resources (Input, FieldBounds, Rules, RNG seed, etc).
- `src/engine/games/`
  - `mod.rs`: game registry: `fn definitions() -> &'static [GameDefinition]` and `fn build_world(game_id, width, height) -> (World, Schedule, SnapshotWriter)`
  - `pong.rs`: builds Pong world blueprint + schedule + snapshot writer (legacy snapshot layout preserved).
  - `arkanoid.rs`: builds Arkanoid world blueprint + schedule + snapshot writer.

### Engine host type (single active World)
In `src/engine/mod.rs` define:
- `pub struct Engine { width: f32, height: f32, game_id: GameId, world: World, schedule: Schedule, snapshot: SnapshotBuffer, snapshot_writer: SnapshotWriter }`

Rules:
- `Engine::init(width,height)` loads a default `GameId` via the registry and builds a fresh world.
- `Engine::set_game(game_id)` rebuilds `world/schedule/snapshot_writer` from the selected game definition.
- `Engine::step(dt, input_bits)`:
  - clamps dt (shared core util)
  - writes `InputBits { bits: u32 }` resource
  - runs the configured `schedule`
  - updates the snapshot via the configured `snapshot_writer`
- `Engine::snapshot_ptr_len()` returns the current snapshot buffer ptr/len (same WASM contract).

### Shared input bits (single API for both games)
In `src/engine/core.rs` define:
- `INPUT_UP`, `INPUT_DOWN` (used by Pong)
- `INPUT_LEFT`, `INPUT_RIGHT` (used by Arkanoid)
- `INPUT_ACTION` (used by both; Pong restart / Arkanoid serve)
- `INPUT_PAUSE` (reserved; optional later)

Each game interprets only what it needs.

### ECS-lite (what “configuration” means)
This is intentionally not a full ECS framework—just enough structure to avoid “game enum branching” and keep logic reusable.

- **Entities**: opaque `EntityId(u32)`.
- **Components**: per-type storages inside `World` (start with `Vec<Option<T>>` indexed by entity id; optimize later if needed).
- **Resources**: singletons in `World` for global state/config (`InputBits`, `FieldBounds`, `RulesPong`, `RulesArkanoid`, etc).
- **Systems**: plain functions operating on the world; shared systems are reused between games.
- **Schedules**: ordered Vec of systems; each game defines its schedule.

### Per-game snapshot layouts (separate and explicit)
Keep two independent Float32 layouts (JS chooses layout based on active game).

**Pong snapshot**: preserve current indices/meaning exactly to avoid regressions.

**Arkanoid snapshot (prototype v0)**:
- 0 `BALL_X`
- 1 `BALL_Y`
- 2 `BALL_VX`
- 3 `BALL_VY`
- 4 `PADDLE_X`
- 5 `PADDLE_Y`
- 6 `PADDLE_W`
- 7 `PADDLE_H`
- 8 `SCORE`
- 9 `LIVES`
- 10 `LEVEL`
- 11 `FIELD_W`
- 12 `FIELD_H`
- 13 `PHASE` (0=Serve/Ready, 1=Playing, 2=Paused, 3=LevelClear, 4=GameOver)
- 14 `BALL_IN_PLAY` (0/1)
- 15 `COUNT`

No bricks in snapshot yet; add later via compact encoding or a second buffer pointer.

---

## WASM API changes

Keep existing exports:
- `engine_init(width, height)`
- `engine_step(dt_seconds, input_bits)`
- `game_state_ptr() -> *const f32`
- `game_state_len() -> usize`

Add:
- `engine_set_game(game_id: u32)`
  - `game_id` is looked up in the `GameDefinition` registry; invalid values fall back to the default game
  - rebuilds the active `World` from that game’s blueprint using stored dimensions
- `engine_game_id() -> u32`
  - returns current active `GameId` (registry id)

Default if UI never selects: **Pong**.

---

## Web changes (in-app menu + two renderers)

### JS module split
Add:
- `web/games/pong.js`: `PONG_SNAP` + `renderPong(ctx, snap, canvas)`
- `web/games/arkanoid.js`: `ARK_SNAP` + `renderArkanoid(ctx, snap, canvas)`
- `web/game_select.js`: selection state + localStorage + event dispatch helper

### UI (in-app menu)
Update `web/app.js`:
- Add two buttons: “Pong” and “Arkanoid”
- Clicking updates UI state, writes localStorage, dispatches:
  - `window.dispatchEvent(new CustomEvent("game-select", { detail: { gameId } }))`

No WASM imports in Preact components.

### Main loop wiring (`web/index.html`)
- Import `engine_set_game`, `engine_game_id`
- On startup: `engine_init(800,600)` then read localStorage and call `engine_set_game(gameId)`
- Listen for `game-select` events and call `engine_set_game(gameId)`
- Each frame: build `input_bits`, `engine_step(dt,input_bits)`, read snapshot, route to renderer by active game
- Debug panel shows different fields depending on game

### Input mapping (bits-only)
- Pong: keep W/S/Up/Down + Space action
- Arkanoid: add A/D + Left/Right + Space action

---

## Incremental milestones (keeps both games working)

### Milestone 1 — Introduce ECS-lite core (no gameplay changes yet)
- Add `src/engine/ecs/*` with `World`, component storages, resources, schedule, and a minimal entity allocator.
- Add `src/engine/core.rs` (shared input bits + dt clamp).
- Keep existing Pong logic temporarily; ECS exists but isn’t wired in yet.

Acceptance:
- `cargo test` passes
- Web still runs Pong unchanged

### Milestone 2 — Convert Pong to ECS world (preserve snapshot layout)
- Implement `games::pong::build_world(width,height)`:
  - spawns entities (ball, paddles, boundaries, score “resource”)
  - sets resources (field bounds, serve delay, phase, etc)
  - returns a schedule composed from shared systems + Pong-specific systems
  - returns a Pong snapshot writer that fills the **existing** Pong snapshot indices
- Update WASM to own a single `Engine` (world + schedule + snapshot writer).

Acceptance:
- Web still runs Pong
- Pong snapshot layout unchanged

### Milestone 3 — Add Arkanoid blueprint (minimal playable; reuse systems)
- Implement `games::arkanoid::build_world(width,height)` using the same ECS core:
  - entities: paddle, ball, walls; (bricks optional for v0)
  - components/resources: transform/velocity/collider + arkanoid rules/phase
  - schedule: shared movement/collision + Arkanoid serve/rules + Arkanoid snapshot writer

Tests (`src/engine/games/arkanoid/tests.rs` or `src/engine/arkanoid/tests.rs`):
- Serve launches on action
- Min horizontal clamp holds while in play
- Wall bounce inverts correct component

Acceptance:
- `cargo test` passes
- Arkanoid selectable and playable at prototype level

### Milestone 4 — Web selector + renderer split
- Update `web/app.js` selector buttons
- Add `web/games/*.js` renderers
- Update `web/index.html` to switch game and render accordingly

Acceptance:
- Switching games at runtime re-inits selected game
- Pong unchanged; Arkanoid prototype renders and plays

### Milestone 5 — Stabilize ECS layering (new games are “just config”)
- Shared logic lives in reusable systems/components/resources.
- Game-specific code is limited to:
  - the blueprint (which entities/components/resources exist)
  - the schedule (which systems run, in what order)
  - the snapshot writer (layout choice)
- Add brief module docs that describe the contracts: `World`, `Schedule`, and snapshot buffer rules.

Acceptance:
- Adding a third game follows the same pattern without touching Pong logic

---

## Test plan

Rust unit tests:
- Pong: existing suite moved/adapted to `engine::games::pong` (behavior unchanged)
- Arkanoid: minimal tests as above
- Engine switching:
  - switching re-inits active game
  - snapshot len changes appropriately on switch

Manual browser scenarios:
- Defaults to Pong on load
- Select Arkanoid → works; select back → Pong still works
- Refresh preserves selection (localStorage)
- Space restarts Pong (game over) and serves Arkanoid

---

## Assumptions / defaults
- Canvas stays `800×600`
- Switching games always **resets** selected game state
- No new crates/dependencies
- Arkanoid prototype milestone excludes bricks/levels initially
