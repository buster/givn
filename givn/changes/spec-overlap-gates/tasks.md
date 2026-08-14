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

- [x] Scenario: `An added delta duplicating a permanent spec title is a hard error`
  - [x] RED: `the command exits 3` (neuer Step, echte Assertion); Lint
        liefert exit 0 ohne Dup-Check.
        ```
        → 1 scenario (1 failed), 5 steps (4 passed, 1 failed).
        ```
  - [x] GREEN: Produktion: `src/overlap/{mod,analysis,fingerprint,disposition}.rs`
        (Baum-Kollektion, Titel-Gruppierung, Finding-Modell), `src/cli/lint.rs`
        (Scan + Ausgabe + Exit 3), `src/lib.rs`+`src/main.rs` (mod overlap).
        ```
        → 1 scenario (1 passed), 6 steps (6 passed).
        ```
  - [x] REFACTOR: Capability-Erkennung vereinfacht, kein Verhaltenswechsel.
        ```
        Permanente Suite: 154 passed.
        ```
  - [x] COMMIT: `1a6dc6d`

- [x] Scenario: `A delta of another active change duplicating a permanent spec title is a hard error`
  - [x] RED/GREEN: Sofort-GREEN per Step-Reuse — Baum-Scan deckt
        `givn/changes/*/specs` bereits ab; kein neuer Step, keine
        Produktionsänderung nötig.
        ```
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

- [x] Scenario: `A removed delta referencing its target by title is allowed` — **Charakterisierung** (heute bereits exit 0)
  - [x] GREEN: removed-Ausnahme (removed-Szenarien aus der Titel-Gruppierung
        ausgeschlossen) verifiziert.
        ```
        → 1 scenario (1 passed), 5 steps (5 passed).
        ```
  - [x] COMMIT: `40d1fdc`

- [x] Scenario: `Two scenarios with the same shape but different titles produce a warning`
  - [x] RED: Maskierung fehlt → Shapes ("clean" vs "tidy") ungleich → keine
        Warnung, exit 0 statt Warnung auf stderr.
        ```
        → 1 scenario (1 failed) — "stderr contains \"merge\"" schlägt fehl.
        ```
  - [x] GREEN: Produktion: `mask()` in `src/overlap/fingerprint.rs`
        (Quoted Strings + Zahlen), Assertion-Folge maskiert; Spec-Assertion
        auf "merging" präzisiert (Substring-Missverständnis behoben).
        ```
        → 1 scenario (1 passed), 8 steps (8 passed).
        ```
  - [x] REFACTOR: n/a
  - [x] COMMIT: `a7bd2b2`

- [x] Scenario: `The same shape with contradicting facts produces a related-scenario hint`
  - [x] RED/GREEN: Step-Reuse — Facts/Exit-Code-Widerspruch aus dem
        initialen Modul-Slice vorhanden; Szenario verifiziert den
        related_scenario-Pfad.
        ```
        → 1 scenario (1 passed), 8 steps (8 passed).
        ```
  - [x] REFACTOR: n/a
  - [x] COMMIT: `a6579f9`

- [x] Scenario: `A scenario whose assertion sequence is a subset of another produces a subset warning`
  - [x] RED/GREEN: Step-Reuse — Subset-Erkennung (Subsequenz) aus dem
        initialen Modul-Slice vorhanden.
        ```
        → 1 scenario (1 passed), 7 steps (7 passed).
        ```
  - [x] REFACTOR: n/a
  - [x] COMMIT: `05396f5`

- [x] Scenario: `A scenario with more than 14 steps produces a long-scenario warning`
  - [x] RED/GREEN: Step-Reuse — Längen-Check (>14) aus dem initialen
        Modul-Slice vorhanden.
        ```
        → 1 scenario (1 passed), 6 steps (6 passed).
        ```
  - [x] REFACTOR: n/a
  - [x] COMMIT: `657794f`

- [x] Scenario: `The default scope excludes the archive tree`
  - [x] RED: Archive noch im Scope → Parse-Error (exit 1) statt erwartet 0.
        ```
        → 1 scenario (1 failed), 5 steps (4 passed, 1 failed).
        ```
  - [x] GREEN: Produktion: Default-Scope filtert givn/archive
        (src/cli/lint.rs). Permanente Suite bleibt grün (154 passed).
        ```
        → 1 scenario (1 passed), 5 steps (5 passed).
        ```
  - [x] REFACTOR: n/a
  - [x] COMMIT: `38765f8`

- [x] Scenario: `An explicit archive path still lints its file` — **Charakterisierung** (heute bereits exit 2)
  - [x] GREEN: Explizite Pfade unverändert — Archiv-Datei lintet gezielt
        weiter (exit 2, @wip gemeldet).
        ```
        → 1 scenario (1 passed), 5 steps (5 passed).
        ```
  - [x] COMMIT: `1616124`

- [x] Scenario: `The json output reports a parse error as a machine-readable finding`
  - [x] RED: `--json` existiert nicht → clap-Fehler statt JSON.
        ```
        → 1 scenario (1 failed), 4 steps (3 passed, 1 failed).
        ```
  - [x] GREEN: Produktion: `--json`-Flag (src/main.rs), Finding-Sammlung und
        pretty-JSON-Ausgabe in src/cli/lint.rs (Schema kind/level/files/titles;
        Parse-/Wip-Findings als JSON-Einträge). Spec-Assertionen auf
        backslash-freie Substrings umgestellt (Step-Regex-Escape-Falle).
        ```
        → 1 scenario (1 passed), 7 steps (7 passed).
        ```
  - [x] REFACTOR: Prose-Rendering in `render_prose` extrahiert.
        ```
        Permanente Suite: 154 passed.
        ```
  - [x] COMMIT: `9244be1`

- [x] Scenario: `Consecutive And and But steps normalise to the same shape as plain Then steps`
  - [x] RED/GREEN: Step-Reuse — gherkin-Crate löst And/But bereits
        kontextuell zu `Step.ty == Then` auf (Grilling-Finding 11 bestätigt).
        ```
        → 1 scenario (1 passed), 7 steps (7 passed).
        ```
  - [x] COMMIT: `1d1436a`

- [x] Scenario: `Quoted strings are masked in the shape comparison`
  - [x] RED/GREEN: Step-Reuse — `mask()` aus dem Shape-Match-Szenario deckt
        Quoted Strings ab.
        ```
        → 1 scenario (1 passed), 7 steps (7 passed).
        ```
  - [x] COMMIT: `e2a10f8`

- [x] Scenario: `Numbers are masked in the shape comparison but exit codes stay facts`
  - [x] RED/GREEN: Step-Reuse — Zahlen-Maskierung aus `mask()`; Exit-Codes
        als Facts aus `extract_exit_code` → Widerspruch → related_scenario.
        ```
        → 1 scenario (1 passed), 8 steps (8 passed).
        ```
  - [x] COMMIT: `f8110a6`

- [x] Scenario: `Scenario outlines sharing a template produce a shape-match warning`
  - [x] RED/GREEN: Step-Reuse — Template-Steps fließen als Fingerprint ein
        (Examples-Werte sind nicht Teil der Shape).
        ```
        → 1 scenario (1 passed), 6 steps (6 passed).
        ```
  - [x] COMMIT: `135060f`

- [x] Scenario: `Docstrings are ignored in the shape comparison`
  - [x] RED: Fixture-Gherkin parste nicht (verschachtelte `"""`-Delimiter —
        Spec-Fehler, keine Produktionslücke).
        ```
        → 1 parsing error (features_runner).
        ```
  - [x] GREEN: Spec-Fix (äußere Delimiter `"""`, innere ```-Fences);
        Produktion ignoriert Docstrings bereits (nur step.value zählt).
        ```
        → 1 scenario (1 passed), 7 steps (7 passed).
        ```
  - [x] COMMIT: `62b23ed`

- [x] Scenario: `Background steps are prepended before the comparison`
  - [x] RED: Fixture ohne eigenes When umgebaut (erst dann ist der
        Prepending-Effekt beobachtbar); ohne Prepending: SHAPE-Match statt
        subset.
        ```
        → 1 scenario (1 failed) — stderr missing "subset" (SHAPE-Linie statt subset).
        ```
  - [x] GREEN: Produktion: Background-Steps in all_steps prepended
        (of_scenario); ohne When zählen auch Background-Thens als
        Assertionen.
        ```
        → 1 scenario (1 passed), 8 steps (8 passed).
        ```
  - [x] REFACTOR: n/a
  - [x] COMMIT: `a0ae7f0`

- [x] Scenario: `Data table contents are compared as facts and can contradict`
  - [x] RED: Tabellen-Facts fehlen → kein Widerspruch erkannt.
        ```
        → 1 scenario (1 failed), 6 steps (5 passed, 1 failed).
        ```
  - [x] GREEN: Produktion: `extract_table_facts` (Spaltenkopf-Key,
        maskierte Zellwerte) in Facts::extract.
        ```
        → 1 scenario (1 passed), 8 steps (8 passed).
        ```
  - [x] REFACTOR: n/a
  - [x] COMMIT: `7c7a914`

- [x] Scenario: `The same shape without any extractable facts suggests merging`
  - [x] RED/GREEN: Step-Reuse — keine Facts auf beiden Seiten → kein
        Widerspruch → Merge-Vorschlag (konservative Regel vorhanden).
        ```
        → 1 scenario (1 passed), 8 steps (8 passed).
        ```
  - [x] COMMIT: `e14b76e`

---

## Tasks: capability review-disposition-gates (non-@e2e)

- [x] Scenario: `Review blocks when a shape match involving the change's delta is undisposed`
  - [x] RED: Kein Review-Gate → exit 0 statt Block.
        ```
        → 1 scenario (1 failed), 5 steps (4 passed, 1 failed).
        ```
  - [x] GREEN: Produktion: `run_review_overlap_gate` in
        src/overlap/analysis.rs, Aufruf in src/cli/check.rs nach integrity;
        disposition.rs-Tabellen-Parser.
        ```
        → 1 scenario (1 passed), 8 steps (8 passed).
        ```
  - [x] REFACTOR: Bail nur wenn Findings vorliegen (fehlendes review.md ohne
        Findings passiert) — permanente Suite bleibt grün (154 passed).
  - [x] COMMIT: `fcc068f`

- [x] Scenario: `Review blocks when a long scenario in the delta has no split-or-keep entry`
  - [x] RED/GREEN: Step-Reuse — split-or-keep-Pflicht im Review-Gate
        (bereits implementiert).
        ```
        → 1 scenario (1 passed), 7 steps (7 passed).
        ```
  - [x] COMMIT: `39f982d`

- [x] Scenario: `Review passes when a long scenario has a split-or-keep entry` — **Charakterisierung** (Gate-Akzeptanzpfad)
  - [x] GREEN: Akzeptanzpfad mit split-or-keep-Zeile verifiziert.
        ```
        → 1 scenario (1 passed), 5 steps (5 passed).
        ```
  - [x] COMMIT: `ae978a2`

- [x] Scenario: `Review blocks when a delta scenario shape-matches a permanent scenario without disposition`
  - [x] RED/GREEN: Step-Reuse — Mitgliedschafts-Regel (file-Prefix unter
        givn/changes/<id>/specs) im Gate vorhanden.
        ```
        → 1 scenario (1 passed), 8 steps (8 passed).
        ```
  - [x] COMMIT: `742caa4`

- [x] Scenario: `Review passes when a shape match exists only between permanent scenarios`
  - [x] RED/GREEN: Step-Reuse — permanent-only-Paare werden vom Gate
        übersprungen (involves_delta == false).
        ```
        → 1 scenario (1 passed), 7 steps (7 passed).
        ```
  - [x] COMMIT: `b7115f6`

- [x] Scenario: `Review blocks with a clear message when review.md is missing`
  - [x] RED: Fixture ohne Findings erzeugte keinen Block (Gate lief durch).
        ```
        → Szenario um Fixture mit echtem Shape-Match-Paar erweitert.
        ```
  - [x] GREEN: Produktion: Bail nennt review.md, wenn undisponierte
        Findings vorliegen und review.md fehlt.
        ```
        → 1 scenario (1 passed), 5 steps (5 passed).
        ```
  - [x] COMMIT: `30aaca0`

- [x] Scenario: `The net-delta receipt counts added, modified, and removed scenarios`
  - [x] RED/GREEN: Step-Reuse — Receipt (Delta-Op-Zählung) im Gate
        vorhanden, inkl. JSON-Zeile.
        ```
        → 1 scenario (1 passed), 9 steps (9 passed).
        ```
  - [x] COMMIT: `d4479c9`

---

## Tasks: @e2e scenarios (verify.e2e_command)

- [x] Scenario: `A duplicate title across two permanent spec files is a hard error` (@e2e)
  - [x] RED/GREEN: Sofort-GREEN per Step-Reuse — Steps stammen aus den
        non-@e2e-Phasen, Treiber ist das echte Binary (CARGO_BIN_EXE_givn).
        ```
        ./run-tests.sh --e2e → 3 scenarios (3 passed), 25 steps (25 passed).
        ```
  - [x] REFACTOR: n/a
  - [x] COMMIT: `75b22e1`

- [x] Scenario: `The json output reports duplicate-title findings machine-readably` (@e2e)
  - [x] RED/GREEN: Sofort-GREEN per Step-Reuse (JSON-Ausgabe aus dem
        non-@e2e-Slice; valides JSON + kind/title gegen echtes Binary).
        ```
        ./run-tests.sh --e2e → 3 scenarios (3 passed).
        ```
  - [x] COMMIT: `997e386`

- [x] Scenario: `Review passes when every shape match is disposed and reports the net-delta receipt` (@e2e)
  - [x] RED/GREEN: Sofort-GREEN per Step-Reuse (Review-Gate + Receipt aus
        den non-@e2e-Slices; Dispositions-Zeile via Step geschrieben).
        ```
        ./run-tests.sh --e2e → 3 scenarios (3 passed), 25 steps (25 passed).
        ```
  - [x] COMMIT: `1b342fc`

---

## Evidenzpflichten & Abschluss

- [x] Fingerprint-Messung auf den 18 gelabelten watn-Paaren
      (`~/projects/watn/docs/feature-step-overlap-report.md`); Ergebnis →
      `givn/changes/spec-overlap-gates/evidence-fingerprint.md`. Messung
      mit `tools/overlap-measure/` (Overlap-Bibliothek als Path-Dependency).
      ```
      dup pairs found (title|shape|subset): 5/8
      legit pairs flagged as duplicate/shape (false positives): 1/10
      (plus 1 legit Subset-Warnung: DONE-vs-EOF) — Details in
      evidence-fingerprint.md. Entscheidung: Change 4 wird gebaut.
      ```
- [x] Dispositions-Volumen messen: gemessen = 98 Paare (Schwellwert 60
      überschritten → Entscheidung dokumentiert in review.md: maschinell
      generierte `boundary`-Zeilen, kein Folge-Change). Gate-Ausgabe als
      Quelle; review.md enthält Tabelle + split-or-keep.
- [x] arc42: arc42.md (12-Zeilen-Tabelle, 7 Kapitel Affected: Yes) + Edits
      in docs/arc42/ (03, 05, 06, 08, 09 ADR-037, 11 R-10, 12).
- [x] @wip-Tags aus allen 29 Szenarien entfernen (nach vollständigem GREEN;
      verbleibende @wip sind Fixture-Interna).
- [x] Gesamtsuite: non-e2e 135 passed, e2e 19 passed (permanente Specs);
      Delta-Szenarien in den zielgerichteten Läufen grün.
- [x] `givn check review --change spec-overlap-gates` grün: verify,
      verify-e2e, integrity, overlap dispositions, net delta 31 added.
- [x] review.md geschrieben: Coverage-Review, Dispositionstabelle (98 Zeilen),
      split-or-keep, Fabrication-Audit.
- [ ] `givn archive spec-overlap-gates` (nach Review-Grün).
