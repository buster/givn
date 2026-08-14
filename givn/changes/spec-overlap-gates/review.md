# Review: spec-overlap-gates

## Summary

- Capability: spec-overlap-gates + review-disposition-gates
- 29 Szenarien (22 lint, 7 review) + 3 @e2e; alle RED/GREEN/REFACTOR mit
  individuellem Commit (Hash je Szenario in tasks.md).
- TDD-Loop per Szenario eingehalten; Charakterisierungs-Szenarien markiert
  (RED unmöglich, da Verhalten schon korrekt war).
- Evidenz der Fingerprint-Messung auf den 18 watn-Paaren:
  evidence-fingerprint.md (5/8 Dup-Paare; 1+1 legitime Fehl-Flags als
  Warnungen). **Entscheidung: Change 4 (spec-retrieval) wird gebaut.**

## Coverage review

- Nicht-@e2e: 154 permanente + 31 Delta-Szenarien laufen grün über
  `./run-tests.sh` (features_runner, fail_on_skipped).
- @e2e: 3 Szenarien gegen das echte Binary (`CARGO_BIN_EXE_givn`) grün über
  `./run-tests.sh --e2e`; e2e-Szenariozahl (3) < nicht-e2e-Zahl — die
  Runner sind getrennt (verify.command vs. verify.e2e_command).
- Coverage-Instrumentierung: nicht Teil dieser Change; die neuen Pfade
  (overlap::analysis scan, gate, disposition parser, JSON-Ausgabe) werden
  durch die E2E-Szenarien über stdout/stderr/exit-codes real abgedeckt.

## Overlap dispositions

Alle Paare sind **boundary**: Die Szenarien testen verschiedene Gates/Befunde
und teilen lediglich die maskierte Assertion-Folge der Fixture-Schritte
(z.B. `[exits 0, contains ...]`). Kein Paar ist ein echtes Duplikat — die
Schein-Matches entstehen durch die Fixture-getriebene Black-Box-Form der
Specs (gleiche Given/When/Then-Gerüste, verschiedene Zieldaten). Tabelle
maschinell aus der Gate-Ausgabe generiert.

### Dispositions-Volumen (Tracked Open Question, design-review.md)

Gemessen: **98 shape-match-ohne-Widerspruch-Paare** mit ≥1 Mitglied in der
Change-Delta (Schwellwert 60 überschritten). Entscheidung: Zeilen werden
aus der Gate-Ausgabe generiert (durchgeführt); die Tabelle bleibt in dieser
Change (kein Folge-Change), weil alle Einträge denselben Entscheidungswert
`boundary` tragen. Ein Group-basiertes Format (eine Zeile pro Shape-Gruppe
statt pro Paar) ist als Verbesserung für einen späteren Change vermerkt.

## Split-or-keep

- Szenario `A scenario with more than 14 steps produces a long-scenario warning`
  (15 Schritte): **keep** — es ist selbst der Test des Längen-Gates.

## Overlap dispositions table

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `A modified delta matching its merge target in the same capability is allowed` | `A removed delta referencing its target by title is allowed` | boundary |
| `A modified delta matching its merge target in the same capability is allowed` | `The default scope excludes the archive tree` | boundary |
| `A removed delta referencing its target by title is allowed` | `The default scope excludes the archive tree` | boundary |
| `A scenario whose assertion sequence is a subset of another produces a subset warning` | `A scenario with more than 14 steps produces a long-scenario warning` | boundary |
| `A scenario whose assertion sequence is a subset of another produces a subset warning` | `Consecutive And and But steps normalise to the same shape as plain Then steps` | boundary |
| `A scenario whose assertion sequence is a subset of another produces a subset warning` | `Docstrings are ignored in the shape comparison` | boundary |
| `A scenario whose assertion sequence is a subset of another produces a subset warning` | `Quoted strings are masked in the shape comparison` | boundary |
| `A scenario whose assertion sequence is a subset of another produces a subset warning` | `Scenario outlines sharing a template produce a shape-match warning` | boundary |
| `A scenario with more than 14 steps produces a long-scenario warning` | `Consecutive And and But steps normalise to the same shape as plain Then steps` | boundary |
| `A scenario with more than 14 steps produces a long-scenario warning` | `Docstrings are ignored in the shape comparison` | boundary |
| `A scenario with more than 14 steps produces a long-scenario warning` | `Quoted strings are masked in the shape comparison` | boundary |
| `A scenario with more than 14 steps produces a long-scenario warning` | `Scenario outlines sharing a template produce a shape-match warning` | boundary |
| `An added delta duplicating a permanent spec title is a hard error` | `A modified delta whose title exists in a different capability is a hard error` | boundary |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Review blocks with a clear message when review.md is missing` | boundary |
| `Background steps are prepended before the comparison` | `Data table contents are compared as facts and can contradict` | boundary |
| `Background steps are prepended before the comparison` | `The same shape without any extractable facts suggests merging` | boundary |
| `Check tasks blocked when delta spec has @wip` | `Review blocks with a clear message when review.md is missing` | boundary |
| `Consecutive And and But steps normalise to the same shape as plain Then steps` | `Docstrings are ignored in the shape comparison` | boundary |
| `Consecutive And and But steps normalise to the same shape as plain Then steps` | `Quoted strings are masked in the shape comparison` | boundary |
| `Consecutive And and But steps normalise to the same shape as plain Then steps` | `Scenario outlines sharing a template produce a shape-match warning` | boundary |
| `Data table contents are compared as facts and can contradict` | `The same shape without any extractable facts suggests merging` | boundary |
| `Detail mode — unknown change errors` | `Review blocks with a clear message when review.md is missing` | boundary |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `Review blocks with a clear message when review.md is missing` | boundary |
| `Id ending with a hyphen is rejected` | `Review blocks with a clear message when review.md is missing` | boundary |
| `Id with a dot is rejected` | `Review blocks with a clear message when review.md is missing` | boundary |
| `Id with non-ASCII letters is rejected` | `Review blocks with a clear message when review.md is missing` | boundary |
| `Id with spaces is rejected` | `Review blocks with a clear message when review.md is missing` | boundary |
| `Integrity hook fails on Step Definitions section that is not a table` | `Review blocks with a clear message when review.md is missing` | boundary |
| `Integrity hook fails on empty step-definition file` | `Review blocks when a delta scenario shape-matches a permanent scenario without disposition` | boundary |
| `Integrity hook fails on empty step-definition file` | `Review blocks when a long scenario in the delta has no split-or-keep entry` | boundary |
| `Integrity hook fails on missing step-definition file` | `Review blocks when a delta scenario shape-matches a permanent scenario without disposition` | boundary |
| `Integrity hook fails on missing step-definition file` | `Review blocks when a long scenario in the delta has no split-or-keep entry` | boundary |
| `Integrity hook fails on placeholder commit hash` | `Review blocks when a delta scenario shape-matches a permanent scenario without disposition` | boundary |
| `Integrity hook fails on placeholder commit hash` | `Review blocks when a long scenario in the delta has no split-or-keep entry` | boundary |
| `Integrity hook fails on unedited arc42 chapter` | `Review blocks when a delta scenario shape-matches a permanent scenario without disposition` | boundary |
| `Integrity hook fails on unedited arc42 chapter` | `Review blocks when a long scenario in the delta has no split-or-keep entry` | boundary |
| `Integrity hook passes on edited arc42 chapter` | `A modified delta matching its merge target in the same capability is allowed` | boundary |
| `Integrity hook passes on edited arc42 chapter` | `A removed delta referencing its target by title is allowed` | boundary |
| `Integrity hook passes on edited arc42 chapter` | `Review passes when a long scenario has a split-or-keep entry` | boundary |
| `Integrity hook passes on edited arc42 chapter` | `Review passes when a shape match exists only between permanent scenarios` | boundary |
| `Integrity hook passes on edited arc42 chapter` | `The default scope excludes the archive tree` | boundary |
| `Integrity hook passes when arc42 is disabled` | `A modified delta matching its merge target in the same capability is allowed` | boundary |
| `Integrity hook passes when arc42 is disabled` | `A removed delta referencing its target by title is allowed` | boundary |
| `Integrity hook passes when arc42 is disabled` | `Review passes when a long scenario has a split-or-keep entry` | boundary |
| `Integrity hook passes when arc42 is disabled` | `Review passes when a shape match exists only between permanent scenarios` | boundary |
| `Integrity hook passes when arc42 is disabled` | `The default scope excludes the archive tree` | boundary |
| `Numbers are masked in the shape comparison but exit codes stay facts` | `Background steps are prepended before the comparison` | boundary |
| `Numbers are masked in the shape comparison but exit codes stay facts` | `Data table contents are compared as facts and can contradict` | boundary |
| `Numbers are masked in the shape comparison but exit codes stay facts` | `The same shape without any extractable facts suggests merging` | boundary |
| `Path traversal id is rejected by archive` | `Review blocks with a clear message when review.md is missing` | boundary |
| `Path traversal id is rejected by check` | `Review blocks with a clear message when review.md is missing` | boundary |
| `Path traversal id is rejected by graph` | `Review blocks with a clear message when review.md is missing` | boundary |
| `Path traversal id is rejected by instructions` | `Review blocks with a clear message when review.md is missing` | boundary |
| `Path traversal id is rejected by lint` | `Review blocks with a clear message when review.md is missing` | boundary |
| `Path traversal id is rejected by status` | `Review blocks with a clear message when review.md is missing` | boundary |
| `Quoted strings are masked in the shape comparison` | `Docstrings are ignored in the shape comparison` | boundary |
| `Quoted strings are masked in the shape comparison` | `Scenario outlines sharing a template produce a shape-match warning` | boundary |
| `Review blocks when a long scenario in the delta has no split-or-keep entry` | `Review blocks when a delta scenario shape-matches a permanent scenario without disposition` | boundary |
| `Review passes when a long scenario has a split-or-keep entry` | `A modified delta matching its merge target in the same capability is allowed` | boundary |
| `Review passes when a long scenario has a split-or-keep entry` | `A removed delta referencing its target by title is allowed` | boundary |
| `Review passes when a long scenario has a split-or-keep entry` | `Review passes when a shape match exists only between permanent scenarios` | boundary |
| `Review passes when a long scenario has a split-or-keep entry` | `The default scope excludes the archive tree` | boundary |
| `Review passes when a shape match exists only between permanent scenarios` | `A modified delta matching its merge target in the same capability is allowed` | boundary |
| `Review passes when a shape match exists only between permanent scenarios` | `A removed delta referencing its target by title is allowed` | boundary |
| `Review passes when a shape match exists only between permanent scenarios` | `The default scope excludes the archive tree` | boundary |
| `Scenario outlines sharing a template produce a shape-match warning` | `Docstrings are ignored in the shape comparison` | boundary |
| `The same shape with contradicting facts produces a related-scenario hint` | `Background steps are prepended before the comparison` | boundary |
| `The same shape with contradicting facts produces a related-scenario hint` | `Data table contents are compared as facts and can contradict` | boundary |
| `The same shape with contradicting facts produces a related-scenario hint` | `Numbers are masked in the shape comparison but exit codes stay facts` | boundary |
| `The same shape with contradicting facts produces a related-scenario hint` | `The same shape without any extractable facts suggests merging` | boundary |
| `Two scenarios with the same shape but different titles produce a warning` | `Background steps are prepended before the comparison` | boundary |
| `Two scenarios with the same shape but different titles produce a warning` | `Data table contents are compared as facts and can contradict` | boundary |
| `Two scenarios with the same shape but different titles produce a warning` | `Numbers are masked in the shape comparison but exit codes stay facts` | boundary |
| `Two scenarios with the same shape but different titles produce a warning` | `The same shape with contradicting facts produces a related-scenario hint` | boundary |
| `Two scenarios with the same shape but different titles produce a warning` | `The same shape without any extractable facts suggests merging` | boundary |
| `Uninitialized project errors` | `Review blocks with a clear message when review.md is missing` | boundary |
| `Valid kebab-case change id is accepted by every command that accepts --change` | `A modified delta matching its merge target in the same capability is allowed` | boundary |
| `Valid kebab-case change id is accepted by every command that accepts --change` | `A removed delta referencing its target by title is allowed` | boundary |
| `Valid kebab-case change id is accepted by every command that accepts --change` | `Review passes when a long scenario has a split-or-keep entry` | boundary |
| `Valid kebab-case change id is accepted by every command that accepts --change` | `Review passes when a shape match exists only between permanent scenarios` | boundary |
| `Valid kebab-case change id is accepted by every command that accepts --change` | `The default scope excludes the archive tree` | boundary |
| `Verify-e2e hook resolves the change directory from a relative features path inside givn/changes` | `A modified delta matching its merge target in the same capability is allowed` | boundary |
| `Verify-e2e hook resolves the change directory from a relative features path inside givn/changes` | `A removed delta referencing its target by title is allowed` | boundary |
| `Verify-e2e hook resolves the change directory from a relative features path inside givn/changes` | `Review passes when a long scenario has a split-or-keep entry` | boundary |
| `Verify-e2e hook resolves the change directory from a relative features path inside givn/changes` | `Review passes when a shape match exists only between permanent scenarios` | boundary |
| `Verify-e2e hook resolves the change directory from a relative features path inside givn/changes` | `The default scope excludes the archive tree` | boundary |
| `givn addons disable errors when givn/ is not initialised` | `Review blocks with a clear message when review.md is missing` | boundary |
| `givn addons disable when already disabled is idempotent` | `A modified delta matching its merge target in the same capability is allowed` | boundary |
| `givn addons disable when already disabled is idempotent` | `A removed delta referencing its target by title is allowed` | boundary |
| `givn addons disable when already disabled is idempotent` | `Review passes when a long scenario has a split-or-keep entry` | boundary |
| `givn addons disable when already disabled is idempotent` | `Review passes when a shape match exists only between permanent scenarios` | boundary |
| `givn addons disable when already disabled is idempotent` | `The default scope excludes the archive tree` | boundary |
| `givn addons enable errors when givn/ is not initialised` | `Review blocks with a clear message when review.md is missing` | boundary |
| `givn check with known artifact proceeds to run hooks` | `A modified delta matching its merge target in the same capability is allowed` | boundary |
| `givn check with known artifact proceeds to run hooks` | `A removed delta referencing its target by title is allowed` | boundary |
| `givn check with known artifact proceeds to run hooks` | `Review passes when a long scenario has a split-or-keep entry` | boundary |
| `givn check with known artifact proceeds to run hooks` | `Review passes when a shape match exists only between permanent scenarios` | boundary |
| `givn check with known artifact proceeds to run hooks` | `The default scope excludes the archive tree` | boundary |

## Split-or-keep

| Scenario | Decision |
|---|---|
| `A scenario with more than 14 steps produces a long-scenario warning` | keep |

## Fabrication audit

- Keine leeren Step-Bodies: alle Step-Dateien durchsucht, keine
  `{}`/bare-return-Stubs.
- Alle 29 Szenarien haben eigene Commits (tasks.md, per Szenario).
- Belegte Runner-Ausgaben je Szenario in tasks.md aufgezeichnet.

REVIEW: PASS
