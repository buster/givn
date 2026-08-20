# Design: semantic-compact-serialization

## Technical Approach

Die Änderung bleibt auf die E5-/Retrieval-Darstellung in
`src/spec/retrieval.rs` begrenzt. `semantic_serialization` erzeugt weiterhin
eine deterministische, vollständige Darstellung in stabiler Reihenfolge. Sie
fügt jedoch optionale Abschnitte und optionale Felder nur ein, wenn deren
normalisierter Inhalt nicht leer ist.

Immer vorhandene fachliche Identität:

- Feature-Name und Feature-Schlüsselwort
- Rule-Name und Rule-Schlüsselwort, wenn eine Rule existiert
- Szenario-Name und Szenario-Schlüsselwort
- vorhandene Szenario-Schritte mit ihrer aufgelösten Rolle und ihrem Text

Bedingt vorhandene Inhalte:

- Feature-, Rule- und Szenario-Beschreibungen
- Feature- und Rule-Backgrounds einschließlich ihrer nichtleeren Schritte
- Step-Tabellen und Docstrings
- Scenario-Outline-Template
- Examples einschließlich Name, Beschreibung und Tabelle

Leere Werte erscheinen weder als `[]`-Platzhalter noch als leerer
Abschnittsmarker. Nichtleere Tabellen, Docstrings, Beschreibungen und
Examples werden vollständig und in Quellreihenfolge übernommen. Der bestehende
Content-Hash wird aus genau dieser kompakten Darstellung berechnet; dadurch
werden alte Indexeinträge bei einer Darstellungsänderung automatisch als nicht
wiederverwendbar erkannt.

Der semantische Umfang ist der geparste fachliche Gherkin-Inhalt. Kommentare,
Tags, Pfade, Scores, Klassifikationen und Fingerprint-Fakten bleiben wie bisher
Metadaten und sind nicht Teil des Modelltexts. Whitespace wird für semantische
Vergleiche deterministisch normalisiert; Docstring-Delimiter und reine
Quellformatierung sind nicht Teil des Vertrags. Bei einem `Scenario Outline`
bleiben Name/Keyword sowie die Platzhalter in den Step-Templates erhalten;
konkrete Examples-Zeilen bleiben separate vollständige Inhalte.

Die Serialisierungsgrammatik bleibt textbasiert: Abschnitte werden als
deterministisch geordnete Zeilen verbunden, strukturierte Tabellen und
Docstrings werden weiterhin JSON-escaped. Ein optionales Feld wird nur dann
geschrieben, wenn die normalisierte Darstellung ein `Some`-Ergebnis liefert.
Die Tests prüfen die exakten Zeilen und den daraus abgeleiteten Hash, damit
Auslassung, Reihenfolge und Escaping nicht nur über eine Negativsuche nach
`[]` abgesichert sind.

Die deterministische Stub-Einbettung bleibt unverändert. Sie dient weiterhin
dem Gherkin-Testlauf, während die reale E5-Inferenz dieselbe Darstellung
verwendet.

## Architecture Impact

- `src/spec/retrieval.rs`: kompakte Serializer-Helfer und direkte
  Serializer-Tests.
- `givn/changes/semantic-compact-serialization/specs/`: ausführbare
  Gherkin-Delta-Spezifikation.
- `tests/steps/semantic_compact_serialization_steps.rs`: reguläre
  Step-Definitionen für den Indexaufbau und die kompakten Inhalte.
- `tests/steps/semantic_compact_serialization_e2e_steps.rs`: getrennte
  E2E-Step-Definition für den realen CLI-Aufruf.
- `tests/steps/mod.rs`: Registrierung der beiden capability-spezifischen
  Step-Definition-Module.

Es gibt keine neue Persistenzstruktur und keine Änderung an E5-, BGE- oder
NLI-Schwellenwerten, Pair-Caps oder Token-Cap-Verträgen.

Die reguläre Gherkin-Prüfung verifiziert die kompakte Darstellung indirekt
über den `content_hash` des erzeugten Indexeintrags; ergänzende direkte
Serializer-Tests in `src/spec/retrieval.rs` verifizieren den exakten Modelltext
für leere und nichtleere optionale Inhalte. Die E2E-Prüfung bleibt auf der
öffentlichen CLI-Grenze und assertiert deren Index-Erfolg und stdout; der
Indexeintrag ist dabei nur sekundäre Persistenz-Evidenz.

## Verification

Der vollständige Gherkin-Runner ist `./run-tests.sh`. Er führt die Features
unter `givn/specs/` und die während des Changes unter
`givn/changes/semantic-compact-serialization/specs/` aus. Der E2E-Runner ist
`./run-tests.sh --e2e`.

Der Runner verwendet `cucumber-rs` mit `.fail_on_skipped()` in
`tests/features_runner.rs`; undefinierte oder übersprungene Schritte können
damit nicht erfolgreich durchlaufen. Neue Step-Definitionen enthalten keine
No-op- oder Pending-Körper.

Ein einzelnes Szenario wird mit folgendem Filter ausgeführt:

```sh
GIVN_FEATURES=givn/changes/semantic-compact-serialization/specs \
  cargo test --features spec-retrieval --test features_runner -- \
  --name "Empty optional sections do not add retrieval content"
```

Der gezielte Change-Lauf nutzt `GIVN_FEATURES` explizit. `./run-tests.sh` und
`./run-tests.sh --e2e` verwenden standardmäßig die permanenten Features; die
Givn-Hooks setzen `GIVN_FEATURES` beim Ausführen eines aktiven Changes.

Die Testumgebung ist ein CLI-Prozess im temporären Projektverzeichnis. Es
gibt keine externen Services, Datenbanken, Netzwerke oder Third-Party-
Abhängigkeiten, daher sind keine Digital Twins erforderlich. Der reale
E2E-Aufruf verwendet den kompilierten `givn`-CLI-Prozess und prüft dessen
stdout sowie den erzeugten persistenten Index.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| rebuild the permanent scenario retrieval index with `givn spec index` | A maintainer can index scenarios with complete compact passages | CLI / terminal | `cucumber-rs` starts the real `givn spec index` subprocess in an isolated temporary project and asserts exit status, stdout, and index file |

## Decisions

- Keine Serializer-Version wird als separater Index-Schalter eingeführt. Der
  vorhandene Content-Hash ist die Wiederverwendungsgrenze und verhindert, dass
  alte Vektoren mit einer anderen Eingabedarstellung weiterverwendet werden.
- Keine automatische Kürzung, Chunking-Logik oder Schwellenwertanpassung wird
  ergänzt. Überlange vollständige Passagen bleiben durch den bestehenden
  E5-Token-Cap sichtbar.
- Die interne Darstellung bleibt textbasiert statt JSON-only, weil E5 die
  fachlichen Wörter direkt verarbeitet und die bestehende Index-/Hash-
  Schnittstelle unverändert bleiben kann.
