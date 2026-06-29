# Ideologue — the liminal fade ("the end of the universe")

*An idea that has earned its finish line, and so may earn a branch. Companion to
the lore seed `../lore/mnemosyne-and-lethe.md`. Per METHOD.md, an idea waits in an
ideologue until it can name what **done** means — this one now can.*

## The idea

There is a place at the end of the universe where forgetting never completes. A
being remembered there is held forever — its **place still**, its **clarity
forever moving** toward a zero it never reaches. Movement, but so liminal it
never arrives.

Today `Recollection` fades **linearly**: clarity loses a fixed step each unseen
tick and, after enough ticks, reaches zero and is forgotten (`recollection.rs` ::
`fully_faded_is_forgotten`). That is the rest of the world. The end of the
universe is the exception — a fade that only ever *approaches* zero.

## The finish line

> `tests/finish_line_worlds_end.rs` ::
> `at_the_universes_end_a_memory_lingers_still_and_never_arrives`

One red test, the merge condition. It asks for a `Recollection::at_worlds_end()`
whose memory, left unseen for a hundred thousand ticks:

- keeps its **place** exactly (still),
- begins to fade from full clarity (it moves — not frozen at full), and
- is **never wholly forgotten** — clarity stays above zero throughout.

It is red now for two honest reasons: the symbol does not exist, and the linear
fade would reach zero. Green is the day a liminal fade exists and the test passes
— nothing more, nothing less.

## Design sketch (mechanism — for the build phase, not yet decided)

- **Additive.** A new constructor `Recollection::at_worlds_end()`; the existing
  `new(fade)` and its linear behaviour stay untouched, so `fully_faded_is_forgotten`
  stays green. Two fade laws, one type.
- **Approach without arrival.** Remove a *fraction of what remains* rather than a
  fixed step, and clamp so clarity may rest at the threshold but never cross into
  zero. (`f32` cannot strictly decrease forever and stay positive — the liminal
  floor is honest, not a cheat. The floor *is* the end of the universe.)
- **Open question:** whether "the end of the universe" is a property of a
  `Recollection`, a single `Sighting`, or a *place* in the field. The finish line
  pins only the observable, not the shape — settle the shape in the build and
  record it as a `docs/adrqtcc/` entry when it does.

## Path

Base off a green `main`; the red test opens the branch; green merges it. When the
shape settles it graduates to an `adrqtcc` ADR. Until then it waits — but now it
waits with a finish line.
