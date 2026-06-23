# ADR 0001 — Identity is an EntityId, nothing more

- **State:** decided (was a Thought)
- **Kind:** Thought
- **Altitude:** engine / entity model
- **Date:** 2026-06-23
- **Enacted:** 06697d9

## The QTCC

Should an `Entity` also carry a readable name or label beside its `id`?

## The decision

No. `Entity.id` is a bare `EntityId` (`u64`). The readable name lives in the
being's `.being` data, looked up by id. The player is the one fixed id
(`PLAYER = 0`); everything else is found by id or by position.

## Why

The event bus moves ids, not strings (ward 1). A name on the `Entity` would be a
second home for a fact that already lives in the being data — and a second home
is a place to drift. Beings already carry their name; nothing but the player
needs a handle known by literal.

## If this is wrong

Adding a `name`/label to `Entity` is safe later — the no-`String` ward binds
`Event`s, not `Entity`. Reopen as a QTCC if debug or display wants names back.
