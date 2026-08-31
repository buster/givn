# Tasks: release-override-hardening

## S1–S2: release-proof scenarios (delta feature)

- [x] S1 `An installed givn announces each refused request by name` (@e2e
      @release-proof): fixture-request representatives for two channel
      classes (simulated embedder, pair-budget resize); both announcements
      asserted on the release binary's stderr plus the real-model refusal.
      GREEN-first (pins existing sweep + refusal behaviour). Mutation
      evidence: sweep disabled → exit 101 with missing pair-budget
      announcement; reverted → green. Files:
      tests/steps/release_strict_overrides_e2e_steps.rs (budget request,
      announcement binding, env propagation), feature file.
- [x] S2 `An installed givn keeps the review gate closed when model evidence
      is unavailable` (@e2e @release-proof): minimal change fixture (sign-off
      markers, commit evidence, no-op commands, addons disabled) driven
      through `check review` on the release binary with an isolated cache;
      asserts the real-model refusal and non-zero exit.
      GREEN-first (pins existing refusal behaviour; the then-binding is
      shared with the permanent release-strict scenarios). Files:
      tests/steps/release_strict_overrides_e2e_steps.rs, feature file.

## S3: completeness net

- [x] S3 Source-derived drift net: test module walks every `src/*.rs`, scans
      `GIVN_SPEC_*` literals, asserts registry ∪ exemptions in both
      directions (read without policy entry fails; registered channel no
      longer read fails). EXEMPT_CHANNELS: thread budget, honoured
      model-location channels, test-only probe.
      Mutation evidence: unregistered read probe → test FAILED; reverted →
      green. GREEN-first in the compliant tree. Files: src/spec/retrieval.rs
      (release_strict_channel_completeness). Wired into run-tests.sh default
      and `--non-e2e` branches and the existing `@release-proof` branch
      (`cargo test --release --lib -- release_strict`).

## S4: documentation

- [x] S4 arc42 ch. 08 § 8.5a: refusal policy made mechanically complete
      (registry + exemptions, request-class table with split fail-safe rows,
      accepted exception including the download mirror); README enforcement
      note (narrowed claim); design/trust comments at the model-dir sites.

## S5: verification

- [ ] Full suite + e2e green; clippy/fmt clean; real-model review; archive.

## Commit evidence

- [x] COMMIT: (recorded after the scenario commits) -- S1+S2+S3+S4
