# Design: spec-progress-output

## Problem

`givn spec index` (und `spec review`, `spec duplicates`, `spec search` bei
kaltem Cache) geben waehrend Modell-Download, Modell-Vorbereitung und
Szenario-Embedding keinerlei Ausgabe aus. Erst am Ende erscheint die
Zusammenfassung. Bei mehreren Minuten Download sieht der Befehl haengend aus.

Entscheidungen laut Nutzer:
- **Statuszeilen pro Phase UND Datei-Fortschritt waehrend Downloads.**
- Statusmeldungen werden **immer** ausgegeben (keine Terminal-Erkennung,
  keine Unterdrueckung in Pipes/CI).
- Die ehemals geplante Modell-Lade-Meldung wird als **generische
  Embedder-Vorbereitungsphase** umgesetzt (`preparing retrieval embedder`),
  die vor dem Embedding in jedem Backend erscheint. Ein eigener
  Real-Embedder-E2E mit ONNX-Fixture ist explizit out of scope.

## Technology decisions

- Sprache: Rust (bestehendes Projekt, edition 2024).
- Keine neuen Abhaengigkeiten. Der Datei-Fortschritt nutzt den bestehenden
  `hf_hub::api::sync::Progress`-Trait (hf-hub =0.5.0, Cargo.toml-gepinnt):
  `repo.download_with_progress(file, progress)` statt `repo.download(file)`.
  Ein eigener `StderrDownloadProgress` implementiert `init`/`update`/`finish`
  und schreibt Zeilen auf stderr. Version: hf-hub 0.5.0 — verifiziert ueber
  Cargo.lock (Projekt-Lockfile, Zeilen 921-924) und den Trait-Quelltext in
  `~/.cargo/registry/.../hf-hub-0.5.0/src/api/mod.rs:18`.
- Ausgabekanal: **stderr** fuer alle Fortschritts- und Statusmeldungen.
  stdout bleibt Ergebnis-Kanal (Duplicate-Audit-JSON, Index-Zusammenfassung,
  Suchrangkings, Review-Evidenz) und wird nicht angefasst.

## Architecture impact

Betroffen: `src/spec/retrieval.rs` (feature `spec-retrieval`). Keine neuen
Module; eine kleine private Helper-Struktur `StderrDownloadProgress` und eine
Helper-Funktion `notice(msg)` fuer einheitliche Statuszeilen.

### Statusmeldungs-Vokabular (stderr)

| Phase | Meldung (Format) |
|---|---|
| Download-Beginn E5 | `resolving retrieval model artifacts (E5) into <cache-dir>` |
| Datei-Fortschritt | `downloading <datei> <n>/<total> bytes` (aktualisiert, gedrosselt) |
| Download-Ende E5 | `retrieval model artifacts (E5) resolved` |
| Download-Beginn BGE | `resolving reranker model artifacts (BGE) into <cache-dir>` |
| Download-Ende BGE | `reranker model artifacts (BGE) resolved` |
| Download-Beginn NLI | `resolving NLI model artifacts into <cache-dir>` |
| Download-Ende NLI | `NLI model artifacts resolved` |
| Embedder-Vorbereitung | `preparing retrieval embedder` (vor dem Embedding, alle Backends) |
| Embedding-Beginn | `embedding <n> pending scenarios` |
| Szenario-Fortschritt | `embedded <i>/<n> scenarios` |
| Embedding-Ende | `embedding complete` |

Regeln:

- Eine Modell-Akquisitionsmeldung erscheint nur, wenn die Artefakte nicht
  bereits vollstaendig und valide im Cache liegen. Warme Caches erzeugen
  keine Download-Meldung und keinen Datei-Fortschritt.
- Der Download-Beginn wird VOR dem ersten Datei-Download ausgegeben — auch
  wenn der Download anschliessend fehlschlaegt.
- Die E5-Akquisitionsmeldungen erscheinen nur im echten
  Embedder-Pfad (`GIVN_SPEC_EMBEDDER` ungesetzt oder `download-stub`), weil
  `stub`/`missing` keine Artefakte beschaffen.
- **Repair-Doppellauf**: `auto_*_artifacts` ruft `fetch_hf_artifacts` bei
  fehlgeschlagener Validierung zweimal auf. Die Akquisitionsmeldungen werden
  in `auto_*_artifacts` um beide Fetch-Versuche gelegt (EIN Download-Beginn,
  EIN Download-Ende pro Akquisitionsphase); der Datei-Fortschritt erscheint
  dagegen pro tatsaechlichem Datei-Download und darf daher zweimal erscheinen.
- `StderrDownloadProgress` ist retry-tolerant: `hf-hub` ruft bei Retries
  `init()` erneut auf und `update()` erhaelt den Resume-Offset (kumulativ,
  nicht inkrementell). Der Zaehler wird bei `init` zurueckgesetzt; `update`
  behandelt jeden Wert absolut. Update-Ausgaben sind auf >= 1 % Fortschritt
  gedrosselt plus immer die Abschlusszeile; bei Mini-Fixtures reicht eine
  Abschlusszeile pro Datei (die E2E-Assertion akzeptiert das).

### Umbauten im Code

1. `fetch_hf_artifacts`: fuer jeden Datei-Download `download_with_progress`
   mit `StderrDownloadProgress::new(&file)` verwenden. `hf_api` behaelt
   `.with_progress(false)` (kein indicatif-Balken; wir liefern eigene
   Zeilen). Temp-Datei-, Lock-, Rename- und Retry-Semantik bleiben durch
   hf-hub unveraendert.
2. `auto_e5_artifacts`: `notice` bei Beginn (vor dem ersten
   `fetch_hf_artifacts`) und bei erfolgreichem Abschluss. `auto_bge_artifacts`,
   `auto_nli_artifacts` analog.
3. `embed_texts_with_policy`: `preparing retrieval embedder` vor der
   Modell-/Stub-Initialisierung, in allen Backends.
4. `refresh_index`: vor dem Batch-Embedding `embedding <n> pending
   scenarios`; nach Abschluss `embedding complete`. `embed_texts_with_policy`
   bekommt einen optionalen Fortschritts-Callback (`FnMut(usize, usize)`),
   der pro verarbeitetem Szenario `embedded <i>/<n> scenarios` meldet;
   Default: kein Callback (bestehende Aufrufer unveraendert).
5. `index_enabled` bleibt unveraendert (stdout-Zusammenfassung).

## Step definition locations

- `tests/steps/spec_progress_output_steps.rs` — regulare Steps (warm cache,
  failed download, disabled evidence, embedding count). Ein Datei pro
  Capability: `spec-progress-output`.
- `tests/steps/spec_progress_output_e2e_steps.rs` — @e2e Steps (cold-cache
  index, duplicates JSON purity, advisory review mit zweiter Stufe, search
  mit kaltem E5-Cache).
- Registrierung in `tests/steps/mod.rs` (zwei `pub mod`-Zeilen).
- Neues World-Feld `hf_endpoint_override: Option<String>` fuer Szenarien mit
  absichtlich unerreichbarer Modellquelle (toter Endpoint), getrennt vom
  laufenden `artifact_server`.

Vorhandene Steps werden wiederverwendet (`stdout reports a completed
permanent semantic index`, `stdout is valid duplicate-audit JSON`, `the
command exits 0`). Neue Steps:

- Given `an initialized project with unreachable semantic model sources`:
  init root + permanentes Szenario, `GIVN_SPEC_EMBEDDER=download-stub`,
  `HF_ENDPOINT=http://127.0.0.1:1/` (toter Endpoint), kaltes XDG-Cache-Home.
- Then `stderr identifies the failed E5 model artifact`: prueft, dass stderr
  `model.onnx` oder `tokenizer.json` und den Download-Kontext nennt.
  (Der bestehende Then-Step `stderr identifies the E5 model cache and failed
  artifact` passt nicht — er setzt den `missing`-Embedder-Text `E5 cache=`
  voraus; Fehlertexte bleiben unveraendert.)
- Given `an initialized project with a completed E5 model cache in the XDG
  Givn cache and three permanent semantic scenarios`: wie der bestehende
  Ein-Szenario-Fixture, aber drei permanente Szenarien.
- Given `an initialized project with a completed permanent semantic index
  and a cold E5 model cache`: Index mit `stub`-Embedder bauen, dann E5-Cache
  entfernen; startet den ArtifactFixtureServer.
- Then `stdout reports ranked search results`: stdout enthaelt `rank 1:`.
- Then `stdout reports advisory retrieval evidence`: stdout enthaelt
  `BGE status:` (aus `run_second_stage_advisory_review`).
- When `the end-to-end user searches for a permanent semantic scenario with
  model acquisition through the givn CLI`: `GIVN_SPEC_EMBEDDER=download-stub`,
  `XDG_CACHE_HOME`, `HF_ENDPOINT` -> Fixture-Server.
- When `the end-to-end user runs advisory review for the active change
  through the givn CLI`: Index vorhanden, `GIVN_SPEC_EMBEDDER=stub`,
  `GIVN_SPEC_BGE=auto`, `GIVN_SPEC_RERANKER=nli-auto`, Local-Stubs,
  `HF_ENDPOINT` -> Fixture-Server.
- Then-Steps fuer die neuen stderr-Assertions (Start-/Ende-/Fortschritts-/
  Count-Meldungen) als contains-Pruefungen mit den Vokabeln aus der Tabelle.

## Test runner

- `verify.command`: `./run-tests.sh` (givn/commands.yaml).
- `verify.e2e_command`: `./run-tests.sh --e2e`.
- Delta-Spec wird waehrend der Implementierung ueber
  `GIVN_FEATURES=givn/changes/spec-progress-output/specs` ausgefuehrt
  (Mechanismus wie `resolve_nli_features_path` in run-tests.sh; der
  features_runner liest `GIVN_FEATURES`, Default `givn/specs/`).
- Strict mode: cucumber-rs `.fail_on_skipped()` (bereits in
  `tests/features_runner.rs:20`). Nicht implementierte Steps enden mit
  `unimplemented!()`.
- Single-scenario run:
  `GIVN_FEATURES=givn/changes/spec-progress-output/specs cargo test --test features_runner -- --tags @e2e --name "<Szenario-Titel>"`

## E2E infrastructure

- Runner: `./run-tests.sh --e2e` mit `GIVN_FEATURES`-Override auf das
  Delta-Verzeichnis.
- Digital twin: `common::ArtifactFixtureServer` (bestehend) stellt die
  HF-Endpoint-Antworten lokal bereit; `HF_ENDPOINT` zeigt darauf. Keine
  Abhaengigkeit von live huggingface.co. Ort: `tests/common` (Server) und
  die beiden neuen Step-Dateien.
- Tag-Zuordnung: die beiden Second-Stage-E2E-Szenarien (Duplicates, Review)
  tragen `@nli-combined @e2e` und laufen in `run_nli_combined`
  (spec-retrieval,cross-encoder-reranker,nli-evidence) — nur dort sind
  `bge_backend`/NLI kompiliert. Die E5-E2E-Szenarien (Index, Search) tragen
  nur `@e2e` und laufen in `run_e5`.

### Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| build the permanent semantic index through the givn CLI | Cold-cache indexing announces each phase on stderr | CLI subprocess | `run_givn_with_envs` mit `GIVN_SPEC_EMBEDDER=download-stub`, `XDG_CACHE_HOME`, `HF_ENDPOINT` -> ArtifactFixtureServer |
| audit permanent scenarios with JSON output through the givn CLI | Duplicate audit keeps stdout pure JSON while progress runs on stderr | CLI subprocess | `run_givn_with_envs` mit `GIVN_SPEC_BGE=auto`, `GIVN_SPEC_RERANKER=nli-auto`, Local-Stubs, Score-Fixtures, `HF_ENDPOINT` -> ArtifactFixtureServer |
| run the advisory semantic review of an active change through the givn CLI | Advisory review announces second-stage model acquisition during the run | CLI subprocess | `run_givn_with_envs` mit Index-Voraufbau (stub), `GIVN_SPEC_BGE=auto`, `GIVN_SPEC_RERANKER=nli-auto`, Local-Stubs, `HF_ENDPOINT` -> ArtifactFixtureServer |
| search the permanent semantic index through the givn CLI | Search with a cold E5 cache announces model acquisition before results | CLI subprocess | `run_givn_with_envs` mit Index-Voraufbau (stub), `GIVN_SPEC_EMBEDDER=download-stub`, `XDG_CACHE_HOME`, `HF_ENDPOINT` -> ArtifactFixtureServer |

## Local runnability

- Lokaler Lauf des gesamten Systems: `cargo build --release && ./run-tests.sh`
  (CLI; keine Datenbank/Container noetig).
- Digitale Zwillinge: `ArtifactFixtureServer` (HF) ist der einzige externe
  Dienst; er laeuft im Testprozess. Keine weiteren externen Dienste.
- Anticipated interface obstacle: `repo.download` delegiert intern an
  `download_with_progress(file, ())`; der Wechsel auf einen eigenen
  `Progress`-Typ aendert nur die Meldungs-Implementierung (API-Vertrag laut
  hf-hub 0.5.0 `api/sync.rs:795,846-851`); die bestehenden
  E2E-Download-Szenarien (semantic model download) decken die Pfade weiterhin
  ab. Retry-Resume wird durch den absolut-zaehlenden `StderrDownloadProgress`
  toleriert.

## Datenmodell

Keine Aenderungen an Index-Format, Manifest oder Serialisierung.

## Out of scope

- Kein indicatif-Fortschrittsbalken; keine Terminal-Erkennung.
- Keine Unterdrueckung der Statusmeldungen in Pipes/CI.
- stdout-Formate bleiben byte-identisch.
- Kein Real-Embedder-E2E (kein ONNX-Fixture, kein ORT-Runtime-Binary im
  Repo).
