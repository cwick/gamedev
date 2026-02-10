# Engine-Scheduled Systems Plan: Integrate Velocity First

## Summary
Move `integrate_velocity` out of game-specific code and into engine-owned scheduling so games no longer call it manually.

This change establishes the reusable scheduling pattern for future engine-owned systems while preserving deterministic simulation and the current WASM boundary.

Chosen phase model:
1. `Control`
2. `Physics`
3. `Resolve`

Execution order per tick:
1. Clamp `dt` and write input bits.
2. Run `Control` systems.
3. Run `Physics` systems (engine-owned integration).
4. Run `Resolve` systems.

## Architectural Goals
1. Keep simulation platform-agnostic and deterministic.
2. Keep engine-owned behavior in the engine, not in game modules.
3. Keep the model simple and composable for future systems (input, collision helpers, etc.).
4. Avoid per-frame allocations and avoid WASM API churn.

## Public Interface and Type Changes
1. Update scheduler API in `src/engine/ecs/schedule.rs` to support named phases.
2. Add a `SystemPhase` enum with:
   - `Control`
   - `Physics`
   - `Resolve`
3. Add phase-based registration:
   - `with_system_in_phase(phase: SystemPhase, system: SystemFn) -> Self`
4. Keep deterministic ordering:
   - fixed phase order: `Control -> Physics -> Resolve`
   - insertion order preserved within each phase
5. Keep `SystemFn = fn(&mut World, f32)` unchanged.
6. Keep WASM-facing APIs unchanged (`engine_init`, `engine_step`, snapshot ptr/len).

## Ownership Model
1. `Physics` phase is engine-owned for shared integration systems.
2. Game modules register their game logic in `Control` and `Resolve`.
3. Game modules must not call engine-owned systems (like `integrate_velocity`) directly.

## Implementation Plan

### 1) Scheduler Refactor
1. Modify `Schedule` internals from a single list to phase buckets.
2. Keep or adapt builder ergonomics so call sites remain straightforward.
3. Keep `run(world, dt)` as the only execution entrypoint and make phase ordering explicit in code.
4. Add scheduler unit tests for:
   - phase ordering
   - insertion ordering within phase
   - `dt` passthrough

### 2) Engine Composition
1. Register `integrate_velocity` in the `Physics` phase from engine-level construction code.
2. Ensure it runs exactly once per tick between `Control` and `Resolve`.
3. Keep `Engine::step` responsibilities unchanged except for schedule execution semantics.

### 3) Pong Migration
1. Remove manual `integrate_velocity` call from `src/games/pong/mod.rs`.
2. Split system registration by phase:
   - `Control`: `handle_restart`, `apply_input`
   - `Resolve`: collision/scoring/timers and related post-integration logic
3. Preserve existing gameplay behavior:
   - serve delay behavior
   - score accumulation
   - game-over no-op behavior
   - paddle clamping and collision outcomes

### 4) Arkanoid Migration
1. Remove manual `integrate_velocity` call from `src/games/arkanoid/mod.rs`.
2. Split system registration by phase:
   - `Control`: `apply_input`
   - `Resolve`: clamp and collision response
3. Preserve existing behavior:
   - paddle motion from input
   - ball wall bounce response
   - world initialization expectations

### 5) Follow-up Guardrails
1. Add a short comment or doc note in scheduler/engine code:
   - `Control` = intent and velocity setup
   - `Physics` = engine integration systems
   - `Resolve` = post-move consequences
2. Use this same structure when later introducing additional engine-owned systems.

## Test Plan
1. Scheduler tests (`src/engine/ecs/schedule.rs`):
   - runs phases in `Control -> Physics -> Resolve` order
   - respects insertion order within each phase
   - passes `dt` through unchanged

2. Engine/system tests:
   - verify `integrate_velocity` still updates transforms through schedule execution
   - verify it is not called manually from game physics paths

3. Pong regression coverage (`src/games/pong/tests.rs`):
   - top/bottom wall bounce
   - paddle bounce direction and angle behavior
   - scoring and serve delay behavior
   - game-over/restart behavior

4. Arkanoid regression coverage (`src/games/arkanoid/mod.rs` tests and any added tests):
   - paddle input movement still functions
   - paddle clamp bounds
   - ball wall collision response

5. Validation commands:
   - `cargo test`
   - `cargo clippy`
   - `cargo fmt -- --check`

## Acceptance Criteria
1. No game code manually calls `integrate_velocity`.
2. `integrate_velocity` runs via scheduler in `Physics` phase.
3. Ordering is explicit and deterministic: `Control -> Physics -> Resolve`.
4. Existing gameplay tests pass or are updated only for phase registration mechanics, not behavior.
5. No changes to WASM/public runtime API contract.

## Assumptions and Defaults
1. Phase names are fixed as `Control`, `Physics`, `Resolve`.
2. This change migrates only `integrate_velocity` as the first engine-owned scheduled system.
3. Future engine-owned systems will be added incrementally using the same phase model.
