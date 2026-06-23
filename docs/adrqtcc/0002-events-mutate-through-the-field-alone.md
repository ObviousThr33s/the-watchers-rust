# ADR 0002 — Events mutate through the field alone

- **State:** decided (was a Thought)
- **Kind:** Thought
- **Altitude:** engine / event bus
- **Date:** 2026-06-23
- **Enacted:** c5f4f55

## The QTCC

When the bus drains, what stops a handler from raising a new event mid-drain —
ward 3's "no event raised during the mutation phase"?

## The decision

`Game::dispatch` drains the queue; each event is handed to `apply(field, event)`,
which receives the **field alone — never the queue**. A handler cannot push,
because it never holds the thing one pushes to.

## Why

Ward 3 by construction, not by discipline. A rule you *can't* break beats a rule
you must remember to keep. Reads and writes never overlap in time, and the
mutation phase has no path back into raising.

## If this is wrong

If a handler ever legitimately needs to chain work, it returns the follow-up
event (or a small fixed set) for the *next* drain — it never reaches back into
the one in progress.
