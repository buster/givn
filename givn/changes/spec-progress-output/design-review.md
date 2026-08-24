# Design Review: spec-progress-output

## Grilling findings and resolutions

F1 — Duplicates-purity E2E war nur mit `@e2e` getaggt und lief damit im
E5-only-Build, in dem die BGE/NLI-Pfade nicht kompiliert sind; die
Download-Meldungen haetten nie erscheinen koennen. **Geloest:** Szenario traegt
jetzt `@nli-combined @e2e` (nli-combined-Build enthaelt beide Second-Stage-
Layer), konsistent mit dem bestehenden Second-Stage-E2E in
semantic-model-download.

F2 — Der wiederverwendete Then-Step `stderr identifies the E5 model cache and
failed artifact` setzt den `missing`-Embedder-Text (`E5 cache=`) voraus; der
download-stub-Fehlerpfad erzeugt ihn nicht. **Geloest:** neuer Then-Step
`stderr identifies the failed E5 model artifact` (prueft `model.onnx`/
`tokenizer.json` im Download-Kontext). Bestehende Fehlertexte bleiben
unveraendert. Neues World-Feld `hf_endpoint_override` fuer den toten Endpoint.

F3 — Das Disabled-Evidence-Szenario war RED-vakuumig (alle Assertions halten
heute). **Geloest:** positiver Anker ergaenzt (`stdout is valid
duplicate-audit JSON`). Die Negativ-Assertions bleiben als
Regressionsschutz; Scenario 2 hat mit der Embedding-Count-Assertion einen
echten RED-Anteil.

F4 — Proposal und arc42 8.5a.1 versprachen eine Modell-Lade-Meldung, die
Design und Spec nicht enthielten; kein Szenario nutzt den echten Embedder.
**Geloest (Nutzerentscheidung):** Meldung als generische
Embedder-Vorbereitungsphase (`preparing retrieval embedder`) vor dem
Embedding in allen Backends; Proposal, Spec und arc42 8.5a.1 angeglichen.
Real-Embedder-E2E mit ONNX-Fixture explizit out of scope.

F5 — Proposal nannte `spec search`, aber kein Szenario deckte ihn ab.
**Geloest (Nutzerentscheidung):** neues @e2e-Szenario "Search with a cold E5
cache announces model acquisition before results" (Index vorhanden, E5-Cache
kalt, download-stub + Fixture-Server). Inventory und Coverage-Matrix auf vier
Zeilen erweitert.

F6 — `download_with_progress`-Wechsel technisch validiert (hf-hub 0.5.0
`api/sync.rs:795`; `download()` delegiert an `(())`). Retry-Resume
(re-init + Offset-Sprung in `update`) und Mini-Fixture-Duesselung im Design
festgeschrieben.

F7 — Repair-Doppellauf (`fetch_hf_artifacts` zweimal bei korruptem Cache)
wuerde zwei Download-Beginn-Meldungen erzeugen. **Geloest:** genau EIN
Beginn/EIN Ende pro Akquisitionsphase in `auto_*_artifacts`; Datei-Fortschritt
darf pro tatsaechlichem Download zweimal erscheinen.

F8 — stdout-Reinheit fuer Review war unbelegt. **Geloest:** Assertion
`stdout reports advisory retrieval evidence` (prueft `BGE status:`) im
Review-Szenario.

F9 — ADR-Qualifikation wurde behauptet statt demonstriert. **Geloest:** Die
Output-Kanal-Entscheidung ist eine dokumentierte Konvention in arc42 Kapitel
8 (8.5a.1) und ein Quality-Requirement (Q-63/Q-64), kein unabhaengiger
Architekturentscheid mit Alternativen, Architekturwirkung und
Langlebigkeitsfolgen; damit nicht ADR-qualifiziert (Kriterien der
ADR-Qualifikation: keine echte Alternativenabwaegung, keine neuen Bausteine,
keine Migrationsfolgen). Kanonisches Ziel: arc42 08 + 10, dort eingetragen.

F10 — Proposal-Open-Questions geschlossen und Resolutionen dokumentiert.

F11 — Coverage-Matrix 1:1 zum Inventory, alle vier Treibermechanismen
verifiziert (run_e5 fuer Index/Search, run_nli_combined fuer
Duplicates/Review).

F12 — Kein Test assertet exakten stderr; neue Ausgaben ausschliesslich via
`eprintln!`/`notice`, `StderrDownloadProgress` absolut-zaehlend und
retry-tolerant.

F13 — arc42-Edits verifiziert; der "model preparation"-Widerspruch in
8.5a.1 wurde durch die F4-Resolution beseitigt.

## Verification

- `givn lint --change spec-progress-output`: exit 0.
- `givn check proposal|specs|design|arc42-docs --change spec-progress-output`:
  alle exit 0.
- E2E-Tags: `@e2e`-Szenarien bleiben erhalten; `verify.e2e_command`
  (`./run-tests.sh --e2e`) ist konfiguriert.

DESIGN-REVIEW: PASS
