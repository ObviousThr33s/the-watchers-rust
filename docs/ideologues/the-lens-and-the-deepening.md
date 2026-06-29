# Ideologue — the lens and the deepening

*Ideas that have not yet earned a branch. Per METHOD.md an idea waits in an
ideologue until it can name what **done** means; this doc parks the next arc of
work and, for each thread, gestures at the finish line it will need before it is
cut. Design depth here is not branch depth — the design may run many levels deep
while the branches stay a flat run.*

The game now has a spine: you gaze, and what you face fills one Portal that feeds
the Stats area and the floating overlay — no menus, no second control scheme.
Three arcs deepen it. They are ordered by how close each is to naming its finish
line, not by importance.

---

## I. Sight — shading, colour, brightness (gated by the lens)

The first-person view is monochrome Braille relief: legible as shape, but "a bit
hard to see what is going on." Colour is the fix, and it is **earned, not
toggled** — the Lenskeeper's gift (see `src/game/npc.rs`) is what provides the
colour scheme. Carry the lens, and the grey world shows its colours.

**Mechanism.** The renderer's spine is already *scene → light-field (`f32`) →
lens → pixel* (`src/gfx/light.rs`, `src/gfx/voxel.rs`). Colour and brightness
belong in that light-field, before quantisation:

- **Brightness** restores the depth cue the Braille pixel stage dropped: distance
  and altitude become luminance, carried as a `ratatui` style on the cell, not a
  coarser glyph.
- **Colour** comes from material, which the heightmap already half-knows — the
  `Texture` bands (ground vs structure) become hue bands (water blue, stone grey,
  foliage green), read from height and the gazed entity.
- The lens is the **switch** in the optics chain: no lens, the pixel stage stays
  monochrome; lens carried, the light-field's colour survives to the screen.

**Finish line (gesture).** A red render test: *with the lens in the inventory the
frame emits styled (coloured) cells; without it, the cells are plain.* The lens
turns one boolean in the pixel stage; the test pins the observable, not the
palette. Closest to ready — the lens exists, the light-field exists.

---

## II. Generation — deeper, bounded, streamed

Today the world is one single-octave `NoiseGround`, a flat sow of flora, a strewn
hoard of items, all on one fixed seed. Enough to stand on; not yet a world.

- **Fractal relief.** Several octaves of noise summed, so the land has both broad
  forms and fine texture instead of one smooth wavelength.
- **The function generator.** Compose terrain from *density functions* (the
  Minecraft noise-router shape: continentalness, erosion, ridges feeding a final
  density), so terrain is authored as a graph of functions, not a hard-coded
  formula. (Deferred until after the weekly reset, by the Professor's call.)
- **Natural bounds.** Deep water and impossibly high plateaus, generated as
  impassable rims, fence "levels and areas" without a hard-coded world cap — the
  map design does the bounding, the engine streams.
- **Chunked streaming + the Roster free-list.** Partition storage by chunk,
  generate each deterministically from its chunk-coordinate seed, load and unload
  around the player. This is the moment the `Roster` finally grows the slot-reuse
  + generation tag its TODO names, turning its slab into a true streaming bound.

**Finish lines (gestures).** *Two octaves give relief at two scales.* *A region
ringed by deep water has an impassable rim.* *A chunk generates identically
however you arrive at it, with resident memory bounded.*

---

## III. The deepening — a comprehensive, diverse feature set

All of these surface through the **same spine** (gaze → Portal → Stats / overlay),
so none of them adds a control scheme — that constraint is what keeps the feature
set deep without the controls sprawling.

- **Beings & behaviour.** Composable, gaze-gated behaviours authored as files,
  each emitting `Event`s into the `Haps` bus; a separate faction/stance table; and
  **group readings** (cohesion, alignment, lead, fracture) carried on the
  *being↔group edge*, never averaged — averaging collapses everyone into anyone.
- **The flip (battle mode).** Time pauses into battle when a being is on screen;
  beings move when unwatched. The gaze and occlusion the spine already runs are
  the keystone; this is the founding "narrative via time + occlusion" vision made
  mechanical.
- **Dialogue.** The Lenskeeper's one line becomes a tree walked by the talk key —
  loaded from a file, never hard-coded (the story lives at the file boundary).
- **Memory & the fade.** `Recollection`'s clarity fades as *signal corruption*
  (the `Partial(f32)` seam): datamosh, Hamming noise, white noise. The liminal
  fade (`docs/ideologues/the-liminal-fade.md`) is the first finish line here.
- **Items deepened.** A `.item` file format with provenance — a place, a time, and
  the group or individual that made each — plus stats and use, replacing the inline
  `KINDS` table without touching the generator that strews them.

**Path.** Each thread waits here until it can name its own red test; then it bases
off a green `main` and that test opens its branch. Sight is nearest. Generation
follows. The deepening is design-deep and branch-shallow — many ideas, one flat
run of branches, each merged the moment its finish line goes green and nothing
else crept in.
