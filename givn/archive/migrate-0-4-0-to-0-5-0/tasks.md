# Tasks: migrate-0-4-0-to-0-5-0

Operational migration record. The 0.5.0 contract work itself was shipped
and verified across the retirement-plan changes; this change records the
upgrade execution against this repository.

- [x] Prompt 2: run `givn upgrade` on the 0.5.0 binary; Commit A touches only the config marker; Commit B only this change; `migration.yaml` records Commit A.
  - [x] COMMIT: `e9938d6`
  - [x] COMMIT: `817e948`
- [x] Prompt 3: regenerate guidance (`givn skills sync`, `givn instructions`) and verify the single project-owned override (arc42 impact template) is current for 0.5.0.
  - [x] COMMIT: `3b55389`
- [x] Prompt 4: record the project decisions — semantic features (opt-out default, opt-in documented), grouped layout, retirements, inventories, disposition vocabulary, protected artifact, routing (all pre-applied in this repo; see design-review evidence table).
  - [x] COMMIT: (this artifacts commit)
- [x] Prompt 5: verify — `./run-tests.sh` 354/354, `cargo test --no-default-features` green, `givn lint` clean, `givn spec tree/find/duplicates/route` operational — then archive this change.

DESIGN-REVIEW: PASS marker lives in design-review.md.
