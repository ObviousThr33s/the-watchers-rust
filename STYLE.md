# Obelisk — Style & Engineering Standard

A small, enforced house standard in the spirit of the **JSF AV C++** rules: every
rule carries a *rationale*, rules are *tiered* by how hard they bite, and we
*program in a safe subset* — banning the footguns rather than trusting discipline.

Adapted for **hosted Rust**: we keep JSF's structure (rationale + tiers + subset)
and reject its avionics content. This is a `ratatui` TUI; the heap is a structural
requirement, not a luxury. So the rule is *invert JSF's instinct* — ban the
footguns Rust left in, keep the power Rust added.

Most of this is machine-checked (`Cargo.toml` `[lints]`, `clippy.toml`,
`rustfmt.toml`). The prose below is only for what the tools can't see.

## Tiers

Rust's lint levels **are** the tiers — the compiler is the verification process
JSF had to run by hand.

| Tier | Mechanism | Meaning |
|------|-----------|---------|
| **Shall** | `deny` / `forbid` | The build fails. Deviation is a reviewed, explicit `#[allow]`. |
| **Will** | `warn` | Visible every compile. Fixed at leisure. |
| **Should** | this document | What tools can't check; held by review. |

**The ratchet:** the policy currently sits almost entirely at `warn`, so the build
stays green while the code is cleaned. A lint graduates to `deny` once the code
that violates it is fixed (the `as` casts are the main holdout). The one rule
already at full strength is `unsafe_code = "forbid"` — there's nothing to clean,
and there never should be.

> Clippy lints fire under `cargo clippy`, not `cargo build`. Test carve-outs live
> in `clippy.toml`.

## The safe subset

Ban the footguns; keep iterators, closures, and traits.

- **No `unwrap` / `expect` / `panic!`** — a TUI should degrade, not crash. Return
  `Result`/`Option` and handle it. *Carve-out: tests may `unwrap`/`expect` — a
  failed `expect` is the assertion.*
- **No raw indexing** (`buf[i]`) — it panics on a bad index. Use `.get(i)` and
  handle `None`.
- **No lossy casts** — the raycaster/render math is full of `as` between
  `i16`/`usize`/`f32`; each is a silent truncation. Use checked conversions, or an
  `#[allow]` with a one-line proof at the math boundary.
- **Borrow before you clone** — pass `&T` / `&str`; clone only when ownership
  genuinely demands it.
- **No `unsafe`** — only ever for FFI or memory-mapped hardware, neither of which
  exists here. If it ever does: the narrowest possible block, preceded by a
  `// SAFETY:` proof of the invariant.

## Memory & allocation (hosted)

- `std` and the heap are expected — `Vec`/`String`/`HashMap` are how the UI holds
  state.
- **Don't allocate in hot loops.** A `ratatui` draw frame runs constantly;
  rebuilding a `Vec`/`HashSet`/buffer every frame is the thing to avoid. (Known
  offenders flagged for the render rewrite: `ui::draw_center`,
  `Field::to_ascii_map`, the raycaster.)

## Errors

- Fallible functions return `Result<T, E>`.
- `E` is a **typed enum** for domain logic — never a `String` or `Box<dyn Error>` —
  so a caller can match the cause. See `BeingError`.
- Handle every `Option`/`Result` via `match` or `?`. No silent discards.

## Naming & API shape

- `impl Display`, not a hand-rolled `impl ToString` — you get `.to_string()` free
  and it composes (API Guidelines C-CONV).
- Getters borrow and drop the `get_` prefix: `version(&self)`, not
  `get_version(self)` (C-GETTER).
- New names follow Rust convention (`CamelCase` types, `snake_case` items).
  Existing house quirks (`self_`, `Direction_`) are grandfathered — don't add more.

## Tests as documentation

Tests **are** the spec while the engine is rebuilt.

- Name tests as behaviour sentences — `unseen_she_freezes_in_place_and_fades`.
- Keep them **truthful**: no stale or commented-out scaffolding. A test whose
  comment claims work it no longer does is anti-documentation.
- Use **doc-tests** (` ```rust ` in `///`) for public API — executable
  documentation that `cargo doc` renders and `cargo test` runs. `Being::parse`,
  `Screen`, and `vision::can_see` carry them.

## Formatting

- `rustfmt` owns layout; `rustfmt.toml` pins **hard tabs** to keep the house look.
- The full `cargo fmt` sweep is **deferred** — it touches nearly every file, so it
  lands as its own deliberate commit, never folded into feature work.

## Running it

```sh
cargo clippy        # the safe subset + quality lints
cargo test          # unit tests + doc-tests
cargo fmt --check   # formatting (once the sweep lands)
```
