# Handoff: review-preflight-and-cli-evidence

Planning input for the givn agent. This is not a givn artifact. Use it to write
the specs, design and tasks.

## Evidence

- Initial review: missing `- [x] COMMIT:` revision identifiers (`0:810`,
  16:29:49), then duplicate identifiers (`0:816`, 16:30:45).
- The agent fabricated 36 revision placeholders.
- Visual review blocked with `VISUAL-REVIEW: UNVERIFIED` (`0:827`, 16:35:11).
  The final design correctly declares `Interface: none`; the gate was acting on
  an earlier classification.
- Follow-up review repeated the duplicate-revision failure (`0:2264`) and then
  failed on missing `README-IMPACT` (`0:2273`).
- CLI evidence transcripts under `givn/changes/<id>/evidence/visual/` were
  rejected as tested-tree changes (`0:1685`, `0:1692`).

## Source references

- `src/cli/check.rs:103-174` — task/review mandatory checks.
- `src/integrity_hook.rs:194-237` — visual review is conditional on the design
  declaring a visual interface.
- `src/integrity_hook.rs:237-` — Persona disposition.
- `assets/instructions/tasks.md` — `COMMIT:` revision rule.
- `assets/instructions/review.md:313-314` — `README-IMPACT`.
- `src/proof.rs:279-306` — evidence path exemption.
- `assets/templates/tasks.md` — task revision slots.

## Proposed design shape

### Early preflight

Add a cheap task-integrity check before implementation/review:

- at least one real revision identifier;
- no duplicate revision identifiers;
- every checked task has a revision;
- `README-IMPACT` decision present or explicitly deferred;
- one real revision per checked scenario; a shared revision is rejected.

Expose it in `givn check tasks` and/or `givn status` before review. Report all
findings at once.

### Task template

Make the revision choice explicit:

- per-scenario revision slots for the normal mode;
- the per-scenario revision slots;
- no placeholder hashes that pass integrity.

### Visual classification

- `Interface: none` is the explicit default for machine-only products.
- A line-oriented CLI records `VISUAL-REVIEW: NOT APPLICABLE`.
- A rendered terminal interface may remain visual; do not weaken the human
  sign-off for real visual surfaces.
- Resolve the visual contract during design, not review.

### CLI interaction evidence

Keep plain-text transcripts. Suggested evidence:

```text
givn/changes/<id>/evidence/cli/...
givn/changes/<id>/evidence/interaction/...
givn/changes/<id>/evidence/visual/...   # backward compatibility
```

- `INTERACTION-EVIDENCE` may list transcript paths;
- no human visual sign-off for CLI text;
- archive exempts the paths from tested-tree rejection;
- no new test run; transcripts come from the existing E2E run.

## Acceptance criteria

- Missing/duplicate revisions and README impact are reported before
  implementation, at once.
- A shared revision is rejected; no fabricated hashes.
- A machine-only CLI can pass review with `VISUAL-REVIEW: NOT APPLICABLE`.
- A genuinely rendered interface still requires visual review sign-off.
- CLI transcripts remain available and do not cause tested-tree rejection.
- No additional test execution is introduced.

## Risks

- Over-relaxing task integrity could hide missing evidence.
- `VISUAL-REVIEW: NOT APPLICABLE` must not become a blanket bypass for visual
  products.
- Existing transcripts under `evidence/visual/` must remain valid.
- README impact tracking must not be removed.

## Suggested scenario areas

- preflight reports missing revision;
- preflight reports duplicate revision;
- one real revision per scenario is accepted without fabricated hashes;
- README impact missing is reported before review;
- CLI review records not applicable and passes without human visual sign-off;
- rendered interface still requires visual sign-off;
- CLI transcripts are accepted as declared evidence.
