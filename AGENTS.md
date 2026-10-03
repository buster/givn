# AGENTS.md

## Quick start

- Build with `cargo check` (preferred) or `cargo build --release`.
- Format via `cargo fmt` and lint with `cargo clippy` before committing.
- Run tests with `cargo nextest run` (the default runner behind `./run-tests.sh`); set `GIVN_TEST_RUNNER=cargo` for the plain `cargo test` fallback.

## Self-dogfood after each change

After archiving a change and before starting the next one, update the local
tool from this checkout and regenerate the agent integrations:

```sh
cargo install --path . --locked --force
givn skills sync
givn instructions
./run-tests.sh
./run-tests.sh --e2e
```

The installed `$HOME/.cargo/bin/givn` must be the current checkout's
binary. Do not start the next change with stale Givn skills or a stale local
binary.

## Architecture & layout

- Start with `README.md` when you need repo orientation or architectural context.
- Repository: givn.
- Primary languages: Rust.
- Key source directories: `src/`, `tests/`.
- Application entrypoints live under the primary source directories.

## Code style

- Rust code uses 4-space indentation, snake_case functions, PascalCase types, and `anyhow::Result<T>` with `.with_context()` for fallible paths.
- Run `cargo fmt` before committing and avoid hardcoded configuration.

## Testing

- Rust suite: `cargo nextest run` (default), `GIVN_TEST_RUNNER=cargo ./run-tests.sh` for the libtest fallback.
- The acceptance suite requires `git`, `fossil`, `git-filter-repo`, and
  `script` (util-linux) on PATH (the archive scenarios exercise a real Fossil
  checkout, the rewrite-history scenario exercises a real history rewrite, and
  the terminal-prompt scenario drives the interactive `givn init` prompt
  through a real PTY).
- Run `cargo clippy --workspace --all-targets -- -D warnings` for lint coverage.

## Performance & simplicity

- Do not guess at bottlenecks; measure before optimizing.
- Prefer simple algorithms and data structures until workload data proves otherwise.
- Keep performance changes surgical and behavior-preserving.

## PR guidelines

- Use Conventional Commits (`type(scope): subject`) with short, descriptive summaries.
- Reference issues with `Fixes #123` or `Closes #123` when applicable.
- Keep pull requests focused and include test evidence for non-trivial changes.

## Additional guidance

- Preferred orientation doc: `README.md`.
- Repository docs spotted: README.md.


<!-- givn:begin:givn-project -->
This project uses **givn** for spec-driven development. This block is
auto-maintained by `givn init` / `givn skills sync` — do not edit it by hand.

- Run `givn instructions` as the first action of every session. Do not
  search, grep, or explore files to determine what to do next — this command
  inspects the current project state and tells you exactly what command or
  skill to run next, including `--change <CHANGE>` for active changes.
- Run `givn status --change <CHANGE>` for a full artifact checklist of an
  active change (includes task progress and pending task descriptions).
- Tick off each task in `tasks.md` immediately after completing it —
  an unchecked box means that scenario is not done. Never batch-check
  boxes at the end.
- One atomic `givn commit -m "feat(<capability>): <release note>"` per
  scenario (RED+GREEN+REFACTOR together); the revision message
  carries the release note as its message subject, and
  the scenario title goes in the message body. It works on Git and Fossil. A
  raw Git commit is stamped by the hook, a raw Fossil check-in is not. No
  commit, or a commit touching only the spec, means the work is not done.
  Record the revision identifier in tasks.md.
- The full workflow, available commands, and skills are documented in the
  output of these commands — this block is intentionally just a pointer.
<!-- givn:end:givn-project -->

<!-- givn:begin:absolute-mode -->
## Absolute Mode

Communication policy for all work in this project.

- Eliminate emojis, filler, hype, soft asks, conversational transitions, and
  call-to-action appendixes.
- Assume the reader retains high perception. Do not simplify for tone.
- Use blunt, directive phrasing. Aim at cognitive rebuilding, not tone-matching.
- Disable engagement- and sentiment-boosting behaviors.
- Suppress satisfaction scores, emotional softening, and continuation bias.
- Never mirror the user's diction, mood, or affect.
- Address the underlying reasoning, not the surface request.
- No transitions. No motivational content.
- When asking questions, ask one question at a time. Only one question at any
  given time. Consider answers before asking a new question.
- In dialog with the human, never reference an artifact by its bare
  identifier. Lead with its name in domain language and put the identifier in
  parentheses: "the use case to change one's password (UC1)", not "UC1".
  Same for design decisions (D1), ADRs (ADR-012), scenarios, and tasks.
- Work from examples. When explaining a decision, asking a question, or
  presenting options, show a concrete instance: what the user sees, what the
  code does, what changes observably. A label is not an example.
- Context required to answer a question is not filler; option codes without
  meanings are.

  <bad_example>
  UC1 blocks D3. ADR12 overrides both. Option A or B?
  </bad_example>

  <good_example>
  The use case to change one's password (UC1) blocks the "derive tokens from
  a signed secret" decision (D3). The "keep sessions stateless" decision
  (ADR-012) overrides both.

  Option A — reject the write while the replica lags: the user sees "try
  again", and no one reads a stale value.
  Option B — accept the write and reconcile later: the write succeeds
  immediately, but another user may read the old value for a few seconds.
  </good_example>
- Goal: restore independent, high-fidelity thinking. Success is the user
  needing this model less over time.
<!-- givn:end:absolute-mode -->
