# Design Review: semantic-review-output

Grilling ran in a fresh subagent context against proposal, specs delta,
design, arc42 assessment, the normative specs instruction, and the
retrieval reporting code (`src/main.rs`, `src/cli/spec.rs`,
`src/spec/retrieval.rs`) plus existing review-output step bindings.

## Grilling findings and resolutions

F1 — Design table hid BGE/NLI token-cap guidance lines from default
output; cap evidence would degrade to an unexplained counter and break
the mandated shorten-or-split remediation loop. **Resolved (user
decision):** token-cap guidance lines are status-class content, always
visible in every mode; design table, spec scenario 1, chapter 06
section 6.8.4.1, and the marker notes updated.

F2 — `review_enabled` prints three index-refresh banner lines to stdout
unconditionally before any review output, so `--format json` stdout can
never parse as exactly one document. **Resolved (user decision):** in
JSON mode the banner content embeds in the document as an `index`
summary section; text mode keeps today's banner lines byte-identical.
Design JSON section, spec scenario 10, and chapter 06 updated.

F3 — Byte-identity and cross-run equality scenarios compared full
output across processes while stage status lines carry wall-clock
runtime values; not deterministically falsifiable. **Resolved:** design
gains a determinism rule — comparison bindings normalize runtime fields
before comparing; spec scenario 9 reworded to "every line except
elapsed-runtime values".

F4 — Migration inventory missed `tests/steps/nli_evidence_e2e_steps.rs:253`,
which asserts `NLI_FILTERED` present in default advisory-review stdout
(inverted by this change), and omitted that pins backing permanent-spec
scenarios require migrating permanent expectations. **Resolved:** added
to the characterization list with an explicit audit sweep for any step
asserting filtered/unscored presence in review stdout; permanent-scenario
migration is an explicit implementation task.

F5 — Chatter-collapse scenarios asserted on stdout while resolution
notices print to stderr, and stub backends emit no resolution notices,
so both scenarios passed vacuously and could never fail in RED.
**Resolved (user decision):** scenarios reworded to stderr with a new
deterministic fixture; a test-only resolution-notice stub seam is
specified under "Test-only seams".

F6 — Interaction Coverage Matrix e2e title cells named titles that do
not exist (three matched regular scenarios instead). **Resolved:**
matrix now names the four actual `Real CLI …` scenario titles;
inventory↔matrix↔@e2e mapping verified 1:1.

F7 — New debt row collided with the pre-existing D-9 id in chapter 11.
**Resolved:** renumbered to D-14; register verified collision-free.

F8 — The proposed review document invented a second JSON dialect for
run structs `spec duplicates --format json` already serializes under
ADR-054 versioning. **Resolved (user decision):** the review document
extends the duplicates contract — shared candidate/run grammar reused
verbatim, review-only sections (`change`, `index`, `verdict`) additive;
design JSON section rewritten, "finalised during implementation" hedge
removed.

## Branch verdicts

- Scope: proposal claims and delta scenarios correspond exactly; error
  paths (findings-write failure, overwrite) covered.
- Tech choices: Rust reporting path only, no new dependencies; JSON
  dialect question resolved above.
- Missing scenarios: none remaining after F1/F2/F5 adjustments.
- Testability: all scenarios RED-falsifiable via stub backends plus the
  resolution-notice seam; runtime normalization rule closes F3.
- E2E fidelity: four inventory entries ↔ four @e2e scenarios ↔ four
  matrix rows, real subprocess driving, primary assertions on CLI
  stdout/filesystem output.
- Risk: default-flip breakage of existing pins is inventoried (F4) and
  task-level work is explicit.
- ADR qualification: nothing qualifies. Alternatives exist but no
  candidate passes durable_consequence or lower_level_artifact: flag
  surface, confidence ordering, persist-on-failure-only, token-cap
  visibility, and JSON dialect are observable behaviours owned by the
  Gherkin delta and design.md; ADR-054 already owns evidence-output
  versioning (existing_adr_check routed there).
- arc42: independent 12-row walk matches the marker table (rows 3, 5,
  6, 10, 11, 12 Yes; rest No); chapter edits verified against design.md;
  chapter 08 No upheld (existing channel conventions cover it);
  chapter 11 reflects the debt consequence via D-14.

DESIGN-REVIEW: PASS
