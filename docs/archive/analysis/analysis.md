# Givn 0.3 Refactor: Analyse und Entscheidungsstand

> **Status: historical archive.** This is the 2026-08-14 refactor analysis and
> decision context. It describes a pre-retrieval design state, not a current
> implementation or measurement report. The retrieval-era evidence reports
> (`semantic-duplicates-real-model-filter-report.md`,
> `semantic-duplicates-e5-threshold-vs-serialization-analysis.md`) were removed
> with the semantic engine.

Date: 2026-08-14

Status: Ergebnis einer Diskussion zwischen Autor und Modell. Implementiert
ist nichts; das Dokument hält Kontext, Befunde, Begründungen und offene
Fragen fest.

Revision vom 2026-08-14 (Design-Review-Grilling): Das Dokument wurde nach
einem Durchgrillen des Entscheidungsstands aktualisiert. Revisionen: Tier 2
gestrichen, Enum-Maske gestrichen, gepinnte Embedding-Scores gestrichen,
Review-Protokoll 2, Pfad-Korrekturen und Drift-Befund in 1.4, Fingerprint-
Semantik für Outlines/Docstrings/Tabellen, Sequenzierung in 7.

Grundlage: fünf Dokumente im watn-Repository (`watn/docs/`):

- `feature-step-overlap-report.md` — Baseline-Befund
- `givn-plan-deepseek-pro.md`
- `givn-plan-terrapro.md`
- `qwen3.8-verdict.md`
- `givn-embedding-idea.md`

## 1. Ausgangslage

### 1.1 Das Problem

watn (der Referenz-Konsument von givn) hat über mehrere Wochen eine
Spec-Basis akkumuliert, die strukturell redundant ist:

| Oberfläche | Anzahl | Ausführungsstatus |
|---|---:|---|
| Aktive Feature-Dateien | 25 | ausgeführt |
| Aktive Szenarien | 223 | ausgeführt |
| Archivierte Feature-Dateien | 26 | nicht ausgeführt (Evidence) |
| Archivierte Szenarien | 216 | historisch |
| Step-Dateien | 30 (inkl. mod.rs) | 29 Module registriert |
| Binding-Attribut-Deklarationen | 858 | global registriert |

Der Report `watn/docs/feature-step-overlap-report.md` dokumentiert: zwei exakte
Titel-Duplikate (F1, F2), drei Subset-Szenarien (F3, F5, F6), wiederholte
Abdeckung derselben Invarianten über bis zu sieben Feature-Familien
(F8–F12), duplizierte Step-Mechanics über Module (S1–S5) und 23 lange
Szenarien. Die Suite war trotzdem grün — Grün bewies nur, dass globale
Bindings registrierbar sind, nicht dass die Szenarien unabhängig sind.

### 1.2 Prozess-Ursachen (aus dem Report)

1. **Change-lokale Planung.** Reviews erzwingen 1:1-Zuordnung zwischen
   Change-Inventar und eigenen Szenarien, aber keine Ownership-Prüfung
   gegen den gesamten permanenten Baum.
2. **Additives Archive.** Archive verschiebt Artefakte und hängt permanente
   Specs an; stärkere Szenarien wurden addiert, ältere Subsets blieben aktiv.
3. **Entry-Point-Slicing.** Szenarien wurden nach Kommandos organisiert
   (`watn setup`, `watn provider`, `watn models`), nicht nach Invarianten —
   dieselbe Invariante wurde pro Kommando neu eingeführt.
4. **Globale Cucumber-Registrierung.** Um Binding-Kollisionen zu vermeiden,
   erfanden spätere Capability-Module neue Prosa statt zu wiederverwenden —
   Alias- und Helper-Duplikation als Nebenwirkung.
5. **Superset-Additionen ohne Cleanup.** Stärkere Szenarien kamen später
   dazu; die schwächeren blieben, weil die Arbeit als additive Abdeckung
   galt.
6. **Befunde wurden nie zum Gate.** Frühere Reviews erkannten Duplikate
   (change-spezifisch) — daraus entstand kein repository-weiter Check.

### 1.3 Die drei Vorschläge und ihre Ziel-Ebenen

- **DeepSeek-Pro-Plan**: Enforcement. Es gab kein Gate, das Duplikate
  blockiert. F1/F2 hatten identische Titel — ein normalisierter
  Textvergleich hätte gereicht. Inkrementelle Phasen (Lint-Scope-Fix →
  Overlap-Lint → supersedes → Ledger → Skill-Rewrites).
- **Terra-Pro-Plan**: Transaktionssemantik. Der Runner führt alte und neue
  Szenarien nebeneinander aus, Archive ist additiv, Merge-Identität ist
  der Titel, Garantien in Skills überzeichnen das Binary. Antwort:
  V2-Kontrakt mit Ankern, Registry, CAS, Journal.
- **Embedding-Idee** (Autor): Retrieval. Der Autor sieht ähnliche
  Szenarien nicht und schreibt deshalb neue. Antwort: lokaler
  Embedding-Index mit exakter Duplikat-Sperre und semantischer
  Ähnlichkeitsprüfung.

Das Verdict (qwen3.8) bestätigte beide Pläne als faktisch solide, mit
einer Korrektur: DeepSeek ist der bessere Ausführungsplan, Terra die
bessere Systemanalyse. Die Diskussion endete nicht beim Verdict, sondern
hinterfragte dessen ungeprüfte Prämisse (dass semantische Suche der
Engpass sei) und rekonstruierte die Entscheidungen neu.

### 1.4 Verifizierte Code-Befunde (Grundlage)

- `src/gherkin/merge.rs:42-65` validiert nur gegen die **eine**
  Merge-Zieldatei (`givn/specs/<cap>/<cap>.feature`, geroutet per
  Capability-Tag). F1/F2 (gleiche Titel in verschiedenen Dateien) sind
  unsichtbar.
- `src/gherkin/delta.rs:5-12`: DeltaOp = Added/Modified/Removed; kein
  supersedes.
- `src/gherkin/merge.rs:77-84`: Modified/Removed ersetzen/löschen per
  Titel-Lookup.
- `src/gherkin/delta.rs:80-85`: Archive strippt alle `@givn.*`-Tags
  (Anwendung beim Merge-Rendering, `merge.rs:209-217`; Archive ist der
  Aufrufer).
- `src/cli/archive.rs:111-152`: merged **alle** Delta-Dateien eines
  Changes (Mehr-Dateien-Änderung ist möglich); Backup 82-109, Rollback
  nicht nur bei Verify-Fehler (164-171), sondern auch bei Verify-E2E
  (198-205) und Coverage-Gate-Fehler.
- `tests/features_runner.rs:154-165` **in watn** (`~/projects/watn`,
  externes Referenzprojekt — hier im givn-Repo nicht vorhanden): watn's
  Runner führt `givn/specs` plus die rohen Deltas aller aktiven Changes
  aus (`GIVN_ARCHIVE_ONLY` als einzige Ausnahme beim Archive-Verify).
- `src/cli/lint.rs` kennt nur ParseError/Wip; Default-Lint rekurriert in
  `givn/archive` (empirisch: 51 Dateien = 25 aktiv + 26 archiviert).
- `@e2e`-Regel existiert in **4–6 Wortlauten über 5+ Dateien** — nicht
  drei: `assets/skills/givn-spec/SKILL.md.tmpl:69-72` und `:79-83`
  (zwei gleichzeitig), `assets/instructions/specs.md:74,94-107`,
  `assets/instructions/design.md:210-211`,
  `assets/instructions/review.md:193-198`,
  `assets/commands/givn-spec.md:89-97`. Zusätzlich ein Widerspruch in
  `SKILL.md.tmpl`: Zeile 41 definiert Flag-Kombinationen als
  Inventory-Einträge, `:79-83` verbietet genau ein @e2e pro Variante.
  Konsequenz für 2.9: "eine normative Prosa-Quelle" beschreibt einen
  Sollzustand — heute tragen Templates und `assets/commands/*.md`
  ebenfalls Normtext, und der Projekt-Override-Cascade
  (`src/skills/render.rs:12-27`) muss bei der Konsolidierung mitbehandelt
  werden.

## 2. Entscheidungen

### 2.1 Identität: baumweit eindeutige Titel

**Problem.** Der Merge-Engine identifiziert Szenarien über (Titel,
Datei). Zwei Szenarien mit gleichem Titel in verschiedenen Dateien sind
mehrdeutig (F1/F2); Renames sind Identitätswechsel; parallele Edits am
selben Szenario sind nicht auflösbar (Lost-Update).

**Optionen.** (a) Terra's stabile Anker + Registry-Revisionen + CAS;
(b) erzwungene baumweite Titel-Eindeutigkeit.

**Begründung für (b).** Titel-Eindeutigkeit löst die Mehrdeutigkeit —
das einzige Identitätsproblem, das in watn tatsächlich belegt ist (F1/F2).
Lost-Update ist hypothetisch: parallele Arbeit passiert in separaten
Worktrees/Branches, ein beobachteter Vorfall existiert nicht. Terra's
Maschinerie wäre allein größer als der Rest des Vorhabens und löst ein
Problem ohne belegte Instanz.

**Konsequenzen.** Ein Rename ist ein Identitätswechsel: Offene
Modified-Targets (Deltas, die den alten Titel referenzieren) brechen
laut am Archive ("target not found") — nicht still. Der Autor
korrigiert die Referenz. Das wird akzeptiert.

**Zusatzargument.** Die Autoren sind LLM-Agenten, die Referenzen lesbar
in Delta-Dateien schreiben müssen. Ein Verweis wie "ersetzt
`Bash widget runs …`" (als removed+added-Paar mit Prosa-Link in
review.md, 2.2) ist im Diff les- und tippbar; ein Anker-UUID wäre ein
Lookup-Schritt bei jedem Schreiben und eine Copy-Paste-Fehlerquelle.
Prosa-Identität passt zum KI-getriebenen Workflow; unsichtbare IDs
nicht.

### 2.2 Supersession ohne neuen Merge-Op (Option C)

**Problem.** Stärkere Szenarien müssen schwächere ablösen können
(Report: "supersedes ist ein First-Class-Op, atomar"). Ohne das bleibt
der F4-Mechanismus bestehen: additiv statt ersetzend.

**Optionen.** (a) `@givn.supersedes` nur innerhalb derselben Datei;
(b) Cross-File-Supersession als Merge-Op mit baumweiter Vorvalidierung;
(c) kein neuer Op — `removed` + `added` über zwei Delta-Dateien, Gates
prüfen das Ergebnis.

**Begründung für (c).** Die Korrektur während der Analyse: Cross-File
ist heute **schon** ausdrückbar — `archive.rs` merged alle Delta-Dateien
eines Changes, jede in ihre Capability-Datei; Backup-Rollback existiert.
Der Unterschied der Optionen ist nicht "möglich/unmöglich", sondern wo
die Kausalbeziehung "A ersetzt B" festgehalten wird: (a) verliert sie
über Dateigrenzen — genau dort, wo die Duplikate entstanden (F1/F2/F3
sind Cross-File); (b) macht sie zum Maschinenbeleg, kostet baumweite
Vorvalidierung und setzt die Eindeutigkeits-Migration voraus (ein
`@givn.supersedes "Titel"` wäre in der heutigen Welt mehrdeutig); (c)
lässt sie als Prosa in review.md, prüft das Ergebnis deterministisch.

(c) gewählt: keine Engine-Änderung, die Gates entstehen ohnehin, nichts
verfestigt sich — ein späterer supersedes-Tag bliebe als reine
Engine-Validierung offen.

**Akzeptierte Schwäche.** Der Net-Delta-Receipt meldet
`removed: 1, added: 1`, kann aber nicht beweisen, dass das Paar
zusammengehört. Der schwächste Durchsetzungspunkt im ganzen Konstrukt;
kompensiert durch die Dispositionstabelle (2.3) und eine weiche
Review-Anweisung: Der Reviewer prüft jedes removed+added-Paar auf eine
Supersession-Aussage oder eine explizite "nicht zusammengehörig"-
Erklärung. Ein zusätzlicher maschineller Gate dafür wurde im Review
verworfen (unnötige Komplexität bei unklarem Mehrwert — der Reviewer
sieht den Receipt ohnehin).

**Rename-Semantik.** Ein Titel-Rename ist heute nur als
removed+added-Paar ausdrückbar (`delta_op` kennt keinen Rename-Tag);
die Kausalität "dasselbe Szenario, neuer Name" lebt als Prosa in
review.md. Das Paar erscheint als Delta-vs-Delta-Shape-Match und
erzeugt eine Pflicht-Disposition "Rename, selbes Szenario". Kein neuer
Engine-Op.

### 2.3 Zweistufige deterministische Prüfung

**Der prägende Einblick.** Wenn zwei Szenarien sehr ähnlich *aussehen*,
gibt es zwei Fälle: (1) sie decken dasselbe Verhalten ab → eines wird
gelöscht (welches, hängt von E2E-Coverage und Varianten ab); (2) sie
sind verschieden, aber schlecht benannt → jeder menschliche Reviewer
hätte dasselbe Problem → die Benamung ist der Fehler, die Titel müssen
so unterschiedlich werden, dass sie nicht mehr ähnlich klingen. Der Gate
muss nicht entscheiden, welcher Fall vorliegt — er meldet Ähnlichkeit,
der Autor klassifiziert, der Gate prüft das **Ergebnis** der
Klassifikation: Fall 1 durch Verschwinden, Fall 2 durch unterscheidbare
Titel.

**Revision aus dem Review.** Der ursprüngliche Tier 2 ("sehr ähnlicher
Titel + Fingerprint-Match → Umbenennen oder Löschen erzwingen") ist
gestrichen: Ein Block auf einem Fuzzy-Titel-Score verletzt das Axiom
aus 2.5 (ein Klassifikator darf Fehler machen, wenn er nur findet —
nicht, wenn er blockt) und hätte eine Schwellwert-Kalibrierung auf dem
Motivations-Datensatz selbst erzwungen (kein Held-out-Set). Die beiden
verbleibenden Tiers unterscheiden sich nur im Titel-Befund:

| Tier | Befund | Aktion |
|---|---|---|
| 1 | Identischer Titel baumweit | Hard Error |
| 2 | Fingerprint-Match, Titel beliebig | Warnung + Pflicht-Disposition in review.md |

**Tier-1-Scope.** Der Scan läuft über den **ganzen Baum**: permanente
Specs plus die Deltas **aller** aktiven Changes (eigene und fremde) —
Konflikte paralleler Changes fallen beim Lint auf, nicht erst beim
Archive-Merge. Rückwärtskompatibilität ist nicht im Scope: Altlasten
(in watn F1/F2) werden manuell bereinigt. givns eigener Baum ist
titel-eindeutig (verifiziert: 154 Szenarien, null Titelduplikate —
die anfangs vermuteten `Bar`/`Ignored`/`S` sind Fixture-Text in
Docstrings, keine Szenario-Titel) und braucht keine Cleanup-Change.

**Modified-Ausnahme.** Ein `@givn.modified`-Szenario trägt per
Konstruktion denselben Titel wie sein permanenter Vorgänger. Genau ein
Paar ist erlaubt: Delta vs. sein Merge-Target mit gleichem
Capability-Tag **und unverändertem Titel**. Alles andere (Delta-vs-
Delta, Delta-vs-andere-Capability) bleibt Error. Ein Rename ist kein
Modified — er läuft als removed+added-Paar (2.2).

**Disposition.** Die Pflicht-Disposition ist maschinell erzwungen:
Fingerprint-Match ohne Disposition-Eintrag → Review blockt (analog zur
Split-or-keep-Spalte in 2.6). Die Disposition trägt die
Klassifikation des Autors (Duplicate/Variant → Zusammenlegen oder
Löschen; eigene Boundary → Begründung), die der Reviewer verifiziert.
Die Namensqualität überlebender Paare ist **Reviewer-Urteil** — kein
maschineller Post-Disposition-Check auf Titel-Ähnlichkeit.

Zusätzlich: Shape-Match mit hartem Widerspruch → `related_scenario`
(Info, kein Block). Subset (Then-Folge B ⊂ Then-Folge A) → Warnung
"Teilmenge — löschen oder Boundary im Titel".

Hard-Fail nur auf Sicherheit: der identische Titel ist deterministisch
und unbestreitbar; alles Weichere warnt und verlangt Disposition.

### 2.4 Fingerprint-Algorithmus (zustandslos)

Kein Index, keine Persistenz, deterministisch auf jeder Maschine,
CI-fähig ohne Setup.

**Schritt 1 — Normalisierung.** `And`/`But` auf den semantischen Typ
des Vorgängers auflösen; quoted Strings → `<v>`; Zahlen → `<n>`;
Polarität neutralisieren (`should not contain X` → `should contain X`
plus Fact `not_contains: X`), damit positive/negative Paare im Shape
matchen und die Unterscheidung in die Facts wandert; Whitespace
kollabieren. **Keine Enum-Maske**: Die ursprünglich geplante pro-
Projekt-Enum-Liste (Shells, Modell-/Provider-Namen) ist gestrichen —
der Normal-Kontext (Quoted Strings, Zahlen) ist bereits abgedeckt, ein
falscher Eintrag hätte echte Unterschiede ausgelöscht (Erasure ist
schlimmer als ein verpasstes Match), und Config-Key plus Migration
wären Kosten ohne Beleg. Wiedereinführung nur mit Dogfood-Daten.

**Schritt 2 — Shape-Vergleich.** Then-Sequenzen separat per LCS
(Assertions sind das primäre Signal, Given/When schwächer gewichten);
Subset-Erkennung über die Then-Folgen; Scenario Outline pro
Examples-Zeile abflachen — jede geflattete Zeile zählt als eigenes
Szenario, Vergleiche laufen paarweise über geflattete Zeilen;
Background-Steps dem Kontrakt voranstellen (vererbter Kontext gehört
zum Kontrakt).

**Schritt 3 — Facts und Widersprüche.** Exit-Codes, stdout/stderr
contains/not-contains, `[DONE]` vorhanden/fehlt, TTY/non-TTY,
CLI-Flags, Config-Präsenz, Kommandoname; Data Tables mit Kopfzeile →
Spaltenkopf als Key, Werte maskiert mit Literal in den Facts;
Tabellen ohne Kopfzeile → Positions-Index als Key. Docstrings werden
**ignoriert** (kein Shape, keine Facts — akzeptierte Lücke der
Facts-Extraktion). Harte Widersprüche: unterschiedliche Exit-Codes,
contains vs. not-contains mit gleichem Literal, `[DONE]` vs. ohne
`[DONE]`, TTY vs. non-TTY, Erfolg vs. erwarteter Fehler. Bei
Widerspruch nie Duplikat-Klassifikation — egal wie ähnlich der Shape
ist. **Policy bei nicht extrahierbaren Facts:** Ein Widerspruch zählt
nur, wenn **beide** Seiten den Fact extrahieren; fehlende Extraktion
= kein Fact, konservativ — mehr Warnungen, nie eine Klassifikation auf
fehlenden Facts. (Die 2.8-Abgrenzung gilt explizit: Gherkin ist
givns eigenes Format und darf geparst werden; Step-Definitionen sind
fremder Testcode und bleiben tabu.)

**Schritt 4 — Titel-Ähnlichkeit.** Normalisierte Titel, Token-LCS/Jaccard,
als Namensqualitätssignal für die Disposition — das Urteil über
umzubenennende Titel fällt der Reviewer, kein maschineller Check.

**Maskierungs-Kontradiktion zwischen den Vorarbeiten.** DeepSeek will
Zahlen maskieren — das würde Exit-Codes mitmaskieren, und dann matchen
sich Szenarien, die Exit 0 vs. Exit 3 fordern (falsch). Die
Embedding-Idee des Autors ist hier korrekt: Exit-Codes, CLI-Flags,
`[DONE]`, erwartete Literale und Polarität tragen den
Verhaltensunterschied und gehören in die Facts, nicht in die Maskierung.
Diese Architektur (Shape getrennt von Facts) übernimmt der
Fingerprint.

**Bekannte Lücken.** (1) Paraphrasen — gleiche Bedeutung, andere
Wortwahl ("exit status should be 0" vs. "the program exits
successfully"); (2) Schrittzerlegung (durch Authoring-Regeln gegen
Composite-Steps begrenzt); (3) unquotete Domänenwerte in Prosa (Folge
der gestrichenen Enum-Maske — nur Zahlen und Quoted Strings werden
maskiert); (4) Given-Reihenfolge. Lücken 2–4 akzeptiert; Lücke 1 ist
der einzige Kandidat für eine Embedding-Schicht — und entscheidet
nicht allein, siehe 2.5 und 6.

### 2.5 Embeddings: nur Retrieval, und nur bei gemessenem Mehrwert

**Der entscheidende Reframe.** Lücke 1 zerfällt in zwei Teilprobleme:
**Finden** (ein semantischer Nachbar muss auftauchen) und **Beurteilen**
(ist er Duplikat, Variante oder eigene Boundary?). Das Beurteilen ist
bereits gelöst — gratis: Der Autor ist ein LLM-Agent, der eine Handvoll
Kandidaten selbst klassifizieren kann; der Reviewer verifiziert in der
Disposition. Der mehrstufige Klassifikator der Embedding-Idee ist
Überbau auf der falschen Hälfte. Übrig bleibt nur das Finden, und
Finden braucht Recall, keine Präzision: Ein Klassifikator darf Fehler
machen, wenn er nur findet; er darf keine machen, wenn er blockt.
(Nach der Streichung von Tier 2 gilt dieses Axiom wieder durchgängig —
der einzige Block ist der deterministische identische Titel.)

**Bedingter Bau.** Der Handoff-Befund "deterministisch fängt nur 2/8
Dup-Paare" vergleicht gegen exakte Titel, nicht gegen den geplanten
Fingerprint — ein Strohmann. Die Retrieval-Schicht wird deshalb erst
gebaut, wenn der fertige Fingerprint auf den 18 gelabelten Paaren
gemessen ist und Lücke-1-Fälle (Paraphrasen mit verschiedenem Titel)
nachweislich durchrutschen. Bis dahin: deterministische Schicht
zuerst, Embeddings sind nicht entschieden.

**Laufzeit.** Single-Binary bleibt: `givn spec index` + `givn spec
search` als Rust-Kommandos mit `fastembed-rs` (unterstützt
`intfloat/multilingual-e5-large`). Kein Shell-out auf das Python-
Experiment, kein externer Dienst. Modell-Download beim ersten
Index-Lauf, danach lokal gecacht. Die Gates brauchen den Index nie —
kein zustandsabhängiger Block, CI bleibt setup-frei.

**Suchzeitpunkte.**
1. Verhaltens-Query vor dem Schreiben (givn-spec, Schritt 0) — drei
   Queries: User-Action, Kern-Assertion, Boundary. Verhindert, dass das
   Szenario in die falsche Richtung entsteht (F8-Muster: sieben
   Feature-Familien für eine Tier-Auswahl). Propose/explore können die
   Suche optional früher nutzen.
2. Fertiger Gherkin nach dem Schreiben — gegen die permanenten Specs,
   nicht gegen den eigenen Delta (Selbst-Match ausschließen). Höchste
   Recall, weil Szenario-gegen-Szenario-Matching die stärkste
   Embedding-Form ist. Gilt auch für `@givn.modified`-Szenarien: der
   F2-Duplikat entstand laut Historie über einen modified-Delta
   (eb328dd).

**Review-Protokoll 2 (keine gepinnten Scores).** Der Reviewer
aktualisiert den Index und queried eigenständig gegen den **frischen**
Baum — nicht gegen die Aufzeichnung des Autors. Jeder Treffer ohne
Disposition ist ein Befund; ein staler Autoren-Index kann den Check
nicht austricksen, weil der Check nie auf dem Autoren-Zustand basiert.
Gepinnte Scores (Datei, Titel, Score) sind gestrichen: Sie sind über
Modelle/Maschinen nicht reproduzierbar und unter Protokoll 2
überflüssig — die Disposition trägt Entscheidungen, nicht Zahlen.

**Validierungsmetrik.** Recall@k (k=5) auf den bekannten Duplikat- und
Boundary-Paaren des Reports — nicht Schwellwert-Trennung, weil auf dem
Score niemand blockt. Achtung bei der Interpretation: die 8 Dup-Paare
sind das Motivations-Set selbst, kein Held-out.

**Index.** Inkrementell per content_hash, deckt den aktiven Baum ab
(permanente Specs; aktive Deltas anderer Changes nur hinter einem
Flag, eigene Deltas nie — Treffer gegen offene Deltas sind transiente
Störung), lokal, Modell-Download beim ersten Lauf. Der Index wird vor
dem Review-Query aktualisiert.

**Deutsche Anforderung.** "givn-Szenarien dürfen deutsch sein" ist ein
Produktprinzip (givn ist sprach-agnostisch), kein Datenbefund — das
watn-Korpus ist vollständig englisch, die 0/8-DE-Ergebnisse des
Experiments stammen von synthetischen deutschen Sätzen und sind
schwaches Indiz. Die Modellwahl (e5-large, 2,24 GB) hängt an dieser
Prinzip-Entscheidung, nicht an Messung.

### 2.6 Längengates: Hinweis, kein Hard-Fail

Der Report hat eine Skala (10–14 inspizieren, 15–19
Boundary-Entscheidung, 20+ Rechtfertigung); DeepSeek macht >19 zum
Error außer mit `@long.rationale`. Verworfen aus zwei Gründen: Der
Report warnt selbst doppelt — Split erzwungene Transaktionen nicht nur
wegen des Step-Counts, und verstecke Workflows nicht in Composite-Steps.
Ein hartes Gate setzt den Anreiz genau in die verbotene Richtung: Der
Agent kürzt sichtbare Steps, indem er Seitenabläufe in einen
undurchsichtigen Step "complete setup" verpackt. Und die 23 langen
Szenarien in watn sind dort lang, wo sie es sein müssen
(PTY-Setup-Präambeln, echte Transaktionen).

Stattdessen: Lint warnt bei >14 Steps; die Dispositionstabelle bekommt
eine Spalte "split-or-keep" (echte Transaktionsgrenze oder Begründung
für Zusammenhalt); maschinell prüfbar bleibt nur: langes Szenario ohne
Eintrag → Review blockiert. Der Rest ist Reviewer-Urteil.

### 2.7 Test-Policy: Black-Box first

**Prinzip.** Möglichst immer die echten Schnittstellen testen.
Interne Logik nur im Ausnahmefall — sie kann sich ändern, ohne dass
sich beobachtbares Verhalten ändert. E2E ist teuer (Laufzeit), daher
sind schnellere Unit-Tests für Edge-Cases angebracht. Hexagonal:
Domainkern mit Fake-Adaptern, Happy Path und produktive Schnittstellen
per E2E (Beispiel: InMemory-Repository für alle Fälle,
Postgres-Testcontainer-Repository für den Happy Path).

**Regel.** Ein interner Test darf nur Fälle abdecken, die das E2E nicht
abdeckt — nie denselben Fall. Review-Frage: "Welchen Fall deckt dieses
Szenario, den das E2E nicht deckt?" Ohne Antwort wird gelöscht.

**Konsequenz für den Report.** F1's nicht-E2E-Szenario wird gelöscht,
nicht umbenannt; Report-Empfehlungen wie "rename it to a distinct
lower-level search-state contract" (F6) werden zu Löschungen — die
Konsolidierung wird aggressiver als der Report plant. Die Policy
gehört in die Skills (givn-design, givn-steps), nicht in Gates.

### 2.8 Step-Seite

**Entscheidungen.** Kein Source-Parsing im Binary (ein generisches CLI
soll keinen fremden Testcode interpretieren — `givn steps report` mit
sprachabhängigen Regex verworfen). Kein projektseitiger Binding-Index
(zu komplex).

**Zweiteilung der Duplikation.** (1) Szenario-getriebene Aliase (S3, S5)
lösen sich durch die Szenario-Konsolidierung — entfallene Szenarien
entziehen den Alias-Bindings den einzigen Aufrufer. (2)
Mechanics-Duplikation (S1/S2: Page-Poller, PTY-Treiber) überleben die
Konsolidierung, weil ihre Szenarien gelayered sind und bleiben — sie
sind Prompt/Regel-Thema plus manuelles Helper-Refactor nach dem
Delegationsmodell (S6). Die Hypothese (Szenario-Arbeit mindert
Step-Duplikation) ist mit vorhandenen Daten testbar: pro Binding
zählen, wie viele aktive Szenarien es referenzieren, gegen die
Konsolidierungskandidaten schneiden.

### 2.9 Prosa: eine normative Quelle

`assets/instructions/*.md` ist die einzige Quelle für Regeltext. Skills
und Slash-Commands werden dünne Choreografie, die `givn instructions`
ausführt. Drift ist belegt: 4–6 `@e2e`-Wortlaute über 5+ Dateien
(Templates, `assets/commands/*.md`, drei Instruction-Artefakte) plus
ein Widerspruch innerhalb von `SKILL.md.tmpl` (Zeile 41 vs. 79-83) —
die Konsolidierung muss deshalb Templates **und** Commands **und** den
Projekt-Override-Cascade (`src/skills/render.rs`) abdecken, nicht nur
`assets/instructions`. Jede der Regeln aus dieser Diskussion müsste
sonst mehrfach gepflegt werden.

**Normative `@e2e`-Regel:** eine E2E-Evidenzzelle pro realer
User-Action, repository-weit. Ein zweites E2E-Szenario für dieselbe
Action braucht eine neue Schnittstellen-Rechtfertigung. Die
feature-lokale Formulierung ("pro User Interaction Inventory entry")
ist der Mechanismus, der in watn die Setup-, Modellwahl- und
Filter-Flows pro Capability dupliziert hat (F8/F10) — sie ist nicht nur
Drift, sie ist die Ursache eines Befunds.

## 3. Analytischer Befund: F1 präzise geklärt

Die erste Charakterisierung in der Diskussion ("legitime Schichtung,
umbenennen, nicht blockieren") war falsch; die Prüfung des Codes ergab:

- Szenario A (`credential-sources.feature:17-26`, `@e2e`): startet das
  echte Binary, prüft Exit-Status und den API-Key im echten
  HTTP-Request.
- Szenario B (`provider-setup.feature:187-195`): `provider_setup_steps.rs:
  185-199` ruft direkt `config::get_provider_api_key` auf und legt das
  Ergebnis in `world.pending_config["resolved_key"]` ab. Der Then-Step
  (`provider_setup_steps.rs:1183-1210`) degradiert bei vorhandenem
  `resolved_key` zu einem Rückgabewert-Vergleich — die Prosa "API
  request should use API key" überzeichnet, was tatsächlich geprüft
  wird. Kein Binary, kein Request, kein HTTP.

B ist eine strikte Teilmenge von A mit überzeichneter Assertion; A deckt
den Resolver-Pfad über das echte Binary mit ab. Richtige Disposition:
Löschung. Lehre für das Design: F1 hätte auch ein deterministischer
Check gefangen (identischer Titel) — für die Gate-Rechtfertigung der
Embeddings ist F1 kein Beleg; der Beleg wäre nur ein Fall mit
verschiedenen Titeln, den der Fingerprint verpasst. Genau das misst das
geparkte Experiment.

## 4. Verworfen (mit Grund)

- **Terra's V2-Kontrakt** (Anker, Registry, CAS, Journal,
  Verlustfrei-Garantie): löst Mutation/Lost-Update, das als hypothetisch
  eingestuft ist; allein größer als der Rest des Vorhabens zusammen.
  Die billigen Einzelbefunde (GIVN_FEATURES, Doc-Drift, Integritäts-
  Gate) bleiben unter "Verschoben".
- **Embedding-Blocking** (`probable_duplicate` → Block, Exit-Code 2):
  macht die Gates zustandsbehaftet (Indexfrische als Gate-Voraussetzung,
  Modell-Footprint inkl. CI-Agents, Kalibrierung pro Modell und
  Projekt). Ein Auto-Block hätte F1 zwar korrekt getroffen, aber auf
  einem Score, auf dem niemand blocken darf.
- **Tier-2-Fuzzy-Block** (sehr ähnlicher Titel + Fingerprint-Match →
  Umbenennen/Löschen erzwingen): ein Block auf einem Titel-Score
  verletzt dasselbe Axiom wie Embedding-Blocking und bräuchte eine
  Schwellwert-Kalibrierung auf dem Motivations-Datensatz selbst (kein
  Held-out). Ersetzt durch Pflicht-Disposition + Reviewer-Urteil über
  Namensqualität (2.3).
- **Enum-Maske im Fingerprint** (pro Projekt konfigurierbare
  Domänenwert-Liste): Erasure-Richtung gefährlich (falscher Eintrag
  löscht echte Unterschiede), Quoted-String-/Zahlen-Maskierung deckt
  den Normalfall ab, Config-Key plus Migration ohne Beleg. Wiedereinführung
  nur mit Dogfood-Daten (2.4).
- **Gepinnte Embedding-Scores in der Disposition**: über
  Modelle/Maschinen nicht reproduzierbar; unter Review-Protokoll 2
  überflüssig (2.5).
- **Supersession-Gate** (removed+added-Paar ohne review.md-Eintrag →
  Block): als unnötige Komplexität mit unklarem Mehrwert verworfen —
  der Reviewer sieht den Net-Delta-Receipt ohnehin; stattdessen weiche
  Prüfanweisung im Review-Skill (2.2).
- **Binding-Index und Source-Parsing**: ein generisches CLI soll keinen
  fremden Testcode interpretieren; projektseitiger Index zu komplex.
- **Längen-Hard-Fail**: setzt Anreiz zu Composite-Steps (2.6).
- **Dispositions-Tags (`@boundary.*`) und Mini-Ledger**: Der Default ist
  Löschung (2.7); für die seltenen Ausnahmen reicht Prosa in review.md.
  Der Ledger wäre der kleinste Verwandte der abgelehnten Registry.

## 5. Bewusst verschoben

- **Runner-Doppelausführung.** `tests/features_runner.rs:154-165` **in
  watn** (`~/projects/watn`, extern verifiziert): führt permanente
  Specs plus rohe Deltas aus — modified-Szenarien laufen neben ihrer
  veralteten Version, removed-Platzhalter bleiben ausführbar, bis
  archiviert ist. Konsequenz: Die Suite kann während eines Changes
  altes und neues Verhalten gleichzeitig verlangen; gelöschte Bindings
  müssen bis zum Archive am Leben bleiben. Hat nie bewusst gebissen —
  behandeln, wenn es als Problem erkennbar wird.
- **`GIVN_FEATURES`-Dead-Contract.** givn setzt die Variable, watn
  liest sie nie.
- **Doc-Drift `config.yaml` vs. `commands.yaml`.** Mit der
  Prosa-Konsolidierung mitbehandeln.

## 6. Offen

- **Fingerprint-Messung auf den realen Paaren.** Der Fingerprint wird
  zuerst gebaut und auf den 18 gelabelten Dup-/Boundary-Paaren des
  Reports gemessen (Recall, kein Schwellwert-Block). Erst wenn
  Paraphrasen-Paare mit verschiedenem Titel nachweislich durchrutschen
  (Lücke 1), wird die Retrieval-Schicht gebaut (2.5). Das
  Embedding-Experiment im HANDOFF hat geliefert, was es konnte —
  aber der deterministische Vergleichs-Baseline darin ist ein
  Strohmann (exakte Titel statt geplantem Fingerprint).
- **Dispositions-Volumen auf givns eigenem Baum.** 154 Szenarien,
  Fingerprint-Matches ungemessen — die Gate-Change muss die eigene
  Baum-Disposition mitmachen, deren Umfang ein Planungsrisiko ist.
- **Index-Zusammensetzung.** Treffer gegen offene Deltas fremder
  Changes sind transiente Störung; Flag-Entscheidung steht (2.5),
  Details erst beim Bau der Retrieval-Schicht.

## 7. Sequenzierung (fünf Changes, in dieser Reihenfolge)

Die Reihenfolge ist durch die Revisionen des Reviews erzwungen; die
Changes selbst sind Skizze, nicht ausgeführt:

1. **Gate-Change (deterministische Schicht).** Lint-Scope-Fix (Archive
   raus aus Default-Lint), Tier 1 (identischer Titel baumweit, Hard
   Error, Scope: permanente Specs + Deltas aller aktiven Changes,
   Modified-Ausnahme), Tier 2 als Pflicht-Disposition mit
   maschinellem Check, `--json`-Ausgabe, Split-or-keep-Spalte,
   Net-Delta-Receipt. In derselben Change: givns eigener Baum wird
   dispositions-sauber; Gate-E2E pro Tier gegen Fixture-Bäume über
   das echte Binary, Unit-Tests nur für Normalisierungs-Edge-Cases
   mit Fall-Begründung (Black-Box-first). Keine vorherige
   Cleanup-Change nötig — der eigene Baum ist titel-eindeutig.
   Fingerprint-Messung auf den 18 watn-Paaren als Evidenz für die
   Change-4-Entscheidung.
2. **Prosa-Konsolidierung.** `assets/instructions/*.md` als einzige
   normative Quelle; Templates, `assets/commands/*.md` und der
   Projekt-Override-Cascade (`src/skills/render.rs`) inklusive; neue
   `@e2e`-Norm.
3. **Skill-Rewrites.** givn-spec Suchschritte (Query + Gherkin, falls
   4 gebaut wird), givn-review Dispositionstabelle + entfernt die
   gepinnten Treffer, Supersession-Prüfung für removed+added-Paare
   (weich), givn-design/givn-steps Black-Box-Policy.
4. **Retrieval-Schicht — nur bei Fingerprint-Miss-Beweis aus 1.**
   `givn spec index`/`search` in Rust (fastembed-rs, e5-large),
   Review-Protokoll 2.
5. **watn-Konsolidierung zuletzt.** Dogfood der fertigen Gates
   (Report's Recommended Consolidation Order, korrigiert um die
   Black-Box-First-Policy: Löschungen statt Umbenennungen); baumweite
   Altlasten (F1/F2) manuell auflösen — Rückwärtskompatibilität ist
   explizit nicht im Scope.
