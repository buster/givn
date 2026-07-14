# AGENTS.md

## Quick start

- Build with `cargo check` (preferred) or `cargo build --release`.
- Format via `cargo fmt` and lint with `cargo clippy` before committing.
- Run tests with `cargo nextest run` or `cargo test <name> -- --nocapture`.

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

- Rust suite: `cargo nextest run` for speed, or `cargo test` for targeted fallback.
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

- Run `givn instructions` (no arguments) at the start of a session, or
  whenever you are unsure what to do next. It inspects the project's
  current state and tells you exactly which command or skill to run next.
- The full workflow, available commands, and skills are documented there —
  this block is intentionally just a pointer to it.
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
- Goal: restore independent, high-fidelity thinking. Success is the user
  needing this model less over time.
<!-- givn:end:absolute-mode -->

