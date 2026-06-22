# Test catalogs — polymorphic integrity

Every file in this folder is **generated**, never hand-written. They are many
shapes of one truth: the test suite in `tests/`.

- `content-authors.md` — the `.being` safety contract
- `contributors.md` — the whole map, one sentence per guarantee
- `reviewers.md` — each sentence next to its `///` rationale

**Polymorphic integrity** is the rule that holds them together: a guarantee may
appear in many forms for many audiences, but all forms derive from a *single*
source, so they cannot drift out of agreement. Change a test and every view
changes with it; there is no second copy to forget. Edit a `.md` by hand and you
have broken the integrity — the form now lies about the source.

To regenerate (run from the repo root):

```
powershell -File scripts\tests-content.ps1
powershell -File scripts\tests-contributors.ps1
powershell -File scripts\tests-reviewers.ps1
```

The reader is `scripts\_catalog.ps1` — a pure read over `tests\*.rs`. It never
touches a test. The tests remain the one source of truth; these are its echoes.
