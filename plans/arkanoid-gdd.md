# Arkanoid (Breakout-style) — Game Design Doc (v0.1)

## One-liner
Classic paddle + ball brick-breaker with tight controls, light progression, and short sessions.

## Goals (tight scope)
- **Shippable**: one polished mode with a small set of levels.
- **Feel-first**: responsive paddle/ball physics that are easy to learn and satisfying to master.
- **Readable**: clean visuals, clear feedback, minimal UI.

## Non-goals (for v1)
- No story/cutscenes, online features, or complex meta-progression.
- No large power-up catalog (start with a tiny set or none).
- No procedural generation (hand-authored levels only).

## Target platform + input
- Web build (same stack as Pong project).
- Input: mouse, keyboard, gamepad.

## Player fantasy
“I can control the rebound.” The player’s skill is positioning the paddle and choosing angles to clear patterns efficiently.

---

## Core gameplay

### Core loop
1. Serve the ball.
2. Keep the ball in play by bouncing it off the paddle.
3. Break all bricks (or meet a clear condition) to advance.
4. Lose a life if the ball falls below the paddle.

### Win / lose
- **Win a level**: all breakable bricks destroyed (or a level-defined target count).
- **Lose a run**: lives reach 0.

### Session length (initial target)
- 5–10 levels in the first release.
- 1–3 minutes per level for a new player.

---

## Controls (translating the Arkanoid dial)
The original “dial” is effectively **high-resolution horizontal control**. We mimic that with *smooth, high-precision paddle positioning* and optional sensitivity settings.

### Mouse (recommended default)
- **Relative mouse (delta) mode**: paddle moves by mouse movement each frame (feels like a dial).
  - Pros: high precision, no screen-edge issues, matches “spin the dial” muscle memory.
  - Cons: requires pointer lock.
- Optional toggle: **Absolute mouse mode**: paddle x maps to cursor x.
  - Pros: simple to understand.
  - Cons: cursor can hit screen edges; less “dial-like”.

### Keyboard
- Left/Right (or A/D) moves paddle.
- Add **acceleration + friction** so taps allow micro-adjustments but holding reaches max speed quickly.
- Optional modifier: **Shift = slow/precision**.

### Gamepad
- Left stick: analog paddle speed (apply deadzone + curve for fine control).
- D-pad: digital movement (same feel as keyboard).
- Optional bumper: precision modifier (like Shift).

### Feel settings (simple menu)
- Paddle max speed
- Paddle acceleration
- Mouse sensitivity
- “Dial mode” toggle (mouse relative vs absolute)

---

## Mechanics and rules

### Paddle
- Fixed vertical position; horizontal movement only.
- Paddle has a width (can be modified by power-ups in later versions).
- Paddle collision determines rebound angle.

### Ball
- **Constant speed**: Fixed velocity magnitude (~4 pixels/frame, adjustable per level). No acceleration from paddle velocity.
- **Rebound angle (paddle collision)**:
  - Divide paddle width into 8 zones (left to right).
  - Far left zones: ball rebounds at ~75° left.
  - Center zones: ball rebounds more vertically (~60° from horizontal).
  - Far right zones: ball rebounds at ~75° right.
  - This creates the "skill" in paddle positioning—hit edges for steep angles, center for safer vertical shots.
- **Minimum horizontal component**: Clamp to prevent near-vertical loops (e.g., ball always has ≥20% of speed moving horizontally).
- **Wall bounces**: Perfect reflection off top and side walls (angle in = angle out).
- **Serve state**:
  - Ball sits on paddle at rest.
  - Player presses action button (spacebar/gamepad) to launch; ball inherits paddle's center x position.
  - Launch angle: 60° upward (slight left/right bias if paddle was moving, optional).
  - After a life loss: 1-second serve delay (visual countdown), then Ready for next serve.

### Brick types (v1 set)
Keep this small so levels are interesting but implementation stays manageable:
- **Normal** (blue/cyan): 1 hit to break. Worth 10 points.
- **Tough** (yellow/orange): 2 hits to break. Shows visible crack after first hit. Worth 20 points (per brick, not per hit).
- **Unbreakable** (silver/metallic): Never breaks; acts as a wall. Used to shape ball routes and create puzzle patterns. Worth 0 points. Does reflect the ball like a wall.

### Level rules
- Each level defined by:
  - **Grid**: 6 rows × 13 columns (78 possible brick slots; original Arkanoid used 88).
  - **Ball speed**: Base pixels/frame (e.g., level 1 = 3 px/frame, level 8 = 5 px/frame).
  - **Brick layout**: Mix of Normal, Tough, and Unbreakable types.
  - **Win condition**: All destructible bricks cleared (no lives limit per level; only global lives matter).
- **Difficulty ramp**:
  - Levels 1–2: Mostly Normal bricks, simple patterns. Teaches controls.
  - Levels 3–4: Introduce Tough bricks and some Unbreakables. More complex routes.
  - Levels 5–6: Dense Unbreakable layouts that force precise angles.
  - Levels 7–8: High ball speed + mixed patterns. High-risk/high-reward positioning.

---

## Scoring and progression

### Scoring (simple)
- Normal brick: 10 points.
- Tough brick: 20 points (awarded when fully destroyed, not per hit).
- Unbreakable: 0 points.
- Optional bonus (v1.5): "Streak" multiplier—if player breaks 5+ bricks without the ball touching the paddle, next brick is worth 1.5× points. Reset on paddle contact. *Only if it doesn't complicate HUD.*

### Lives
- Start with 3 lives.
- **Extra life**: Award 1UP at 5,000 points, then again at 15,000, etc. (classic Arkanoid pattern). Shows "1UP" briefly on screen when awarded.

### Difficulty ramp
- Early levels teach angle control and safe clears.
- Mid levels add tight gaps via unbreakables and mixed toughness.
- Later levels increase ball speed slightly and use patterns that demand controlled rebounds.

---

## UI / UX

### In-game HUD
- Lives, score, level number.
- Small “Paused” overlay.

### Menus (minimal)
- Title: Play, Controls, Credits.
- Controls: input mode toggles + sensitivity.

### Feedback
- **Brick hit**: Bright beep (e.g., 800 Hz, 50ms) + small particle burst (1–3 sparks from brick center).
- **Tough brick first hit**: Slightly lower beep (700 Hz) + visible crack sprite overlayed on brick. Remains cracked until second hit.
- **Tough brick destroyed**: Higher beep (900 Hz) + burst.
- **Paddle hit**: Soft "pong" sound (400 Hz, 30ms). Pitch slightly varies if hit near edges (friendly audio feedback for angle).
- **Wall bounce**: Very quiet beep (600 Hz, 20ms) or silent (original was silent in some versions).
- **Life loss**: Descending tone (500 → 200 Hz over 200ms) + tiny screen shake (2–3 pixels, 1 frame).
- **1UP awarded**: Musical flourish (ascending tones, ~500ms).
- **Level clear**: Brief fanfare (3–4 note pattern, celebratory) + "COMPLETE" text + pause for 2 seconds before auto-advance or "Press Action to Continue".

---

## Audio / Visual direction

### Visual style (classic Arkanoid + modern clean)
- **Bricks**: Flat, high-contrast colors. Grid layout with minimal spacing.
  - Normal: Bright cyan/blue.
  - Tough: Warm yellow/orange (easier to distinguish from Normal).
  - Unbreakable: Silver/metallic with a grid pattern (not flat, to show "hard").
- **Ball**: Bright white/yellow circle with a 2–3 frame motion trail (direction indicator).
- **Paddle**: Flat white or cyan bar; highlights slightly on input for feedback.
- **Background**: Dark (navy or black) for contrast. Optional subtle grid.
- **Particles**: Simple 1–2 pixel sparks on brick destroy. Quick fade (~200ms).

### Audio style (arcade-inspired, not 8-bit bloopy)
- Use simple sine-wave tones (80s arcade feel) or clean chiptune samples.
- All SFX under 100ms (snappy, not muddy).
- No looping music in v1; optional peaceful loop in later versions.

---

## Technical notes (implementation-friendly)

### Physics & collision
- **Fixed timestep**: 60 FPS (16.67ms per frame) for deterministic ball movement.
- **Collision resolution order** (per frame):
  1. Paddle (highest priority; prevent ball from sinking through).
  2. Bricks (resolve nearest collision first to avoid multi-hit per frame).
  3. Walls (lowest priority).
- **Continuous collision detection**: If ball speed is high, use swept AABB or raycast to prevent passing through thin objects.

### Game state machine
```
Ready → Playing ↔ Paused
Playing → (ball lost) → Serve
Playing → (all bricks cleared) → LevelClear
Playing → (lives = 0) → GameOver
```

### Data structures
- **Level data**: Array of 6×13 grids. Each cell: brick type enum (None, Normal, Tough, Unbreakable) or empty.
- **Metadata per level**: ball speed, level number.
- **Game state**: lives, score, current level, brick grid (with Tough brick HP), ball pos/vel, paddle pos, 1UP threshold, served flag.
- **Input abstraction**: Map all input methods (mouse delta, keyboard keys, gamepad stick) to a single `paddle_velocity` value (−1 to +1).

### Serve flow
- Ball rests on paddle at center of paddle's x position.
- On action input: ball gains upward velocity + slight horizontal bias if paddle is moving.
- Ball is "in play" only after serve.

---

## Content plan (v1)
- **Level count**: 8 levels (ship), with room to add more.
- **Brick set**: Normal, Tough, Unbreakable.
- **No power-ups** for the first playable; add only if the base feel is already strong.

---

## Follow-up features (pick 2–3)

### Power-ups (Classic Arkanoid set, add to v1.1+)
If base feel is strong, introduce power-ups as rare drops (1–2 per level):
- **Expand Paddle** (⬜): Widen paddle by 50% for next 3 hits or 15 seconds.
- **Slow Ball** (🔵): Reduce ball speed by 25% for 20 seconds.
- **Catch** (✋): Ball sticks to paddle after next hit; press action to re-launch. (High skill ceiling.)
- **Multi-ball** (⚪⚪): Spawn 1–2 extra balls (original Arankoid spawned up to 4). Risky—easier to lose lives, but more fun.
- **Laser** (⚡): Paddle shoots 1–2 projectiles per second upward. Lasts 10 seconds.
- **Break** (💥): Ball passes through bricks in a straight line for 3 seconds.

### Other follow-ups
1. **Level themes + hazards**: Moving bricks or "laser gate" rows that open/close on a timer.
2. **Challenge modes**: Time Attack (clear as fast as possible) or 1-life "Arcade" run with leaderboards (local only).
3. **Difficulty selector**: Easy (fewer tough bricks, slower ball), Normal, Hard.

---

## Milestones (suggested)
1. **Feel prototype**: paddle + ball + walls + one brick type.
2. **Levels**: load level layouts + win/lose flow.
3. **Polish**: UI, audio, particles, settings.
4. **Content pass**: 8 curated levels with difficulty ramp.

