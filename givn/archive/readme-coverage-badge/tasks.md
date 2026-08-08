# Tasks: readme-coverage-badge

## RED / GREEN / REFACTOR

1. - [x] **RED**: Scenario written with `@e2e @wip`; first run failed (`archive skill missing the badge-placement rule`) because the guidance lacked the leading-badge rule.
2. - [x] **GREEN**: Updated `assets/addons/readme/README.md.tmpl` and `assets/skills/givn-archive/SKILL.md.tmpl`; added step module `tests/steps/readme_coverage_badge_steps.rs`; registered in `tests/steps/mod.rs`; rebuilt binary and ran `givn skills sync`; updated repo `README.md` badge. Scenario passes (4 steps).
3. - [x] **REFACTOR**: Removed `@wip`; `cargo fmt`; `cargo clippy --workspace --all-targets -- -D warnings` clean.