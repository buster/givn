# Change seed: author-change

Status: implemented in `author-change`

- Use-case ID: `author-change` (existing, modified), including the `support` /
  `llm-attribution` fragment capability
- Topic: `multi-scm-support`
- Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner`
- Suggested start: after `scm-backend`
- Handoff: ready for `/givn-propose` (historical)

## Accepted decisions

### Q2 — Attribution parity

- Context: Git stamps commits through a chained `commit-msg` hook; Fossil
  2.26 offers only `before-commit` and `after-receive`, and a hook cannot
  rewrite the committer.
- Options:
  1. **`givn commit` is the SCM-neutral path; Git keeps the chained hook as
     a fallback; Fossil attribution exists only through `givn commit`.**
     In practice: `givn commit -m "feat: parser"` stamps the check-in comment
     on both backends; raw `git commit` still carries attribution; raw
     `fossil commit` does not, and the guidance says so.
     Gain: one documented path with honest per-backend limits.
     Cost: Fossil users must use `givn commit` for attribution.
  2. **Build a Fossil hook that rewrites the check-in comment.**
     In practice: impossible with the available Fossil hooks.
     Gain: parity by mechanism.
     Cost: not implementable.
  3. **Drop attribution on Fossil.**
     In practice: Fossil projects never carry the trailer.
     Gain: no wrapper.
     Cost: an asymmetric lifecycle.
- Chosen: **Option 1** because it keeps the trailer format unchanged and
  states the limits. Rejected: Options 2 and 3.

### Q7 — Initialization without an SCM

- Context: `givn init` can run in a plain directory with no backend.
- Options:
  1. **Scaffold and exit 0 with a named warning** — archive and managed
     upgrade are unavailable.
     In practice: skills are generated; a later `givn archive` refuses by
     name.
     Gain: a usable corpus without a backend.
     Cost: some lifecycle commands refuse later.
  2. **Refuse initialization** — require a backend.
     In practice: plain directories cannot start.
     Gain: no partial lifecycle.
     Cost: blocks exploratory use.
- Chosen: **Option 1** because initialization is safe without a backend.
  Rejected: refusing.

## Open questions

### Unregistered capability

Resolved during spec authoring: the `llm-attribution-installation`
capability was registered or folded.

## Scope note

Backend-aware initialization report, conditional attribution plumbing,
guidance that names only commands valid for the active backend, and the
`givn commit` wrapper. Modified the init and guidance scenarios in
`givn/specs/author-change/` and
`givn/specs/fragments/llm-attribution.feature`.
