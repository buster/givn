# Change seed: rewrite-history

Status: proposed by rewrite-history

- Use-case ID: `rewrite-history` (new)
- Topic: `local-only-integrations`
- Personas: `developer--primary-user`, `givn-maintainer--owner`
- Suggested start: second; depends on `local-only-rules` for the report that
  names the occurrences
- Handoff: ready

## Accepted decisions

### Q2 — Report versus rewrite

- Context: past revisions can still contain strict rules even after the
  current revision is clean.
- Options:
  1. **Report plus an explicit opt-in rewrite command.**
     In practice: the report names the paths; the developer runs the rewrite
     deliberately.
     Gain: a real guarantee without silent destructive operations.
     Cost: a new command with safety rules.
  2. **Report only.** Gain: smallest scope; cost: no remediation in Givn.
  3. **Report and refuse.** Gain: strong gate; cost: blocks unrelated work.
- Chosen: **report plus explicit rewrite** because the user chose it.
  Rejected: report-only; report-and-refuse.

### Q20 — Mechanism

- Context: rewriting history is destructive and needs a dependable tool.
- Options:
  1. **`git-filter-repo` when installed; refuse by name with install
     guidance when absent.**
     In practice: the opt-in command is fast and correct; no fallback.
     Gain: modern, purpose-built tool; no dependency for the normal
     lifecycle.
     Cost: one external tool for this command.
  2. **Built-in `git filter-branch`.**
     Gain: no external tool; cost: deprecated, slow, warning noise.
  3. **Own plumbing implementation.**
     Gain: full control; cost: the largest risk in a destructive path.
- Chosen: **`git-filter-repo`, refuse if absent** because the user chose it.
  Rejected: filter-branch; own plumbing.

### Confirmation

- Chosen: `--yes` proceeds; a terminal prompts after showing the paths and
  revision count; piped or closed input without `--yes` refuses.
  Rejected: flag-only; no confirmation.

### Fossil

- Chosen: refuse by name; Givn does not rewrite Fossil history.
  Rejected: silent no-op; implementing Fossil rewriting.

## Open questions

- Q20 remainder (open): backup policy for original refs and the exact
  force-push guidance wording.
- Q2 boundary (confirmed): unreachable objects, remotes, and forks stay
  outside the guarantee.

## Scope note

Introduces the permanent use case `givn/specs/rewrite-history/` with
capability `history-rewrite`: clean-state preflight, reachable-history scan,
confirmation, Git rewrite through `git-filter-repo`, verification, new
revision identifiers, and the boundary/force-push report. Fossil refuses by
name. It ships separately from `local-only-rules` because it is destructive,
irreversible, and gated on an external tool; the core change only reports and
points here.
