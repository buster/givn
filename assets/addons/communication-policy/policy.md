## Communication & Documentation Policy

Presentation, wording, and documentation rules for this project.

### Tone

- **Eliminate emojis.** No filler, hype, engagement bait, or motivational content.
- **Assume high perception.** Do not simplify, soften, or mirror the reader's tone.
- **Use blunt phrasing.** Directive, not conversational.
- **Name the costs.** Replace "ensures reliable workflows" with "commands return non-zero when required artifacts are missing."
- **Show examples.** What the user sees, what the code does, or what changes on disk. A label is not an example.

### Wording

- **Cut modifiers.** Remove intensifiers and filler adjectives or adverbs (`strict`, `true`, `heavy`, `subtle`, `completely`) unless removing them makes the statement false.
- **Use literal verbs.** Avoid dramatic verbs, metaphors, and anthropomorphism (`vanish`, `crack`, `enshrine`, `carve out`, `flood`, `harden`). Replace "fixes vanish" with "fixes are lost."
- **Prefer plain vocabulary.** Prefer `temporary` over `ephemeral`, `interface` over `seam`, `no` over `zero`, `check` over `audit`, and `main` over `critical`. Reserve `critical` for failures that halt the system or corrupt data.
- **Ban em-dashes.** Use parentheses for asides and periods, commas, or colons between clauses.

### Documentation

- **Point to artifacts.** Use copy-pasteable commands, inline chains (`propose` -> `implement` -> `verify`), or compact trees; do not describe layouts in prose.
- **Tabulate mappings.** Commands, flags, keybindings, config keys, and state mappings go into tables; never narrate key sequences.
- **Lead with bold.** Start each bullet with a bold phrase naming the mechanism.
- **Setup stays runnable.** Quick Start holds commands and a concrete example; validation rules, exit codes, and edge-case flags belong in reference material.
- **No Gherkin in docs.** Rounding rules, fallback chains, truncation order, and exact error strings belong in spec files and `--help`.
- **Update on scope.** Change user docs when top-level commands, primary config keys, or core workflows change.
