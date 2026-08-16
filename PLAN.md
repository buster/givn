# PLAN.md — givn 0.3 Refactor: sechs Changes

Dieses Dokument spezifiziert die Refactor-Vorhaben aus
`docs/givn-refactor-0.3/analysis.md` (Entscheidungsstand nach
Design-Review-Grilling, 2026-08-14) als sechs aufeinanderfolgende givn-Changes.
Für jede Change enthält es mindestens den Inhalt einer proposal.md
(Problem/Opportunity, Proposed Solution, Out of Scope, Open Questions) in
User-Perspektive und Domain-Sprache, plus Spec-Anhaltspunkte
(Kandidaten-Szenarien), Design-Anhaltspunkte (technische Entscheidungen aus
analysis.md, die in design.md gehören) und Evidenz-/Verifikationspflichten.

Verwendung: Jeder Abschnitt 1–6 ist die Eingabe für die Erstellung einer
givn-Change (`givn propose`, dann spec/design/tasks). Proposals sind bewusst
in User-Perspektive geschrieben; alles Technische steht unter
"Design-Anhaltspunkte" und darf erst in design.md einfließen. Die Changes
werden nacheinander abgearbeitet — niemals parallel, die Abhängigkeiten sind
hart. Ergebnisse werden als Stacked PRs abgelegt (Abschnitt
"PR-Workflow — Stacked PRs").

Quellen: `docs/givn-refactor-0.3/analysis.md`,
`tools/embed-analysis/HANDOFF-EMBEDDINGS.md`, watn-Repo (`~/projects/watn`,
Referenz-Konsument) mit `watn/docs/feature-step-overlap-report.md`.

---

## Abhängigkeiten und Entscheidungspunkte

| # | Change (vorgeschlagene ID) | Repo | Voraussetzung |
|---|---|---|---|
| 1 | `spec-overlap-gates` | givn | keine (eigener Baum ist titel-eindeutig, verifiziert: 154 Szenarien, null Titelduplikate) — **erledigt, archiviert** |
| 2 | `workflow-enforcement-wip-cleanup` | givn | 1 (Gates aktiv; die permanente Spec muss vor der Prosa-Konsolidierung von Altlasten bereinigt werden) — **erledigt, archiviert** |
| 3 | `prose-consolidation` | givn | 2 (Regeltext wird nach Gates und WIP-Bereinigung konsolidiert) — **erledigt, archiviert** |
| 4 | `skill-rewrites` | givn | 3 (Skills dürfen nur noch choreografieren, was konsolidierte Instruktionen enthalten) — **erledigt, archiviert** |
| 5 | `spec-retrieval` | givn | 1 (Fingerprint-Messung belegt Paraphrasen-Durchrutscher; Retrieval wird gebaut) |
| 6 | `watn-consolidation` | watn | 1 (Gates müssen existieren; 2–5 optional für den Start) |

- Change 2 behandelte **16 echte @wip-Tags** in
  `givn/specs/givn-workflow-enforcement/givn-workflow-enforcement.feature`.
  Die Szenarien und Step-Definitionen laufen bereits grün; die Tags sind
  stale, weil `@wip` kein `@givn.*`-Tag ist und beim Archivieren nicht
  entfernt wird. Die Change verifizierte jedes Szenario und aktivierte es
  durch ein `@givn.modified`-Delta ohne Szenario-@wip. Ein dabei gefundener
  Merge-Randfall für den Tag des ersten Szenarios wurde in
  `src/gherkin/merge.rs` behoben und mit einem Regressionstest abgesichert.
  Fixture-interne `@wip`-Texte sind kein permanenter Tag und bleiben
  unverändert. **Ergebnis: archiviert.**
- Change 5 wird gebaut: Die Messung aus Change 1 (`evidence-fingerprint.md`)
  belegt 3 verpasste Paraphrasen-Paare (F5, F12a, F12b) — Lücke 1 der
  analysis.md ist real; Retrieval ergänzt die Gates als Authoring- und
  Review-Werkzeug, ohne Block auf Scores.
- Change 3 wurde archiviert: `assets/instructions/specs.md` ist die normative
  E2E-/Inventory-Quelle; Skills, Slash-Commands und Templates delegieren an
  `givn instructions specs`. Die Parity-Suite lief mit 166 Non-E2E- und 24
  E2E-Szenarien grün; Projekt-Overrides und Flag-Varianten sind abgedeckt.
- Change 4 wurde archiviert: Review-/Design-/Steps-Guidance erklärt die
  deterministischen Gate-Dispositionen und die Black-Box-First-Frage; die
  Retrieval-Anweisung bleibt bis Change 5 aus. Post-Merge liefen 169
  Non-E2E- und 25 E2E-Szenarien grün.
- Change 6 läuft im watn-Repo mit einer bereits konsolidierten givn-Version;
  baumweite Altlasten (F1/F2) werden manuell aufgelöst.
  Rückwärtskompatibilität ist explizit nicht im Scope.
- givn ist selbst givn-Konsument: Jede Change 1–5 läuft durch die eigenen
  neuen Gates, sobald diese aktiviert sind.

---

## PR-Workflow — Stacked PRs

Alle Changes laufen als **Stacked PRs** über die offizielle Extension
`gh stack` (`github/gh-stack`, installiert). Ein Layer = eine Change = ein
PR. Jeder PR basiert auf dem Branch des darunterliegenden PR; nur der
unterste PR zielt auf `main`. Reviewer sehen pro PR nur das Diff seines
Layers. Die Sequenz (1 → 2 → 3 → 4 → 5 → 6) bleibt hart — die
Base-Verkettung bildet sie ab.

**Installation / Update**

```bash
gh extension install github/gh-stack    # einmalig
gh extension upgrade gh-stack           # Updates
```

**Stack-Layout (givn-Repo)**

```
workflow-enforcement-wip-cleanup → nächster PR (base: spec-overlap-gates)
spec-overlap-gates               → PR #5 (base: spec-refactoring)  ← Change 1
spec-refactoring                 → PR #1 (base: main)              ← first PR
────────────────────────────────────────────────────────────────────────────
main (trunk)
```

Change 6 (`watn-consolidation`) liegt im watn-Repo und bekommt dort
seinen eigenen Stack (bzw. einen einzelnen PR) mit `main` als Trunk.

**Workflow pro Change**

```bash
# Einmalig, von sauberem main aus. Der erste Stack-Layer ist der
# bereits existierende spec-refactoring-Branch:
gh stack init --base main spec-refactoring spec-overlap-gates

# Nach dem Update von PLAN.md auf spec-refactoring den Stack synchronisieren:
gh stack sync

# Nach Change 2 den nächsten Layer anlegen und abarbeiten:
gh stack add prose-consolidation
# ... givn propose/spec/design/tasks/implement/review/archive ...
gh stack push
gh stack submit                        # nächsten PR erzeugen/verlinken

# Danach: skill-rewrites, spec-retrieval,
# anschließend watn-consolidation im watn-Repo.
```

**Regeln**

- Stack-Branches bleiben **linear** (nur Rebase, nie Merge in die
  Stack-Branches). Commits pro Szenario bleiben atomar — sie liegen im
  Layer ihrer Change.
- `gh stack submit` nach jeder Change, damit der neue PR sofort im Stack
  hängt; vorhandene PRs werden dabei aktualisiert, nicht dupliziert.
- Wird der Trunk (`main`) bewegt oder ein unterer PR gemergt:
  `gh stack sync --prune` (fetch + Kaskaden-Rebase + Push + PR-Sync,
  pruned gemergte Branches). Bei Konflikten: `gh stack rebase`,
  auflösen, `gh stack rebase --continue`.
- Merge: `gh stack merge --yes --squash` merged den ganzen Stack
  all-or-nothing; `gh stack merge <PR-Nr>` merged nur bis zu einem Layer.
  Da die Dependencies hart sind, wird immer bottom-up gemergt.
- Change 5 wird gebaut (Change 1 hat die Paraphrasen-Lücke belegt); der
  Retrieval-Layer bleibt ein eigener PR, damit die Merge-Entscheidung über
  Embeddings separat getroffen werden kann.
- `gh stack view` zeigt Branches, PR-Links und letzten Commit;
  `gh stack view --short` nur Branchnamen.

---

## 1. Change `spec-overlap-gates` — deterministische Überlappungs-Gates

### Proposal (User-Perspektive)

**Problem / Opportunity**

Ein Projekt, das über Wochen mit givn arbeitet, akkumuliert Szenarien, die
dasselbe Verhalten mehrfach abdecken: identische Szenario-Titel in
verschiedenen Dateien, Teilmengen-Szenarien, dieselbe Invariante in mehreren
Feature-Familien. `givn lint` prüft heute nur Parse-Fehler und @wip-Tags —
kein einziger dieser Befunde wird mechanisch sichtbar oder blockiert. Die
Suite bleibt grün, weil Grün nur beweist, dass Bindings registrierbar sind,
nicht dass Szenarien unabhängig sind. Zusätzlich lintet `givn lint`
standardmäßig auch den Archiv-Baum, was Befunde über historische,
nicht-ausgeführte Szenarien erzeugt, die niemand mehr ändern kann. Und wenn
ein Autor eine schwächeres Szenario durch ein stärkeres ersetzt
(removed+added über zwei Delta-Dateien), gibt es keine maschinelle
Empfangsbestätigung, die den Netto-Effekt ausweist.

**Proposed Solution**

- `givn lint` prüft die Szenario-Titel über den **ganzen aktiven Baum**:
  permanente Specs plus die Deltas **aller** aktiven Changes. Ein identischer
  Titel an zwei Stellen ist ein Hard Error, der Dateien und Titel benennt.
  Ausnahme: genau ein Paar "modifiziertes Szenario in einer Change-Delta vs.
  sein Ziel in der permanenten Spec derselben Capability" — sonst wäre jede
  Modifikation ein Fehler. Ein Titel-Rename ist kein modifiziertes Szenario,
  sondern removed+added mit Begründung in review.md.
- Szenarien, die **gleich aussehen** (gleiche Observable-Form: gleiche
  Assertion-Folge nach Normalisierung), aber verschiedene Titel tragen,
  erzeugen eine Warnung und eine **Pflicht-Entscheidung** in review.md
  (Dispositionstabelle): entweder zusammenlegen/löschen oder als eigene
  Boundary begründen. Ohne den Eintrag blockt der Review. Widersprechen sich
  die Facts zweier gleich aussehender Szenarien (Exit-Codes, contains vs.
  not-contains, [DONE], TTY), wird nie eine Zusammenlegung nahegelegt —
  stattdessen ein Hinweis auf das verwandte Szenario.
- Lange Szenarien (> 14 Steps) werden gewarnt; die Dispositionstabelle
  bekommt eine Spalte "split-or-keep", und ein langes Szenario ohne Eintrag
  blockt den Review.
- Der Default-Scope von `givn lint` schließt `givn/archive` nicht mehr ein.
- `givn lint` kann Befunde als JSON ausgeben, damit Werkzeuge und die
  eigenen E2E-Szenarien auf Befunde statt auf Prosa prüfen können.
- Beim Review (oder einem eigenen Kommando) wird ein
  **Net-Delta-Receipt** ausgegeben: wie viele Szenarien die Change addiert,
  modifiziert, entfernt.

**Out of Scope**

- Embedding-/Modell-basierte Ähnlichkeitssuche (Change 4, bedingt).
- Jeder Block auf Ähnlichkeits-Scores: Nur der identische Titel blockt
  hart; alles Weichere warnt und erzwingt eine Disposition. Namensqualität
  (ob zwei überlebende Szenarien besser benannt werden müssen) ist
  Reviewer-Urteil, kein maschineller Check.
- Rückwärtskompatibilität: Altlasten bestehender Projekte (watn F1/F2)
  werden manuell bereinigt (Change 5); der Gate wird nicht grandfathert.
- Parsen fremder Step-Definitionen oder Testcode.
- Längen-Hard-Fail (ein harter Block bei >19 Steps wurde verworfen —
  er setzt den Anreiz, sichtbare Steps in undurchsichtige Composite-Steps
  zu verpacken).
- Runner-Doppelausführung in watn (bewusst verschoben, analysis.md §5).

**Open Questions**

- Dispositions-Volumen auf givns eigenem Baum: 154 Szenarien, Anzahl der
  Shape-Matches ungemessen. Falls die Zahl groß ist, muss die Change
  entscheiden, ob die eigene Baum-Disposition Teil der Change ist oder als
  Folge-Change läuft — der Review der Change selbst blockt sonst.
- Wortlaut der Ausgabe-Kategorien und das JSON-Schema (Warnung/Error/Hinweis,
  Datei, Titel, Befundart) — wird im Design fixiert und per E2E getestet.
- Position des Hard-Errors: `givn lint` (frühester Zeitpunkt) vs. `givn
  check` — Entscheidung im Design, Empfehlung aus dem Review: lint.

### Spec-Anhaltspunkte (Kandidaten-Szenarien)

Feature-Dateien der Change decken ab (über das echte Binary gegen
Fixture-Bäume):

- Identischer Titel in zwei permanenten Spec-Dateien → Hard Error, beide
  Dateien benannt, Exit-Code dokumentiert.
- Identischer Titel: permanente Spec vs. Delta einer **anderen** aktiven
  Change → Hard Error.
- Identischer Titel: Deltas zweier aktiver Changes → Hard Error.
- Modified-Delta mit gleichem Titel wie sein Merge-Target (gleiche
  Capability) → kein Fehler.
- Modified-Delta mit gleichem Titel in anderer Capability → Hard Error.
- Shape-Match mit verschiedenem Titel ohne Disposition-Eintrag in
  review.md → Review blockt; mit Eintrag → Review läuft.
- Shape-Match mit hartem Fakten-Widerspruch → Hinweis `related_scenario`,
  kein Zusammenlegungs-Vorschlag.
- Subset (Then-Folge B ⊂ Then-Folge A) → Warnung "Teilmenge".
- Szenario mit >14 Steps ohne split-or-keep-Eintrag → Review blockt;
  mit Eintrag → Review läuft.
- `givn lint` ohne Argumente schließt `givn/archive` aus (ein archivierter
  Parse-Fehler erzeugt keinen Befund); expliziter Pfad auf eine
  Archiv-Datei lintet weiterhin gezielt.
- `--json` liefert maschinell lesbare Befunde (Befundart, Datei, Titel)
  für alle Gate-Fälle.
- Net-Delta-Receipt weist removed/added/modified der Change aus.
- Bestehende Verhaltens-Verträge bleiben erhalten: Exit-Codes 0/1/2 von
  `givn lint` für die heutigen Fälle (Parse/Wip) ändern sich nicht;
  Normalisierung behandelt `And`/`But`, Quoted Strings, Zahlen, Polarität,
  Scenario Outlines (pro Examples-Zeile), Background-Voranstellung,
  Data Tables (Spaltenkopf als Fact-Key, ohne Kopfzeile Positions-Index),
  Docstrings werden ignoriert; fehlende Facts bedeuten nie einen
  Widerspruch (konservativ).

### Design-Anhaltspunkte (aus analysis.md, für design.md)

- Fingerprint: Normalisierung → Shape-Vergleich (Then-Folgen per LCS,
  Given/When schwächer) → Facts/Widersprüche → Titel-Ähnlichkeit nur als
  Namensqualitäts-Signal (kein Block, kein Schwellwert). Keine Enum-Maske.
- Gate-E2E pro Tier über das echte Binary; Unit-Tests nur für
  Normalisierungs-Edge-Cases, jeder mit Beantwortung der Frage "welchen
  Fall deckt dieser Test, den das E2E nicht deckt?" (Black-Box-first,
  analysis.md §2.7).
- Maschinelle Checks: Shape-Match ohne Disposition → Review blockt;
  langes Szenario ohne split-or-keep → Review blockt. Sonst kein Block.
- `src/hooks.rs` (Built-in-Hook-Registry) ist der etablierte Ort für
  Gate-Hooks — Entscheidung: Hook vs. Lint-Erweiterung, im Design fixieren.

### Evidenz-/Verifikationspflichten

- Fingerprint-Messung auf den 18 gelabelten Dup-/Boundary-Paaren des
  watn-Reports (Recall, kein Schwellwert-Block): Wie viele der 8
  Duplikat-Paare und wie viele Paraphrasen-Fälle mit verschiedenem Titel
  werden gefunden? Das Ergebnis entscheidet über den Bau von Change 4 und
  wird in der Change-Dokumentation festgehalten.
- Der givn-eigene Baum (154 Szenarien) ist nach Abschluss der Change
  dispositions-sauber: jeder Shape-Match ist in der Change selbst
  disposiert oder als Folge-Work-Item dokumentiert.

### Risiken

- Unbekanntes Dispositions-Volumen auf dem eigenen Baum kann die Change
  aufblähen (Planungsrisiko, in Open Questions adressiert).
- watn-Runner-Doppelausführung (analysis.md §5) bleibt bestehen und kann
  während der Gates-Entwicklung Störgeräusche erzeugen — bewusst verschoben.

---

## 2. Change `workflow-enforcement-wip-cleanup` — permanente Workflow-Spec bereinigen

### Proposal (User-Perspektive)

**Problem / Opportunity**

Die permanente Workflow-Enforcement-Spec enthielt 16 Szenarien mit `@wip`.
Ihre Bindings und beobachtbaren Ergebnisse waren bereits grün, aber der Tag
signalisierte weiterhin einen unfertigen Zustand und machte fertige Prüfungen
für Autoren und Reviewer irreführend.

**Proposed Solution**

- Die 16 bestehenden Szenarien werden als fertiggestellt aktiviert.
- Die Szenariotitel, Schritte und beobachtbaren Ergebnisse bleiben unverändert;
  nur die Lifecycle-Markierungen werden über `@givn.modified`-Deltas entfernt.
- Nach dem Archivieren enthält die permanente Spec keine
  Szenario-`@wip`-Tags mehr; wörtliche `@wip`-Erwähnungen in Fixture-Schritten
  bleiben erhalten.
- Der Merge-Randfall, bei dem ein Tag vor dem ersten Szenario im Feature-Header
  hängen bleiben kann, wird durch einen Regressionstest verhindert.

**Out of Scope**

- Neue Workflow-Enforcement-Regeln oder Szenarien.
- Änderungen an vorhandenen Step-Definitionen oder dem Test-Runner, außer dem
  notwendigen Merge-Parser-Fix für den ersten Szenario-Tag.
- Bereinigung von `@wip`-Text in anderen Capabilities oder im Archiv.

**Open Questions**

- Keine. Die 16 Szenarien waren bereits implementiert und konnten einzeln gegen
  den aktiven Delta-Baum verifiziert werden.

### Spec-Anhaltspunkte

- Jedes der 16 vorhandenen Szenarien wird über ein `@givn.modified`-Delta ohne
  Szenario-`@wip` aktiviert.
- Die vollständige Workflow-Enforcement-Spec bleibt grün.
- Die Archivierung entfernt alle Lifecycle-Tags, ohne Fixture-Text zu ändern.
- Der erste permanente Szenariotag wird beim modifizierenden Merge ebenfalls
  entfernt.

### Design-Anhaltspunkte

- Bestehende `cucumber-rs`-Suite und
  `tests/steps/workflow_enforcement_steps.rs` wiederverwenden.
- Strict Mode bleibt über `.fail_on_skipped()` im vorhandenen Runner bewiesen.
- Vor dem Archivieren wird der aktive Delta-Pfad über `GIVN_FEATURES` gezielt
  ausgeführt; danach werden permanente Non-E2E- und E2E-Suiten ausgeführt.
- Der textbasierte Merge entfernt Tags, die nach der `Feature:`-Zeile vor dem
  ersten Szenario stehen, damit ein modifiziertes erstes Szenario keine alten
  Tags erbt.

### Evidenz-/Verifikationspflichten

- `givn lint --change workflow-enforcement-wip-cleanup` ist clean.
- Alle 16 Szenarien werden einzeln und als Delta-Suite ausgeführt.
- Permanente Suite: 163 Szenarien; E2E-Subset: 22 Szenarien; beide grün.
- Nach dem Archivieren gibt es keinen Szenario-`@wip`-Tag in der permanenten
  Workflow-Spec; Fixture-Erwähnungen bleiben bestehen.
- Der Regressionstest für den ersten Szenariotag und `cargo check` sind grün.

---

## 3. Change `prose-consolidation` — eine normative Prosa-Quelle

### Proposal (User-Perspektive)

**Problem / Opportunity**

Dieselben Regeln stehen heute in mehreren Dateien mit abweichenden
Wortlauten: Die @e2e-Regel existiert in 4–6 Formulierungen über 5+ Dateien
(Skill-Templates, Slash-Commands, drei Instruktions-Artefakte), und ein
Template widerspricht sich selbst (Flag-Kombinationen als
Inventory-Einträge vs. genau ein @e2e pro Happy Path). Jede neue Regel aus
der Refactor-Diskussion (Dispositionen, split-or-keep, Black-Box-Policy)
müsste sonst in jeder Quelle separat gepflegt werden — Drift ist damit
vorprogrammiert.

**Proposed Solution**

- `assets/instructions/*.md` ist die **einzige** Quelle für Regeltext.
- Skills und Slash-Commands werden dünne Choreografie, die die
  konsolidierten Instruktionen ausführt — kein eigener Normtext mehr.
- Die @e2e-Norm gilt repository-weit in einem Wortlaut: **eine
  E2E-Evidenzzelle pro realer User-Action**; ein zweites E2E-Szenario für
  dieselbe Action braucht eine neue Schnittstellen-Rechtfertigung. Die
  feature-lokale Formulierung ("pro User Interaction Inventory entry") wird
  ersetzt, weil sie der Mechanismus war, der Setup-, Modellwahl- und
  Filter-Flows pro Capability dupliziert hat.
- Der Projekt-Override-Mechanismus (eigene Skill-/Instruction-Dateien im
  Konsumenten-Projekt) bleibt erhalten und muss mit der konsolidierten
  Quelle zusammenspielen.

**Out of Scope**

- Inhaltliche Neuformulierung von Regeln über die Konsolidierung hinaus.
- Retrieval-/Suchschritte in Skills (Change 4/5).
- Änderungen an den Gates aus Change 1.

**Open Questions**

- Wie Slash-Commands (`assets/commands/*.md`) technisch an die
  Instruktions-Quelle gebunden werden (Rendering vs. Verweis) — Design.
- Ob der Duplikations-Check ("kein Normtext in Templates/Commands")
  maschinell verifiziert werden kann (Lint auf Regel-Snippets) oder eine
  Reviewer-Prüfung bleibt.

### Spec-Anhaltspunkte

- `givn instructions` liefert die konsolidierte @e2e-Regel im normativen
  Wortlaut (ein E2E pro realer User-Action, repository-weit).
- Gerenderte Skills (z.B. givn-spec) enthalten keinen eigenen Regeltext
  mehr, sondern verweisen auf die Instruktionen.
- Der Projekt-Override eines Konsumenten überschreibt weiterhin (und nur)
  die projektspezifischen Teile.
- Die Regel "ein E2E pro realer User-Action" erscheint im gerenderten
  Material nicht mehr in widersprüchlichen Formulierungen (inkl. des
  Flag-Kombinations-Widerspruchs im heutigen Template).

### Design-Anhaltspunkte

- `src/skills/render.rs` (heute: naive String-Substitution mit 4
  Kontextvariablen, Projekt-Override-Cascade) ist die Stelle, die
  umgebaut wird.
- Driftquellen laut Code-Befund: `assets/skills/givn-spec/SKILL.md.tmpl`
  (Zeilen 41 vs. 79-83 widersprechen sich), `assets/instructions/specs.md`,
  `design.md`, `review.md`, `assets/commands/givn-spec.md`.
- Doc-Drift `config.yaml` vs. `commands.yaml` (analysis.md §5) wird hier
  mitbehandelt.

### Evidenz-/Verifikationspflichten

- Repo-weiter Scan: kein Regeltext-Duplikat zwischen Instructions,
  Templates und Commands nach der Konsolidierung.

---

## 4. Change `skill-rewrites` — Skill-Choreografie für die neuen Gates

### Proposal (User-Perspektive)

**Problem / Opportunity**

Die Skills kennen die neuen Gates nicht: givn-review weiß nichts von
Dispositionstabellen, split-or-keep oder removed+added-Paaren; givn-design
und givn-steps kennen die Black-Box-First-Policy nicht. Autoren würden die
Gates als Hindernis erleben statt als Werkzeug, weil die Anleitung fehlt,
wie man ihnen korrekt begegnet.

**Proposed Solution**

- givn-review prüft die Dispositionstabelle: Jeder Shape-Match ist
  klassifiziert (zusammenlegen/löschen oder Boundary-Begründung), jedes
  lange Szenario hat einen split-or-keep-Eintrag, und jedes
  removed+added-Paar trägt eine Supersession-Aussage oder eine explizite
  "nicht zusammengehörig"-Erklärung.
- givn-design und givn-steps tragen die Black-Box-First-Policy: Möglichst
  immer die echte Schnittstelle testen; ein interner Test darf nur Fälle
  abdecken, die das E2E nicht abdeckt — für jeden Unit-Test muss die Frage
  "welchen Fall deckt dieser Test, den das E2E nicht deckt?" beantwortet
  sein, sonst wird er gelöscht. Konsequenz in der Praxis: schwächere
  Duplikat-Szenarien werden gelöscht, nicht umbenannt.
- givn-spec bekommt die Such-Schritte aus Change 5 **nur, wenn** Change 5
  gebaut wird; bis dahin verweist der Skill auf die deterministischen
  Gates.

**Out of Scope**

- Die Gates selbst (Change 1), die Prosa-Quelle (Change 3), die
  Retrieval-Implementierung (Change 5).
- Step-Mechanics-Konsolidierung in watn (Change 6).

**Open Questions**

- Wie präzise die Review-Anweisungen formuliert sein müssen, damit ein
  LLM-Reviewer sie nicht als Checkliste abhakt (Fabrication-Audit als
  Gegenmittel existiert bereits in givn).

### Spec-Anhaltspunkte

- Gerenderte givn-review-Instruktionen enthalten die Dispositions-Prüfung
  (Shape-Match, split-or-keep, removed+added-Supersession).
- Gerenderte givn-design/givn-steps-Instruktionen enthalten die
  Black-Box-First-Frage ("welchen Fall deckt dieser Test, den das E2E
  nicht deckt?").
- Ohne Change 5 enthalten givn-spec-Instruktionen keinen Such-Schritt;
  mit Change 5 enthalten sie ihn (bedingtes Szenario, erst bei Change 5
  relevant).

### Design-Anhaltspunkte

- Alle Änderungen sind Asset-/Template-Änderungen auf Basis der
  konsolidierten Quelle aus Change 3; kein Binary-Code außer ggf.
  Render-Anpassungen.
- Keine gepinnten Embedding-Scores in review-Anweisungen (Review-Protokoll 2,
  analysis.md §2.5): Der Reviewer queried gegen den frischen Index, die
  Disposition trägt Entscheidungen, nicht Zahlen.

---

## 5. Change `spec-retrieval` — semantische Suche beim Authoring (ERLEDIGT / ARCHIVIERT)

> Diese Change wird nur erstellt, wenn die Messung aus Change 1 belegt,
> dass der deterministische Fingerprint Paraphrasen-Fälle (gleiche
> Bedeutung, verschiedene Titel) verpasst. Siehe "Abhängigkeiten und
> Entscheidungspunkte".

> **STATUS: ERLEDIGT / ARCHIVIERT** — umgesetzt auf Stack-Layer
> `spec-retrieval`, archiviert mit Commit `97bf8b3`. Die Messbedingung war durch
> Change 1 erfüllt: deterministische Fingerprints verpassten belegte
> Paraphrasen.

### Proposal (User-Perspektive)

**Problem / Opportunity**

Der Autor schreibt neue Szenarien, ohne ähnliche bestehende Szenarien zu
sehen — insbesondere solche, die dasselbe Verhalten mit anderer Wortwahl
beschreiben (Paraphrasen), die kein deterministischer Vergleich findet.

**Proposed Solution**

- `givn spec index` baut einmalig einen lokalen Suchindex über die
  permanenten Specs (Modell wird beim ersten Lauf geladen, danach lokal
  gecacht).
- `givn spec search` findet die ähnlichsten Szenarien zu einer Anfrage
  (Verhaltens-Query vor dem Schreiben; fertiges Gherkin nach dem
  Schreiben). Treffer werden angezeigt, nie geblockt — die Klassifikation
  (Duplikat/Variante/Boundary) trifft der Autor, der Reviewer verifiziert
  in der Disposition.
- `givn spec review --change <id>` aktualisiert den permanenten Index und meldet
  undisponierte semantische Kandidaten der aktiven Change; `givn check review`
  ruft denselben Lauf advisory-only nach dem deterministischen Gate auf.
- Im Review aktualisiert der Reviewer den Index und queried gegen den
  **aktuellen** Baum: Jeder Treffer ohne Disposition ist ein Befund.
  Gepinnte Scores gibt es nicht — die Disposition trägt Entscheidungen.
- Eigene Deltas der laufenden Change werden nie gegen sich selbst
  gematcht.

**Out of Scope**

- Jede Form von Block auf Ähnlichkeits-Scores.
- Indexierung von Step-Definitionen oder fremdem Testcode.
- CI-Nutzung eines heruntergeladenen Produktionsmodells: Die Gates verwenden
  einen deterministischen lokalen Stub; CI bleibt netzwerk- und modellsetupfrei.
- Treffer gegen offene Deltas anderer Changes (transiente Störung; nur
  hinter einem Flag, Details im Design).

**Open Questions**

- Keine offenen Fragen für die archivierte Change. Fremde aktive Deltas bleiben
  außerhalb des Standardindexes; der Produktionsembedder ist optional,
  feature-gated und lokal cachebar.

### Spec-Anhaltspunkte

- `givn spec index` baut den Index über die permanenten Specs und meldet
  den Umfang.
- `givn spec search` liefert für ein bekanntes Duplikat-Paar dessen
  Partner in den Top-5 (Recall-Metrik auf den 18 gelabelten Paaren,
  k=5, als Validierungsmetrik — nicht als Gate).
- Die Suche matcht ein Szenario nicht gegen die eigenen Deltas.
- Review-Wiederholung gegen den aktualisierten Index: undisponierter
  Treffer → Befund.
- Unveränderte Szenarien werden per content hash wiederverwendet; fehlende
  Modelle, fehlende Changes, unsichere Change-IDs und entfernte Deltas sind
  explizit getestet.

### Design-Anhaltspunkte

- Optionaler In-Binary-Rust-Pfad über `fastembed-rs` mit
  `intfloat/multilingual-e5-large`; kein Shell-out auf das Python-Tool, kein
  externer Dienst. Der Default-/musl-Build bleibt feature-frei.
- Der vollständige Lockfile-Age-Gate wurde für diese Change aufgrund der
  expliziten Nutzerentscheidung per reproduzierbarer Allowlist überschrieben;
  die 30-Tage-Regel bleibt für spätere Releases bestehen.
- Index inkrementell per content_hash; Modell-Download beim ersten Lauf.
- Review-Protokoll 2 (analysis.md §2.5): frischer Index, Vergleich gegen
  den aktuellen Baum, nie gegen die Autoren-Aufzeichnung.

---

## 5a. Change `enforce-spec-similarity-gate` — harte semantische Spec-Eindeutigkeit (IN ARBEIT)

> Diese Change ist der nächste Stack-Layer auf dem aktuellen Retrieval-/Review-
> Stand. Sie darf nicht durch eine vom Agenten veränderbare Konfiguration
> abgeschwächt werden.

### Ziel und unveränderliche Regel

- Die Binärdatei enthält den festen Schwellenwert
  `SIMILARITY_THRESHOLD = 0.95`.
- Es gibt keinen Override über `givn/config.yaml`, CLI-Flags,
  Umgebungsvariablen oder Review-Dispositionen.
- Jeder Szenario-Paarvergleich mit einem Score `>= 0.95` ist ein harter
  Review-Fehler. Eine Ausnahme oberhalb des Schwellenwerts ist nicht zulässig.

### Prüfbereiche

Jedes Review prüft beide Mengen:

1. Alle permanenten Szenarien nach Anwendung des aktuellen Deltas gegeneinander.
   Gleiches Feature und unterschiedliche Feature-Dateien sind gleichermaßen
   gate-relevant; der Report markiert nur den Scope.
2. Jedes hinzugefügte oder geänderte Szenario des aktuellen Deltas gegen die
   permanenten Szenarien vor der Änderung.

Die Projektion berücksichtigt auch `@givn.removed`-Szenarien. Ein echtes
Duplikat kann dadurch nur dann grün werden, wenn das Duplikat im resultierenden
Spec-Baum tatsächlich entfernt ist.

### Verbindliche Remediation

- **MERGE:** Die zwei Szenarien beschreiben dieselbe Invariante oder Variante.
  Das Duplikat wird gelöscht; ein Review-Eintrag allein löst den Fehler nicht.
- **KEEP-AS-BOUNDARY:** Die Szenarien sind fachlich verschieden. Das behaltene
  Szenario wird so umgeschrieben, dass Titel und Schritte die unterscheidende
  Boundary ausdrücklich benennen und der Score unter `0.95` fällt.

Der Agent darf keinen Kandidaten lediglich als `keep`, `boundary` oder
`KEEP-AS-BOUNDARY` markieren und dadurch den Gate-Fehler unterdrücken. Die
Klassifikation ist die Handlungsanweisung; der Gate-Zustand wird ausschließlich
aus dem erneuten Embedding-Vergleich abgeleitet.

### Archiv-Verhalten

- `givn spec review --change <id>` und `givn check review --change <id>` blockieren
  bei jedem verbleibenden Kandidaten.
- Nach einem erfolgreichen Archiv wird der permanente Index inkrementell
  aktualisiert und enthält die neu gemergten Szenarien.
- Ein Modell-/Cache-Fehler beim nachgelagerten Refresh erzeugt eine sichtbare
  Warnung; die bereits bestandenen deterministischen Archiv-Gates werden nicht
  rückwirkend verändert.

### Ausdrücklich nicht erlaubt

- Kein konfigurierbarer Schwellenwert.
- Keine akzeptierte Ausnahme für ein valides, aber weiterhin zu ähnliches
  Szenario.
- Kein globales `allow_similarity`- oder Dispositions-Flag.
- Kein Gate, das nur Titelgleichheit prüft und semantische Paraphrasen ignoriert.

---

## 5b. Change `cross-encoder-reranker` — pairwise semantic review experiment (IN ARBEIT / STACK-LAYER)

> Dieser Stack-Layer basiert auf `38c2f22` (`enforce-spec-similarity-gate`) und
> wird auf einem separaten Branch gegen die harte Embedding-Grenze verglichen.
> Permanente Szenarien werden während des Benchmarks nicht verändert.
>
> Die Implementierung ist im aktuellen Produkt standardmäßig deaktiviert. Sie
> wird erst mit `--features spec-retrieval,cross-encoder-reranker` aktiviert;
> der spätere Stack-Layer entscheidet über eine Freigabe als Gate.

### Hypothese

Der Bi-Encoder findet Kandidaten schnell, bewertet aber zwei Szenarien getrennt.
Ein lokaler multilingualer Cross-Encoder sieht beide Szenariotexte gemeinsam
und kann deshalb echte Duplikate von validen Interface-/Boundary-Varianten
trennen.

### Geplanter Ablauf

- Den bestehenden Embedding-Index als Candidate Generator behalten.
- Pro Szenario deterministisch die Top-5-Embedding-Nachbarn holen.
- Kandidaten mit `fastembed::TextRerank` und dem multilingualen
  `BGERerankerV2M3` in beide Richtungen bewerten.
- Die beiden Raw-Logits mitteln; der Reranker-Score, nicht der alte
  Embedding-Score, entscheidet den Gate-Zustand.
- Branch-Konstante `RERANKER_LOGIT_THRESHOLD = 3.0`; kein Projekt-, CLI-,
  Environment- oder Review-Override.
- `MERGE` löscht echte Duplikate; `KEEP-AS-BOUNDARY` erfordert weiterhin eine
  Umschreibung unter die Reranker-Grenze. Eine Disposition unterdrückt keinen
  Score.

### Messung

Verglichen werden Volltext-Embedding, Kandidatenmenge, Top-K-Recall,
Reranker-Scores, CPU-Laufzeit und manuell klassifizierte Kandidaten. Der Branch
wird nur übernommen, wenn der Reranker die False-Positive-Rate senkt, bekannte
Duplikate weiterhin findet und die lokale Review-Laufzeit vertretbar bleibt.

### Status

- Basis-Commit: `38c2f22`.
- Keine permanenten Szenarien als Teil des Benchmarks geändert.
- Realer Benchmark abgeschlossen: 839 integrierte Kandidaten, 7 Reranker-
  Kandidaten über Raw-Logit `3.0`, `750.67 s`; nach fachlicher Prüfung sind die
  Kandidaten echte Duplikat- oder Same-Invariant-Varianten, die zu mergen oder
  zu parametrisieren sind. Eine unabhängige Boundary wurde in dieser Stichprobe
  nicht bestätigt.
- Der Branch ist damit ein vielversprechender Vergleich, aber noch keine
  freigegebene Ersatzlösung für das Gate: größere gelabelte Stichprobe,
  Top-K-Recall und CPU-Laufzeit müssen vor dem finalen Merge bestätigt werden.

---

## 6. Change `watn-consolidation` — Dogfood der Gates im Referenzprojekt

> Diese Change wird im watn-Repo (`~/projects/watn`) angelegt, nachdem
> givn mit Change 1 (mindestens) konsolidiert dort eingesetzt wird.

### Proposal (User-Perspektive)

**Problem / Opportunity**

watn hat über Wochen eine Spec-Basis akkumuliert, die strukturell
redundant ist: zwei exakte Titel-Duplikate, drei Subset-Szenarien,
wiederholte Abdeckung derselben Invarianten über bis zu sieben
Feature-Familien, duplizierte Step-Mechanics über Module, 23 lange
Szenarien. Die Suite war trotzdem grün — Grün bewies nur, dass globale
Bindings registrierbar sind, nicht dass die Szenarien unabhängig sind.
Mit den neuen Gates aus Change 1 werden diese Befunde erstmals mechanisch
sichtbar und teilweise blockierend.

**Proposed Solution**

- Die existierenden Titel-Duplikate (F1/F2) und Subset-Szenarien werden
  manuell aufgelöst (Rückwärtskompatibilität ist nicht im Scope).
- Die Konsolidierung folgt der Recommended Consolidation Order des
  Reports, korrigiert um die Black-Box-First-Policy: Schwächere
  Szenarien, deren Verhalten ein E2E bereits deckt, werden **gelöscht**,
  nicht umbenannt (betrifft u.a. F1's Nicht-E2E-Szenario und F6).
- Jeder Shape-Match, den die Gates melden, wird in der Disposition
  klassifiziert; lange Szenarien bekommen split-or-keep-Einträge.
- Die Step-Duplikation wird nach dem Delegationsmodell behandelt:
  Szenario-getriebene Aliase verschwinden mit der Konsolidierung;
  Mechanics-Duplikation (Page-Poller, PTY-Treiber) bleibt, weil ihre
  Szenarien gelayered sind, und wird separat als Helper-Refactor
  behandelt.
- Die Suite bleibt durchgehend grün; jeder Schritt ist eine eigene
  Change mit eigener Evidenz.

**Out of Scope**

- givn-Engine-Änderungen (die sind Changes 1–5).
- Neues Verhalten in watn — nur Konsolidierung bestehender Specs.
- Runner-Doppelausführung (analysis.md §5, verschoben).

**Open Questions**

- Reihenfolge der Konsolidierungsschritte: Der Report empfiehlt eine
  Ordnung; die erste watn-Change entscheidet, ob ihr gefolgt wird oder
  ob die Gates-Ergebnisse eine andere Ordnung erzwingen (z.B. erst die
  blockierenden Titel-Duplikate).

### Spec-Anhaltspunkte

- Jede Konsolidierungs-Change in watn trägt ihre eigenen Deltas
  (removed/added) mit Disposition für jeden Shape-Match; Zielzustand:
  keine Titel-Duplikate, keine Subset-Szenarien, alle Gates grün.
- Die Szenario-Löschungen werden durch die Black-Box-Frage gerechtfertigt
  ("welchen Fall deckt das E2E, den dieses Szenario zusätzlich deckt?" —
  ohne Antwort wird gelöscht).

### Evidenz-/Verifikationspflichten

- `givn lint` (mit den neuen Gates) ist nach Abschluss grün; die
  Dispositionstabelle ist vollständig; die Suite (Cucumber) bleibt grün.

---
