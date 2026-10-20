# Tasks — review-preflight-and-cli-evidence

IMPLEMENTATION-MODE: one-pass
ONE-PASS-COMMIT: e7ae7928a6f9144e10b164293b19bd65c44794c8

Release note: Surface task-integrity and README-impact requirements before implementation, and preserve plain-text CLI interaction evidence without forcing a visual-review sign-off.

The implementation was delivered in one pass: the production and test changes
share `e7ae7928`; each task below replays that revision.

- [x] Setup and runner contract: `cargo check --tests`, `cargo test --lib` → passing.
  - [x] COMMIT: e7ae7928a6f9144e10b164293b19bd65c44794c8 — feat(review-preflight): one-pass implementation
- [x] Task preflight reports duplicate revisions and missing README-IMPACT in one pass.
  - [x] COMMIT: e7ae7928a6f9144e10b164293b19bd65c44794c8 — feat(review-preflight): preflight findings
- [x] A disclosed one-pass implementation passes review integrity.
  - [x] COMMIT: e7ae7928a6f9144e10b164293b19bd65c44794c8 — feat(review-preflight): one-pass disclosure
- [x] A machine-only CLI records `VISUAL-REVIEW: NOT APPLICABLE` and passes review.
  - [x] COMMIT: e7ae7928a6f9144e10b164293b19bd65c44794c8 — feat(review-preflight): not-applicable CLI review
- [x] CLI interaction transcripts are accepted as declared evidence at archive.
  - [x] COMMIT: e7ae7928a6f9144e10b164293b19bd65c44794c8 — feat(review-preflight): CLI transcript evidence
- [x] Full-suite readiness: `./run-tests.sh` and `./run-tests.sh --e2e` passing; lint clean.
  - [x] COMMIT: e7ae7928a6f9144e10b164293b19bd65c44794c8 — feat(review-preflight): readiness
