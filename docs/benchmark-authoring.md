# Adding a benchmark

A benchmark is a pinned task package, not a prompt in a string. The same runner,
grader and bundle format serve a small two-stage workflow and a longer
brownfield evolution.

## Manifest

`benchmark.toml` declares:

- `schema = 1`, `id`, `version`, `title`, `description`;
- `[workspace]` — exact `public_files`, a `scaffold` directory, and a public
  `probe` command;
- `[agent]` — allowed model and output bound;
- `[[stages]]` — id, task, timeout, `requires`, `require_archive`,
  `require_probe`, optional stage-specific `probe`;
- `[grader]` — a private grader manifest with a command and timeout;
- `[verification]` — a trusted reference and seeded defect patches, each with
  the requirement ids it must trip.

Paths are relative to the benchmark root. Public files must resolve under the
root and must not be under `private/`. Only the listed public files and scaffold
are copied into a trial workspace; no sibling file is selected by substring.

## Public and private material

- **Public**: the task, the full interface contract, and narrow smoke probes.
  Every independently graded obligation must appear in the public contract.
- **Private**: reference implementation, hidden test cases, seeded defects.
  Hidden tests may vary combinations and boundaries, never unstated
  expectations.
- A task may reference an open choice. If it has an unambiguous recommended
  option, the answer policy may accept it; an arbitrary agent-written decision
  never becomes an oracle.

## Grader contract

The grader receives:

```sh
<grader-command> --bundle BUNDLE --benchmark BENCHMARK --output RESULT.json
```

It must write `givn-harness/grade/v1` JSON:

- `requirements[]` — unique `id`, `description`, `stage`, `critical`, and
  `status` in `pass | fail | ungraded`;
- `summary` — required/graded/passed/failed/ungraded counts and critical
  failures.

A grader may execute the submission only through its documented black-box
entry point. It must not import submission code. It reads `stages/<id>/snapshot`
and must not modify it. `ungraded` means the stage was not reached or the
evidence is missing; it is neither pass nor fail.

## Verification gate

`givn-harness benchmark verify <benchmark>` builds synthetic bundles and
requires:

1. the reference passes every required check;
2. the empty scaffold fails at least one substantive check;
3. every seeded defect trips its declared requirement ids.

Generate defect patches against the reference with `diff -u` using `a/` and
`b/` labels so `patch -p1` applies them from the copied reference directory.

## Cross-version state

For a follow-up benchmark, create state through the initial snapshot and read it
through the follow-up snapshot on the same data directory. Creating and
returning a new loan only under the follow-up version does not prove that
persisted data survived the change.
