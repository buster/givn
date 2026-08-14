# Tasks: spec-overlap-gates

RED→GREEN→REFACTOR→COMMIT per scenario. Non-@e2e first, then @e2e.

Legend: pending (unchecked) versus done (checked with evidence + commit
hash). Check boxes immediately after each scenario — tasks.md must always
reflect true state.

---

## Strict-mode proof (do first)

- [x] **Proof-of-strictness**: `.fail_on_skipped()` ist bereits in
      `tests/features_runner.rs` aktiv (bestehender Beweis: 154 permanente
      Szenarien laufen mit hartem Fail auf undefined Steps). Kein neuer
      Runner nötig; Stub-Muster `unimplemented!()`.
      ```
      Bestehende Suite beweist Strict Mode (fail_on_skipped seit
      reverse-engineer-lint-command aktiv, kein Exit-0-Durchrutschen).
      ```

---

## Setup

- [x] Step-Dateien anlegen und registrieren: `tests/steps/spec_overlap_gates_steps.rs`
      (exit 3, JSON-Steps) und `tests/steps/review_disposition_gates_steps.rs`
      (Dispositions-Steps), beide als `pub mod` in `tests/steps/mod.rs`.
      ```
      cargo check → Finished dev profile; Step-Dateien erstellt und
      registriert (tests/steps/mod.rs um zwei `pub mod`-Zeilen erweitert).
      ```

---

## Tasks: capability spec-overlap-gates (non-@e2e)

- [ ] Scenario: `A duplicate title across two permanent spec files is a hard error` (später als @e2e — siehe unten; hier regulär nicht geführt)
  - [ ] RED: `unimplemented!()` in neuen Steps; `src/overlap/`-Modul-Skelett.
        ```
        <output>
        ```
  - [ ] GREEN: Produktion: `src/overlap/` (Skeleton) + Lint-Integration, Exit 3.
        ```
        <output>
        ```
  - [ ] REFACTOR: Aufräumen.
        ```
        <output>
        ```
  - [ ] COMMIT: `<hash>`

- [x] Scenario: `An added delta duplicating a permanent spec title is a hard error`
  - [x] RED: `unimplemented!()` nicht nötig — vorhandene Steps genutzt, neuer
        Step `the command exits 3` mit echter Assertion; Produktions-Code fehlt,
        Lint liefert exit 0.
        ```
        GIVN_FEATURES=givn/changes/spec-overlap-gates/specs cargo test --test
        features_runner -- --name "An added delta duplicating a permanent
        spec title is a hard error"
        → 1 scenario (1 failed), 5 steps (4 passed, 1 failed) — "the command
        exits 3" schlägt fehl (tatsächlich 0).
        ```
  - [x] GREEN: Produktion: `src/overlap/{mod,analysis,fingerprint,disposition}.rs`
        (Baum-Kollektion, Titel-Gruppierung, Finding-Modell), `src/cli/lint.rs`
        (Scan + Ausgabe + Exit 3), `src/lib.rs`+`src/main.rs` (mod overlap).
        ```
        Gleiche Zielinvokation → 1 scenario (1 passed), 6 steps (6 passed).
        ```
  - [x] REFACTOR: Capability-Erkennung vereinfacht, kein Verhaltenswechsel.
        ```
        Gleiche Zielinvokation → 1 scenario (1 passed).
        ```
  - [x] COMMIT: `1a6dc6d`

- [x] Scenario: `A delta of another active change duplicating a permanent spec title is a hard error`
  - [x] RED/GREEN: Sofort-GREEN per vollständigem Step-Reuse — der Baum-Scan
        aus dem vorherigen Szenario deckt `givn/changes/*/specs` bereits ab;
        kein neuer Step, keine Produktionsänderung nötig.
        ```
        GIVN_FEATURES=givn/changes/spec-overlap-gates/specs cargo test --test
        features_runner -- --name "A delta of another active change
        duplicating a permanent spec title is a hard error"
        → 1 scenario (1 passed), 7 steps (7 passed).
        ```
  - [x] REFACTOR: n/a
  - [x] COMMIT: `89f4b30`

- [x] Scenario: `Deltas of two active changes sharing a title are a hard error`
  - [x] RED/GREEN: Sofort-GREEN per Step-Reuse (Scan über alle aktiven
        Changes vorhanden).
        ```
        → 1 scenario (1 passed), 8 steps (8 passed).
        ```
  - [x] REFACTOR: n/a
  - [x] COMMIT: `58a7b39`

- [x] Scenario: `A modified delta matching its merge target in the same capability is allowed` — **Charakterisierung** (heute bereits exit 0; RED unmöglich; Regression-Guard für die modified-Ausnahme)
  - [x] GREEN: Modified-Ausnahme in `title_duplicates` (genau ein Paar
        modified-Delta vs. permanentes Ziel gleicher Capability) verifiziert.
        ```
        → 1 scenario (1 passed), 5 steps (5 passed).
        ```
  - [x] COMMIT: `a79ddc4`

- [x] Scenario: `A modified delta whose title exists in a different capability is a hard error`
  - [x] RED/GREEN: Sofort-GREEN per Step-Reuse (Capability-Vergleich der
        modified-Ausnahme bereits implementiert — ungleiche Capability →
        kein Exempt → Duplikat-Error).
        ```
        → 1 scenario (1 passed), 6 steps (6 passed).
        ```
  - [x] REFACTOR: n/a
  - [x] COMMIT: `4a3d50e`

- [ ] Scenario: `A removed delta referencing its target by title is allowed` — **Charakterisierung** (heute bereits exit 0)
  - [ ] GREEN: Produktion: removed-Paar-Ausnahme.
  - [ ] COMMIT: `<hash>`

- [ ] Scenario: `Two scenarios with the same shape but different titles produce a warning`
  - [ ] RED: `<output>`
  - [ ] GREEN: Produktion: Fingerprint (Assertion-Folge, Maskierung), Shape-Match-Warnung.
  - [ ] REFACTOR: `<output>`
  - [ ] COMMIT: `<hash>`

- [ ] Scenario: `The same shape with contradicting facts produces a related-scenario hint`
  - [ ] RED: `<output>`
  - [ ] GREEN: Produktion: Facts (exit_code), Widerspruch → related_scenario.
  - [ ] REFACTOR: `<output>`
  - [ ] COMMIT: `<hash>`

- [ ] Scenario: `A scenario whose assertion sequence is a subset of another produces a subset warning`
  - [ ] RED: `<output>`
  - [ ] GREEN: Produktion: Subset-Erkennung (LCS).
  - [ ] REFACTOR: `<output>`
  - [ ] COMMIT: `<hash>`

- [ ] Scenario: `A scenario with more than 14 steps produces a long-scenario warning`
  - [ ] RED: `<output>`
  - [ ] GREEN: Produktion: Längen-Check (>14, ohne Background).
  - [ ] REFACTOR: `<output>`
  - [ ] COMMIT: `<hash>`

- [ ] Scenario: `The default scope excludes the archive tree`
  - [ ] RED: `<output>`
  - [ ] GREEN: Produktion: givn/archive aus Default-Scope.
  - [ ] REFACTOR: `<output>`
  - [ ] COMMIT: `<hash>`

- [ ] Scenario: `An explicit archive path still lints its file` — **Charakterisierung** (heute bereits exit 2)
  - [ ] GREEN: Bestätigung, dass explizite Pfade unverändert linten.
  - [ ] COMMIT: `<hash>`

- [ ] Scenario: `The json output reports a parse error as a machine-readable finding`
  - [ ] RED: `<output>`
  - [ ] GREEN: Produktion: `--json`-Flag (main.rs), JSON-Schema für Parse-Findings.
  - [ ] REFACTOR: `<output>`
  - [ ] COMMIT: `<hash>`

- [ ] Scenario: `Consecutive And and But steps normalise to the same shape as plain Then steps`
  - [ ] RED: `<output>`
  - [ ] GREEN: Produktion: StepType-basierte Assertion-Folge (gherkin `Step.ty`).
  - [ ] REFACTOR: `<output>`
  - [ ] COMMIT: `<hash>`

- [ ] Scenario: `Quoted strings are masked in the shape comparison`
  - [ ] RED: `<output>`
  - [ ] GREEN: Produktion: Quoted-String-Maskierung.
  - [ ] REFACTOR: `<output>`
  - [ ] COMMIT: `<hash>`

- [ ] Scenario: `Numbers are masked in the shape comparison but exit codes stay facts`
  - [ ] RED: `<output>`
  - [ ] GREEN: Produktion: Zahlen-Maskierung (Exit-Code als Fact bleibt).
  - [ ] REFACTOR: `<output>`
  - [ ] COMMIT: `<hash>`

- [ ] Scenario: `Scenario outlines sharing a template produce a shape-match warning`
  - [ ] RED: `<output>`
  - [ ] GREEN: Produktion: Outline-Template-Fingerprint.
  - [ ] REFACTOR: `<output>`
  - [ ] COMMIT: `<hash>`

- [ ] Scenario: `Docstrings are ignored in the shape comparison`
  - [ ] RED: `<output>`
  - [ ] GREEN: Produktion: Docstring-Ignorierung.
  - [ ] REFACTOR: `<output>`
  - [ ] COMMIT: `<hash>`

- [ ] Scenario: `Background steps are prepended before the comparison`
  - [ ] RED: `<output>`
  - [ ] GREEN: Produktion: Background-Voranstellung.
  - [ ] REFACTOR: `<output>`
  - [ ] COMMIT: `<hash>`

- [ ] Scenario: `Data table contents are compared as facts and can contradict`
  - [ ] RED: `<output>`
  - [ ] GREEN: Produktion: Data-Table-Facts (Spaltenkopf-Key).
  - [ ] REFACTOR: `<output>`
  - [ ] COMMIT: `<hash>`

- [ ] Scenario: `The same shape without any extractable facts suggests merging`
  - [ ] RED: `<output>`
  - [ ] GREEN: Produktion: konservative Fact-Regel (fehlender Key = kein Widerspruch).
  - [ ] REFACTOR: `<output>`
  - [ ] COMMIT: `<hash>`

---

## Tasks: capability review-disposition-gates (non-@e2e)

- [ ] Scenario: `Review blocks when a shape match involving the change's delta is undisposed`
  - [ ] RED: `<output>`
  - [ ] GREEN: Produktion: Review-Gate in `run_mandatory_checks("review")`, disposition.rs (Tabellen-Parser).
  - [ ] REFACTOR: `<output>`
  - [ ] COMMIT: `<hash>`

- [ ] Scenario: `Review blocks when a long scenario in the delta has no split-or-keep entry`
  - [ ] RED: `<output>`
  - [ ] GREEN: Produktion: split-or-keep-Zeilen-Check.
  - [ ] REFACTOR: `<output>`
  - [ ] COMMIT: `<hash>`

- [ ] Scenario: `Review passes when a long scenario has a split-or-keep entry` — **Charakterisierung** (Gate-Akzeptanzpfad)
  - [ ] GREEN: Bestätigung mit split-or-keep-Zeile.
  - [ ] COMMIT: `<hash>`

- [ ] Scenario: `Review blocks when a delta scenario shape-matches a permanent scenario without disposition`
  - [ ] RED: `<output>`
  - [ ] GREEN: Produktion: Mitgliedschafts-Regel (≥1 Delta-Mitglied).
  - [ ] REFACTOR: `<output>`
  - [ ] COMMIT: `<hash>`

- [ ] Scenario: `Review passes when a shape match exists only between permanent scenarios`
  - [ ] RED: `<output>`
  - [ ] GREEN: Produktion: permanent-only-Exemption.
  - [ ] REFACTOR: `<output>`
  - [ ] COMMIT: `<hash>`

- [ ] Scenario: `Review blocks with a clear message when review.md is missing`
  - [ ] RED: `<output>`
  - [ ] GREEN: Produktion: review.md-Pflicht-Bail.
  - [ ] REFACTOR: `<output>`
  - [ ] COMMIT: `<hash>`

- [ ] Scenario: `The net-delta receipt counts added, modified, and removed scenarios`
  - [ ] RED: `<output>`
  - [ ] GREEN: Produktion: Net-Delta-Receipt (Delta-Tags-Zählung) auf stdout.
  - [ ] REFACTOR: `<output>`
  - [ ] COMMIT: `<hash>`

---

## Tasks: @e2e scenarios (verify.e2e_command)

- [ ] Scenario: `A duplicate title across two permanent spec files is a hard error` (@e2e)
  - [ ] RED: `./run-tests.sh --e2e` gegen Fixture → Fehlschlag.
        ```
        <output>
        ```
  - [ ] GREEN: Schritte gegen echtes Binary (CARGO_BIN_EXE_givn), Fixture-Baum.
        ```
        <output>
        ```
  - [ ] REFACTOR: `<output>`
  - [ ] COMMIT: `<hash>`

- [ ] Scenario: `The json output reports duplicate-title findings machine-readably` (@e2e)
  - [ ] RED: `<output>`
  - [ ] GREEN: `<output>`
  - [ ] REFACTOR: `<output>`
  - [ ] COMMIT: `<hash>`

- [ ] Scenario: `Review passes when every shape match is disposed and reports the net-delta receipt` (@e2e)
  - [ ] RED: `<output>`
  - [ ] GREEN: `<output>`
  - [ ] REFACTOR: `<output>`
  - [ ] COMMIT: `<hash>`

---

## Evidenzpflichten & Abschluss

- [ ] Fingerprint-Messung auf den 18 gelabelten watn-Paaren
      (`~/projects/watn/docs/feature-step-overlap-report.md`); Ergebnis →
      `givn/changes/spec-overlap-gates/evidence-fingerprint.md`. Messung
      mit Wegwerf-Skript `tools/overlap-measure/`, Overlap-Bibliothek.
      ```
      <output>
      ```
- [ ] Dispositions-Volumen messen: `givn check review --change spec-overlap-gates`
      ausführen, fehlende Zeilen auflisten, review.md-Dispositionstabelle
      daraus generieren (jedes shape-match-ohne-Widerspruch-Paar mit ≥1
      Delta-Mitglied; jedes lange Delta-Szenario).
- [ ] arc42: arc42.md mit 03/05/06/08/09/11/12 Affected: Yes + echte Edits
      der Kapitel (MADR in 09, Konsequenzen in 11, Glossar in 12).
- [ ] @wip-Tags aus allen 29 Szenarien entfernen (nach vollständigem GREEN).
- [ ] Gesamtsuite: `./run-tests.sh` und `./run-tests.sh --e2e` grün.
- [ ] `givn check review --change spec-overlap-gates` grün (Dispositionen vollständig).
- [ ] Review-Artefakt review.md schreiben (Dispositionstabelle, Evidence,
      Coverage-Review-Klassifikation).
- [ ] `givn archive spec-overlap-gates` (nach Review-Grün).
