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
- The acceptance suite requires `git`, `fossil`, `git-cliff`, and `script`
  (util-linux) on PATH (the archive scenarios exercise a real Fossil checkout,
  the changelog scenarios generate a release section with the pinned
  `git-cliff 2.13.1`, and the terminal-prompt scenario drives the interactive
  `givn init` prompt through a real PTY).
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
  search, grep, or inspect files to determine what to do next — this command
  reports the current project state and the next lifecycle step, including
  `--change <CHANGE>` for active changes. A reported step applies when
  continuing that change; it does not authorize unrelated work or require every
  task to become a Givn change.
- For new or unclassified work, load `givn-start` to choose answer only, direct
  work without a Givn change, ideation, or a formal change. For a change whose
  goal and scope are already clear, run `givn new <change-id>` directly. Honor
  an explicitly requested, applicable path without another routing conversation.
- For every user-facing question, follow the effective dialog policy from
  `givn instructions dialog`; load it once per context before the first
  question, including clarification and confirmation outside a named
  workflow, and pass the obligation explicitly when work is handed to another
  context. This obligation applies with optional addons disabled and outside
  any named skill; the presenting agent remains responsible for the question.
  Before the first question of a type indexed by the policy, read that type's
  companion examples from `givn instructions dialog-examples`; retained
  examples need no reread. The policy is not loaded automatically.
- Generated guidance is skills-only: Claude Code and OpenCode use their
  native skill invocation, the neutral `.agents` target uses skill discovery,
  OMP uses `/skill:givn-<step>`, and Maki uses an explicit request or an
  agent-selected skill. No generated Givn command files exist.
- Run `givn status --change <CHANGE>` for a full artifact checklist of an
  active change (includes task progress and pending task descriptions).
- Tick off each scenario task when its batch is complete — not when the code
  merely compiles. Never batch-check boxes at the end.
- One atomic `givn commit` per coherent behavior batch, not per scenario. The
  revision message carries the release note as its message subject and names the batch
  in the body. For a single-scenario batch, the scenario title goes in the message body.
  A raw Git commit is stamped by the hook, a raw Fossil check-in is
  not.
- Each checked scenario records the batch `COMMIT:` revision. The commit must
  contain the scenario's implementation or regression test; a comment-only,
  tag-only, task-only, or feature-only change is not evidence.
- Tests may pass immediately when they protect a distinct requirement. Do not
  force a production diff per scenario. Deduplicate or parameterize genuine
  repetition.
- The full workflow and available skills are documented in the output of
  these commands — this block is intentionally just a pointer.
<!-- givn:end:givn-project -->

<!-- givn:begin:communication-policy -->
## Communication & Documentation Policy

Presentation, wording, and documentation rules for all work in this project.

### Tone

- **Eliminate emojis.** No filler, hype, engagement bait, soft asks, conversational transitions, motivational content, satisfaction scores, or call-to-action appendixes.
- **Assume high perception.** Do not simplify for tone, soften delivery, or mirror the user's diction, mood, or affect.
- **Use blunt phrasing.** Directive, not conversational.
- **Name the costs.** Show brief necessary context, concrete consequences, and material costs. Replace "ensures reliable workflows" with "commands return non-zero when required artifacts are missing."
- **Show examples.** What the user sees, what the code does, or what changes on disk. A label is not an example.

### Wording

- **Cut modifiers by default.** Remove intensifiers and filler adjectives or adverbs (`strict`, `true`, `heavy`, `subtle`, `completely`, `solely`, `immediately`, `silently`, `blindly`, `total`, `enduring`) unless removing them makes the statement false. Replace "verify heavy domain permutations in milliseconds" with "verify domain permutations in milliseconds."
- **Use literal verbs.** Avoid dramatic verbs, metaphors, and anthropomorphism (`vanish`, `crack`, `enshrine`, `carve out`, `flood`, `harden`, `secretly depend`, `let it die`). Replace "subtle fixes hidden in old code will vanish" with "performance fixes in old code are lost."
- **Prefer plain vocabulary.** Avoid coined buzzwords (`zero-share`) and clichés (`big-bang`, `brownfield`, `tribal knowledge`, `out of the box`); prefer `temporary` over `ephemeral`, `interface` or `boundary` over `seam`, `no` over `zero`, `check` over `audit`, and `main` or `important` over `critical`. Reserve `critical` for failures that halt the system or corrupt data.
- **Ban em-dashes.** Do not use `—` or `--` for prose breaks. Use parentheses for asides, and periods, commas, or colons to separate clauses.

### Documentation

- **Point to artifacts.** Use copy-pasteable commands, inline chains (`propose` -> `implement` -> `verify`), or compact trees; do not describe layouts in prose.
- **Tabulate mappings.** Commands, flags, keybindings, config keys, and state mappings go into tables; never narrate key sequences.
- **Lead with bold.** Start each bullet with a bold phrase naming the mechanism, then one or two sentences.
- **Setup stays runnable.** Quick Start holds commands and a concrete example; validation rules, exit codes, and edge-case flags belong in reference material.
- **No Gherkin in docs.** Rounding rules, fallback chains, truncation order, and exact error strings belong in spec files and `--help`.
- **Update on scope.** Change user docs when top-level commands, primary config keys, or core workflows change; not for edge-case fixes.
<!-- givn:end:communication-policy -->
