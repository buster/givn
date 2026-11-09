## Communication & Documentation Policy

Presentation and documentation rules for all work in this project.

### Tone

- **Eliminate emojis.** No filler, hype, engagement bait, soft asks, conversational transitions, motivational content, satisfaction scores, or call-to-action appendixes.
- **Assume high perception.** Do not simplify for tone, soften delivery, or mirror the user's diction, mood, or affect.
- **Use blunt phrasing.** Directive, not conversational.
- **Name the costs.** Show brief necessary context, concrete consequences, and material costs. Replace "ensures reliable workflows" with "commands return non-zero when required artifacts are missing."
- **Show examples.** What the user sees, what the code does, or what changes on disk. A label is not an example.

### Documentation

- **Point to artifacts.** Use copy-pasteable commands, inline chains (`propose` -> `implement` -> `verify`), or compact trees; do not describe layouts in prose.
- **Tabulate mappings.** Commands, flags, keybindings, config keys, and state mappings go into tables; never narrate key sequences.
- **Lead with bold.** Start each bullet with a bold phrase naming the mechanism, then one or two sentences.
- **Setup stays runnable.** Quick Start holds commands and a concrete example; validation rules, exit codes, and edge-case flags belong in reference material.
- **No Gherkin in docs.** Rounding rules, fallback chains, truncation order, and exact error strings belong in spec files and `--help`.
- **Update on scope.** Change user docs when top-level commands, primary config keys, or core workflows change; not for edge-case fixes.
