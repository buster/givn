# Design: spec-overlap-gates

## Goals

Deterministische Überlappungs-Gates für Szenarien über den ganzen aktiven
Baum (permanente Specs + Deltas aller aktiven Changes), ohne
Modell-Abhängigkeit. Hard Error nur für identische Titel; alles Weichere
warnt und erzwingt bei echter Merge-Ambiguität eine Disposition im Review.

## Technology decisions

- **Sprache/Toolchain**: Rust, bestehende Toolchain (rustc 1.97.1, installiert
  und verifiziert per `rustc --version`). Keine neuen Dependencies.
- **Gherkin-Parsing**: bestehende `gherkin`-Crate (0.16, bereits in
  Cargo.toml) — liefert Steps, Docstrings, Data Tables, Outline-Beispiele,
  Background. Kein eigener Parser.
- **Kein Modell, keine Embeddings**: der deterministische Fingerprint ist
  vollständig regelbasiert (dieser Change). Messung gegen watn entscheidet
  über Change 4.
- **serde**: bereits vorhanden (bestehende JSON-Ausgaben im Projekt) für
  `--json`.

## Architecture impact

### Neue Module (`src/overlap/`)

```
src/overlap/
  mod.rs          — öffentliche API: analyse_tree(), Finding-Typen
  fingerprint.rs  — Szenario → Shape (Assertion-Folge) + Facts
  analysis.rs     — Paar-Vergleich: Titel-Duplikate, Shape-Match,
                    Subset, Länge; Baum-Scan (ohne givn/archive)
  disposition.rs  — review.md Parsen (Dispositionstabelle,
                    split-or-keep-Zeilen)
```

### Fingerprint-Semantik (aus analysis.md §2.4, vollständig)

**Basis**: `Step.ty` (gherkin-Crate, bereits aufgelöst — `And`/`But`
tragen ihren kontextuellen Typ; ein führendes `And` ohne Vorgänger ist
`Given`). Keine Keyword-String-Vergleiche.

1. **Assertion-Folge**: Schritte ab dem letzten `When`-typisierten Schritt
   (`ty == Then`-Positionen danach) bilden die Shape-Sequenz. Ein Szenario
   **ohne** einen einzigen Then-typisierten Schritt hat eine leere
   Assertion-Folge und nimmt an Shape-/Subset-Vergleichen **nicht** teil
   (nur Titel- und Längen-Checks gelten).
2. **Maskierung**: Quoted Strings (`"…"`, `'…'`) → `"*"`; Zahlen (auch in
   `exits <n>`) → `*`. Alles andere bleibt wörtlich (Keywords
   klein-normalisiert).
3. **Ignoriert**: Docstrings (Inhalt), Given/When-typisierte Schritte.
4. **Background**: Background-Schritte werden der Szenario-Schrittfolge
   vorangestellt; sie zählen nicht zur Längen-Statistik.
5. **Scenario Outlines**: pro Examples-Zeile ein flacher Fingerprint —
   Template-Schritte mit substituierten Beispielwerten, danach maskiert.
   Vergleich zweier Outlines läuft über ihre Template-Fingerprints.
6. **Data Tables**: Tabelleninhalt ist **Fact**, nicht Shape. Key =
   Spaltenkopf; ohne Kopfzeile Positions-Index (`col_0` …). Value =
   Zellwert, maskiert.
7. **Facts extrahieren** (pro Szenario, Best-Effort):
   - `exit_code`: aus `exits <n>` / `exit code <n>`.
   - `contains_polarity`: pro Assertion-Position `contains` vs
     `not-contains`/`does not contain`.
   - `table`: Data-Table-Map.
   - `done`: `[DONE]`-Marker (Szenario-Text).
   - `tty`: `TTY`-Erwähnung.
8. **Widerspruch**: Fact-Key kollidiert nur, wenn er auf **beiden** Seiten
   extrahiert wurde und die Werte verschieden sind. Fehlender Key auf einer
   Seite = kein Widerspruch (konservativ).
9. **Scope-Grenzen**: Szenarien in `Rule:`-Blöcken nehmen nicht teil
   (konsistent mit den bestehenden `feature.scenarios`-Walks von
   parse.rs). Eine Datei, die nicht parst, liefert keine Szenarien für den
   Scan — ihr Parse-Error dominiert (Exit 1). Ein `@givn.modified` ohne
   existierendes Merge-Target erzeugt kein Paar und keinen Befund (der
   Merge-Gate beim Archivieren fängt das wie bisher).

### Paar-Klassifikation (baumweit, über alle Dateien)

- `duplicate_title` (ERROR, Exit-Code 3): identischer Titel an zwei
  Stellen. Ausnahmen: Paare mit `@givn.removed`-Szenario; genau ein Paar
  "`@givn.modified`-Delta vs. Szenario gleichen Titels in der permanenten
  Spec **derselben Capability**". Capability = Name des Spec-Verzeichnisses
  (`givn/specs/<cap>/` bzw. `givn/changes/<id>/specs/<cap>/`).
- `shape_match` (WARN, kein Exit-Änderung): Assertion-Folgen gleich nach
  Normalisierung, verschiedene Titel.
  - **Ohne** Fact-Widerspruch: Hinweis "consider merging", Disposition
    **erforderlich**.
  - **Mit** Widerspruch: Hinweis `related_scenario` (nie
    Zusammenlegungs-Vorschlag), **keine** Disposition erforderlich.
- `subset` (WARN): Assertion-Folge B ist Teilfolge von A (LCS über die
  maskierten Folgen). Hinweis, keine Disposition erforderlich.
- `long_scenario` (WARN): > 14 Schritte (Given/When/Then/And/But gesamt,
  Background-Schritte zählen nicht). Disposition (split-or-keep)
  erforderlich.

### Lint-Integration (`src/cli/lint.rs`)

- Default-Scope: `collect_feature_files(givn/)` **abzüglich
  `givn/archive/`**. Explizite Pfade unverändert (kein Baum-Cross-Check bei
  expliziten Pfaden).
- Nach den bestehenden Parse-/Wip-Checks: Baum-Scan (nur wenn nicht
  `--change`/explizite Pfade) → Findings in bestehende
  Severity-Ausgabe integriert (`[DUPL ]`, `[SHAPE]`, `[SUBST]`,
  `[LONG ]`-Präfixe).
- **Exit-Codes**: 1 (Parse) > 3 (Duplicate) > 2 (Wip) > 0. Bestehende
  0/1/2-Semantik für heutige Fälle bleibt unverändert.
- `--json` (neues Flag `#[arg(long)] json` an `Commands::Lint` in
  src/main.rs): alle Findings — inkl. Parse- und Wip-Findings — als
  pretty-printed JSON-Array auf **stdout**, ein Objekt pro Finding:
  `{"kind": "<duplicate_title|shape_match|subset|long_scenario|parse_error|wip>",
   "level": "<error|warning>", "files": [..], "titles": [..]}`.
  Keine Prosa-Findings auf stderr. Exit-Codes identisch. Mit `--change`
  oder expliziten Pfaden: kein Baum-Scan (wie heute), JSON enthält nur
  die Parse-/Wip-Findings dieses Scopes.

### Review-Gate (`src/cli/check.rs`, `run_mandatory_checks("review")`)

Zusätzlicher Pflicht-Check nach integrity:

0. Existiert `review.md` nicht, `bail!` mit klarer Meldung, die
   `review.md` beim Namen nennt (der Gate braucht die Dispositionstabelle).
1. Baum-Scan (gleiche Analyse wie Lint).
2. Für jedes `shape_match`-**ohne-Widerspruch**-Paar mit ≥1 Mitglied in
   `givn/changes/<id>/specs/`: Zeile in der Dispositionstabelle von
   review.md erforderlich. Fehlt eine → `bail!` mit Liste der Paare
   (Titel beider Szenarien). Paare rein in permanenten Specs: nicht
   erforderlich (Altbestand, in früheren Changes disposiert).
3. Für jedes `long_scenario` mit ≥1 Mitglied im Change-Delta: Zeile mit
   `split-or-keep`-Wert (`split`|`keep`) erforderlich, sonst `bail!`.
4. **Net-Delta-Receipt** auf stdout: Zählung `added`/`modified`/`removed`
   aus den Delta-Tags der Change (leere Deltas → 0/0/0). Format:
   `net delta: N added, M modified, K removed` plus
   `net delta json: {"added":N,...}`-Zeile (maschinell greifbar).

### Dispositionstabelle (review.md, Formatvertrag)

```markdown
## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `Titel A` | `Titel B` | boundary |
```

Werte: `merge` | `delete` | `boundary` | `related`. Titel in Backticks
(flexible Abtrennung). Zusätzlich:

```markdown
## Split-or-keep

| Scenario | Decision |
|---|---|
| `Titel C` | keep |
```

`disposition.rs` parst beide Tabellen tolerant (Zellen-Trim,
Titel-Substring-Match auf den exakten Szenario-Titel).

## Step definitions

Eine Datei pro Capability (Konvention des Repos, tests/steps/mod.rs):

- `tests/steps/spec_overlap_gates_steps.rs` — neue Steps:
  `the command exits 3`, `stdout is valid JSON`, Reuse der bestehenden
  Given/When/Then aus `tests/steps/mod.rs` (Fixture-Baum, `.feature`-Datei
  schreiben, Befehle ausführen).
- `tests/steps/review_disposition_gates_steps.rs` — Steps:
  `the change's review.md contains a disposition row naming "X" and "Y"`,
  `the change's review.md contains a split-or-keep row for "X"`,
  `the change's review.md has no disposition entries`.

Registrierung: beide Dateien in `tests/steps/mod.rs` (`mod …`).

## Test runner

- **Runner**: `./run-tests.sh` → `cargo test --test features_runner`
  (verify.command bereits konfiguriert). E2E: `./run-tests.sh --e2e`.
- **Strict mode**: `fail_on_skipped()` ist in `tests/features_runner.rs`
  bereits aktiv — undefined/skipped Steps schlagen hart fehl.
- **Stub-Muster**: `unimplemented!()` in Schrittkörpern während RED.
- **Single-Scenario-Kommando** (RED/GREEN-Schleife): cucumber 0.23 lehnt
  `--name` + `--tags` kombiniert ab (`conflicts_with` in cucumber::cli).
  Szenario-Titel sind im Baum eindeutig; daher nur `--name`:
  ```bash
  GIVN_FEATURES=givn/changes/spec-overlap-gates/specs \
    cargo test --test features_runner -- --name "<Szenario-Titel>"
  ```
  @e2e-Szenarien während der Entwicklung:
  `cargo test --test features_runner -- --tags @e2e`.

## E2E infrastructure

- Runner-Kommando `./run-tests.sh --e2e` (bereits als verify.e2e_command
  konfiguriert). Keine neuen externen Dienste: Die reale Schnittstelle ist
  das echte Binary (`CARGO_BIN_EXE_givn`, bestehender Black-Box-Seam in
  `tests/common/mod.rs`). Fixture-Bäume in `TempDir`s. Keine digitalen
  Zwillinge nötig — es gibt keine Dritt-Systeme.
- Lokaler Start: `cargo build --release` (Single Binary, kein Service).

### Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| Run `givn lint` across the whole active tree (default scope) | A duplicate title across two permanent spec files is a hard error | CLI | `CARGO_BIN_EXE_givn` Subprocess in TempDir-Fixture mit zwei Feature-Dateien |
| Run `givn lint --json` for machine-readable findings | The json output reports duplicate-title findings machine-readably | CLI | `CARGO_BIN_EXE_givn` Subprocess mit `--json`, Assertion auf geparstem JSON |
| Run `givn check review --change <id>` on a change with overlap findings | Review passes when every shape match is disposed and reports the net-delta receipt | CLI | `CARGO_BIN_EXE_givn` Subprocess auf Fixture-Change mit disposiertem Shape-Match |

## Coverage process boundaries

Keine neuen Prozesse: Der Test-Runner (`cargo test`, Subprocess des
Binarys) ist der einzige instrumentierte Prozess; bestehende
Coverage-Konfiguration (`coverage.measure_command`) bleibt unverändert.

## Evidenzpflicht: Fingerprint-Messung auf watn

Nach Implementierung des Fingerprints (Szenario-Kern komplett) läuft eine
Messung gegen die 18 gelabelten Paare aus
`~/projects/watn/docs/feature-step-overlap-report.md`: Recall = Anteil
gefundener Duplikat-Paare (exakte Titel sind per Definition 100%; gemessen
wird die Zahl der Shape-/Subset-Treffer pro gelabeltem Paar, insbesondere
Paraphrasen-Fälle mit verschiedenem Titel). Ergebnis → Datei
`givn/changes/spec-overlap-gates/evidence-fingerprint.md` und entscheidet
über Change 4. Messaufbau: kleines Wegwerf-Skript
(`tools/overlap-measure/`), das die Overlap-Module als Bibliothek nutzt —
kein Produktcode-Änderungsgrund.

## Risiken & mitigations

- **Dispositions-Volumen im eigenen Baum**: Der Review-Gate erzwingt
  Dispositionen nur für Paare **ohne** Widerspruch. Exit-Code-Kontraste
  (0/2/3) zwischen eigenen Szenarien sind Widersprüche → nur Hinweise.
  Das verbleibende Volumen wird nach Implementierung gemessen (Tracked
  Open Question in design-review.md) und im Review disposiert.
- **Lint-Dauer**: Baum-Scan ist O(n²) über Szenario-Paare. n ≈ 200 →
  vernachlässigbar. Kein Caching nötig (Measure-first-Prinzip).
- **Exit-Code-3-Auswirkung auf Hooks**: Projekte, die `post: [lint, …]`
  konfiguriert haben, erben den neuen Hard Error (Hook behandelt jeden
  Exit != 0 als Fehlschlag, sichtbar als "command exited with status: 3").
  Gewollt — der Gate soll in Hook-Pipelines blockieren; für
  Maschinenkonsum dient `--json` auf stdout.

## arc42 (Addon aktiv)

`addons.arc42: true` — die Change trägt `arc42.md` mit folgenden Kapiteln
als **Affected: Yes** (aus der Auswahl-Tabelle der arc42-docs-Instruktion
abgeleitet): 03 Kontext, 05 Bausteinsicht (neues Modul `src/overlap/`),
06 Laufzeitsicht (Lint-Scan, Review-Gate), 08 Querschnittskonzepte
(Exit-Code-Konvention 1>3>2, JSON-Findings-Vertrag), 09
Entscheidungsarchitektur (MADR: deterministischer Fingerprint statt
Modell), 11 Risiken (Dispositionsvolumen, Fingerprint-Konsequenzen), 12
Glossar (fingerprint, shape match, disposition table, net-delta receipt).
Alle betroffenen Kapitel werden inhaltlich editiert (kein Template-Stand);
Kapitel 11 trägt die Konsequenzen der Kapitel-09-Entscheidung.
