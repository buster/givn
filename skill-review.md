# Skill- und Instruction-Review

**Status:** FIXES APPLIED; REVIEWED

**Fix-Change:** `fix-skill-instruction-integrity`

**Pruefstand:** Branch `semantic/04-semantic-remediation`, Commit `e99bc2d`

**Scope:** Die Refactorings aus PR #8 (`prose consolidation`), PR #9
(`skill rewrites`), die generierten Integrationen und der aktuelle Tree bis
PR #19. Geprueft wurden die Source-Assets unter `assets/`, die generierten
Dateien unter `.opencode/`, `.agents/` und `.claude/`, die Semantic-Review-
und Remediation-Erweiterungen sowie die Diffs gegen die jeweiligen
Vorversionen.

Die urspruengliche Zentralisierungsfrage und spaetere aktuelle Policy-Luecken
werden getrennt bewertet. Ein spaeterer Fehler ist nicht automatisch eine
Regression von PR #8 oder PR #9.

## Ergebnis

Die folgenden Findings beschreiben den Zustand vor der Umsetzung. Die
Korrekturen sind im Change `fix-skill-instruction-integrity` spezifiziert,
implementiert und mit dem Change- sowie Repository-Testlauf verifiziert.

Das Refactoring hat die Regeln nicht pauschal geloescht. Die folgenden Inhalte
sind weiterhin vorhanden und wurden teilweise sinnvoll vereinheitlicht:

- Observable Gherkin-Specs ohne Implementierungsdetails.
- User-Interaction-Inventory und seine Ablage als Kommentarblock.
- Genau ein E2E-Szenario pro normalisierter Benutzeraktion.
- Varianten, Fehlerfaelle und output-aendernde CLI-Flags als regulare Szenarien.
- Getrennte E2E-Step-Definitionen und `verify.e2e_command`.
- Deterministische Overlap-Dispositionen.
- Black-Box-First fuer behaltene interne Tests.
- Verhalten bei nicht verfuegbarer Retrieval-Infrastruktur.
- Semantic-Review-Klassifikationen und ihre Begruendungen.

Vor der Umsetzung gab es substanzielle Delegationsluecken, aktuelle
Policy-Drift und einen ungeklaerten Resolververtrag:

- **S1, hoch:** Die als kanonisch deklarierte `specs`-Instruction enthaelt
  nicht die vollstaendigen Regeln fuer primaere Assertions am echten Interface,
  echte Browser-Steuerung und das Verbot von HTTP/`fetch()`-Abkuerzungen.
- **S2, mittel:** Die `design`-Instruction hat design-spezifische E2E-
  Infrastrukturvorgaben verloren. Diese stehen noch in Command- und Output-
  Templates, aber nicht mehr vollstaendig in `givn instructions design`.
- **S3, niedrig bis mittel:** Die Anweisung, Retrieval bis zu seiner Einfuehrung
  zu vermeiden, ist nach dem archivierten `spec-retrieval`-Change veraltet.
- **S4, hoch:** Die seit PR #19 erforderliche class-spezifische Remediation-
  Verifikation ist in der generischen Review-Instruction und im Review-Template
  nicht als eigener Nachweis beschrieben.
- **S5, mittel, vorbestehend:** Die aktuellen Instructions enthalten
  widerspruechliche Aussagen darueber, ob Datenbankzustand oder ein
  cookie-aware HTTP-Client eine zulaessige E2E-Grenze fuer eine Browser-UI
  darstellen.
- **S6, mittel:** Die effektive Policy kann aus Embedded-Assets, Projekt-
  Overrides, installiertem Binary und gerenderten Snapshots stammen. Der
  Report muss diese Freshness- und Aufloesungsgrenzen explizit unterscheiden.
- **S7, niedrig, Produktvertrag:** Commands weisen Agenten an, Metadaten aus
  `givn instructions` zu lesen, die der aktuelle CLI-Output nicht ausgibt.

Die Regeln waren nicht insgesamt verschwunden, aber die behauptete einzige
normative Quelle war unvollstaendig. Dieser Zustand ist durch den genannten
Change behoben; die detaillierte Vorher-Bewertung bleibt unten als Audit-Spur
erhalten.

## Relevante Commits

Nur Commits mit Aenderungen an Skill-, Command-, Instruction- oder den
zugehoerigen Template-Assets sind aufgefuehrt. Reine Gherkin-
Evidenz-/Task-Commits aus denselben PRs sind nicht wiederholt.

| Commit | PR | Geaenderte Quelle | Aenderung und Zweck | Bewertung |
|---|---|---|---|---|
| `ea1c252` | #8 | `assets/instructions/specs.md` | Fuehrt die zentrale E2E- und Inventory-Policy ein. Normalisiert CLI-Flags als Varianten, sofern sie keine neue Aktion oder Side Effect erzeugen, und verlangt genau ein E2E-Szenario pro normalisierter Aktion. | Die Normalisierung ist sachlich begruendet und reduziert E2E-Kombinatorik. Sie ist eine echte Policy-Aenderung, kein reines Umformatieren. |
| `9ab7910` | #8 | `assets/commands/`, `assets/instructions/`, `assets/skills/`, `assets/templates/` | Entfernt wiederholte E2E-Policy aus Commands, Instructions und Skill-Templates. Die Eintraege verweisen nun auf `givn instructions specs --change <id>` und behalten nur Artefakt-Choreografie. | Ziel ist richtig: weniger widerspruechliche Kopien. Die Delegation wurde jedoch auf eine nicht vollstaendige kanonische Quelle geschaltet. Siehe S1 und S2. |
| `dde897d` | #9 | Review-Command, Review-Instruction, Review-Skill | Fuehrt Overlap-Dispositionen, `split-or-keep` und die Erklaerung von Removed+Added-Paaren als Review-Evidenz ein. | Additive Verschaerfung. Keine Regel verloren. |
| `01b2a25` | #9 | Design-/Steps-Command, Design-Instruction, Design-/Steps-Skill | Fuehrt Black-Box-First ein: Interne Tests bleiben nur mit einer konkreten Begruendung, welchen Fall sie abdecken, den E2E nicht abdeckt. | Additiv und gerechtfertigt. Der Geltungsbereich ist bewusst Design und Step-Schreiben, nicht die gesamte Specs-Policy. |
| `75b0216` | #9 | Specs-Command, Specs-Instruction, Specs-Skill | Verhindert Retrieval-Schritte, solange Retrieval nicht verfuegbar ist, und verweist auf deterministische Lint-/Review-Gates. | Zum damaligen Zeitpunkt gerechtfertigt. Die Bedingung ist inzwischen veraltet, weil `spec-retrieval` archiviert und `givn spec index` implementiert ist. Siehe S3. |
| `43fc90b` | #18, erster Commit | `.opencode/`, `.agents/`, `.claude/`, `AGENTS.md`, `README.md` | Rendert die geaenderten Skill- und Command-Templates in die konfigurierten Agent-Ziele und dokumentiert Self-Dogfooding. | Korrekte Verteilung der Source-Aenderungen. Kein eigener Policy-Inhalt und keine zusaetzliche Loeschung in `assets/`. |
| `9fc9fe1` | #18 | `assets/instructions/specs.md`, `assets/instructions/review.md`, `.opencode/commands/givn-review.md`, weitere Projektdokumentation | Fuegt Semantic-Review-Planung, Blocking-/Advisory-Kontext und die entsprechende Guidance in den aktuellen Tree ein. | Relevanter aktueller Policy-Zustand, aber nicht Teil der urspruenglichen Skill-Zentralisierung. |
| `dc7cfbd` | #18 | `assets/instructions/specs.md`, `assets/instructions/review.md`, `assets/commands/givn-review.md`, Review-Evidenz | Ergaenzt Klassifikations- und Review-Evidenz fuer den Semantic-Review-Change. | Erhaltene Klassifikationsregeln; die spaetere Remediation-Contract-Luecke bleibt offen. |

PR #10 (`spec retrieval`) enthaelt nach dieser Historienpruefung keine
zusammenhaengende Aenderung an den Skill- oder Instruction-Source-Assets. Es
liefert aber den Grund, warum S3 heute relevant ist. PR #18 und PR #19 werden
hier als aktueller Kontext und nicht als Ursache der urspruenglichen
Zentralisierungsluecke betrachtet.

## Regelabgleich

| Regel | Aktueller Nachweis | Bewertung |
|---|---|---|
| Specs beschreiben beobachtbares Verhalten und keine Klassen, Routen oder Step-Mechanik. | `assets/instructions/specs.md:7-19`, `.opencode/skills/givn-spec/SKILL.md:30-35` | Erhalten. |
| Inventory deckt die relevante Black-Box-Schnittstelle ab und steht vor dem Feature. | `assets/instructions/specs.md:39-59`, `assets/templates/spec.feature:1-15` | Erhalten. |
| Eine normalisierte Benutzeraktion hat genau ein E2E-Evidenzszenario. Varianten und Fehlerfaelle sind keine weiteren E2E-Aktionen. | `assets/instructions/specs.md:61-65,116-124`, `assets/templates/spec.feature:2-7,27-28` | Erhalten und durch `ea1c252` bewusst praezisiert. |
| E2E nutzt die reale externe Grenze, einen eigenen Runner und getrennte Step-Definitionen. | `assets/instructions/specs.md:94-137` | Im Kern erhalten. Die konkrete Browser-/Assertionsemantik fehlt dort jedoch. |
| Primaere Assertion ist die Antwort des echten Interfaces; Repository-/DB-Pruefungen sind nur zusaetzlich. | Alte Fassung entfernt durch `9ab7910`; aktuelle explizite Kopien nur in `assets/instructions/review.md:32-44,188-194,226-230` und `assets/templates/design.md:134-137`. | **Nicht in der kanonischen Specs-Instruction erhalten. S1.** |
| Browser-UI wird durch einen echten Browser gesteuert; Backend-HTTP, `fetch()` oder `XMLHttpRequest` ersetzen keine Browserinteraktion. | Noch vorhanden in `assets/templates/design.md:139-144`, `assets/templates/tasks.md:70-77`, `assets/commands/givn-review.md:83-87` und Review-Skills. | Global nicht geloescht, aber aus der behaupteten kanonischen Quelle entfernt. **S1.** |
| Interface-Hindernisse sind Design-Luecken und duerfen nicht durch schwache Assertions umgangen werden. | `assets/instructions/design.md:91-128`, `assets/templates/design.md:97-105`, Review- und Implement-Commands. | Erhalten in Design/Review, nicht als vollstaendige Specs-Policy. **S1.** |
| Design dokumentiert E2E-Runner, Step-Ort, lokale Infrastruktur, Framework und Matrix. | `assets/templates/design.md:108-147`, `assets/commands/givn-design.md:83-86`; aktuelle `assets/instructions/design.md:76-82,130-149` enthaelt nur einen Teil. | Inhalt im Output-Template erhalten, Instructions-Quelle unvollstaendig. **S2.** |
| Review verlangt Overlap-Dispositionen und keine Score-only-Entscheidung. | `assets/instructions/review.md:196-212`, `assets/skills/givn-review/SKILL.md:140-146`, `assets/commands/givn-review.md:142-155` | Erhalten und erweitert. |
| Interne Tests muessen einen eigenen, nicht durch E2E abgedeckten Fall begruenden. | `assets/instructions/design.md:84-89`, `assets/skills/givn-design/SKILL.md:91-95`, `assets/skills/givn-steps/SKILL.md:73-77` | Erhalten und additiv eingefuehrt. |
| Retrieval ist vor seiner Verfuegbarkeit nicht zu verwenden; fehlende Retrieval-Infrastruktur darf nicht stillschweigend passieren. | `assets/instructions/specs.md:67-83`, `.opencode/skills/givn-spec/SKILL.md:50-54` | Erhalten, aber der vorgeschaltete Verbotsblock ist nach PR #10 veraltet. **S3.** |
| Ein erfolgreich klassifizierter Semantic-Review-Kandidat muss seine class-spezifische Remediation im projizierten Tree verifizieren. | `src/spec/retrieval.rs:347-383`, `givn/archive/semantic-remediation/design.md:20-50,90-101`, `docs/arc42/08-crosscutting-concepts.md:475-485` | Im Code und in Change-Artefakten implementiert, aber in generischer Review-Instruction und Review-Template nicht vollstaendig operationalisiert. **S4.** |

## S1: Kanonische Specs-Policy unvollstaendig

### Beobachtung

`assets/skills/givn-spec/SKILL.md.tmpl:45-48` behauptet, die aufgeloeste
`specs`-Instruction sei die normative Quelle fuer "real-interface assertions"
und "driver-fidelity". `assets/skills/givn-implement/SKILL.md.tmpl:114-116`
und `assets/instructions/design.md:76-82` machen dieselbe Zusage.

Die aufgeloeste Quelle `assets/instructions/specs.md:94-137` sagt jedoch nur,
dass E2E an der realen externen Grenze arbeitet, Infrastruktur verwenden darf
und getrennte Step-Definitionen hat. Sie schreibt nicht ausdruecklich vor:

- dass `Then` primaer Interface-Ausgaben pruefen muss;
- dass Repository-/Datenbankassertions nie der alleinige Nachweis sein duerfen;
- dass eine Browser-UI mit einem echten Browser geklickt, befuellt und
  navigiert werden muss;
- dass HTTP-Requests sowie `fetch()`/`XMLHttpRequest` bei einer Browser-UI keine
  gleichwertige Abkuerzung sind;
- dass ein Interface-Hindernis zuerst als Design-Luecke behoben werden muss,
  statt die Assertion auf Repository-Zustand abzuschwaechen.

Diese Passagen wurden durch `9ab7910` aus mehreren Skills, Commands und
Instructions entfernt. Sie sind in Output-Templates und Review-Anweisungen
noch auffindbar. Das ist deshalb kein vollstaendiger globaler Regelverlust,
aber ein Verlust genau an der Stelle, die laut der kanonischen Specs-Instruction die vollstaendige
normative Policy sein soll.

Die historische Ursache liegt frueher: `6d3eaec` fuehrte die primaeren
Real-Interface-Assertions, lokale Runnability, Digital Twins und die
Repository-only-Downgrade-Regel ein; `d9f78f4` ergaenzte echte Browser-
Fidelity, das HTTP/`fetch()`-Verbot und den normalisierten E2E-Scope;
`1e561ff` verankerte Inventory-/Matrix- und Hook-Pruefungen. `ea1c252`
deklarierte anschliessend die kanonische Specs-Instruction und normalisierte
CLI-Flags, migrierte aber nicht alle diese Klauseln dorthin. `9ab7910`
entfernte dann die verteilten Kopien. Der Defekt ist daher eine unvollstaendige
Migration waehrend der Zentralisierung, nicht das Entfernen dieser Klauseln
aus `specs.md` durch `9ab7910`.

Die archivierte Prose-Consolidation-Spec pruefte Routing, Flag-Varianten,
generierte Guidance und fehlende Legacy-Wortlaute. Sie pruefte nicht als
Klausel-fuer-Klausel-Ledger, dass jede entfernte normative Passage in der
aufgeloesten Specs-Instruction wieder vorhanden ist. Die Aussage "lossless"
kann aus diesen Tests allein deshalb nicht abgeleitet werden.

### Risiko

Ein Agent, der `/givn-spec`, `givn-steps` oder `givn-implement` verwendet,
liest die kanonische Instruction auf Aufforderung. Er bekommt dort fuer die
entscheidenden Assertions- und Driver-Regeln nur eine Referenz ohne die
normativen Details. Die spaeteren Kopien in Review- und Output-Templates
werden waehrend der Spec- und Implementierungsphase nicht automatisch gelesen.
Damit ist die Durchsetzung vom jeweiligen Agentenverhalten abhaengig und nicht
von der behaupteten zentralen Instruction.

### Bewertung

**Nicht gerechtfertigt als abgeschlossene Zentralisierung.** Die Absicht, die
Duplikation zu entfernen, ist gerechtfertigt. Die kanonische Datei muss dann
aber genau die delegierten Regeln vollstaendig enthalten. Die kanonische
Specs-Instruction beschreibt selbst eine "complete policy"; der aktuelle Stand
entspricht diesem Architekturentscheid nicht.

## S2: Design-Instruction verlor design-spezifische E2E-Vorgaben

### Beobachtung

Vor `9ab7910` enthielt `assets/instructions/design.md` einen eigenen Abschnitt

- exakter E2E-Runner und Filter fuer `@e2e`;
- separate Step-Definitionen pro Capability;
- lokale Server-, Browser-, Datenbank- und Fake-Infrastruktur;
- Auswahl und Begruendung des E2E-Frameworks;
- separater Strict-Mode-Nachweis fuer den E2E-Runner.

Der Abschnitt wurde durch einen Verweis auf `givn instructions specs` ersetzt.
Diese Punkte sind aber keine Specs-Policy. Sie sind HOW-Entscheidungen und
muessen in einem Design beschrieben werden. Teile stehen weiterhin in
`assets/commands/givn-design.md:83-86` und fast vollstaendig in
`assets/templates/design.md:108-147`.

### Bewertung

**Nicht vollstaendig gerechtfertigt.** Es gibt keinen globalen Verlust im
generierten `design.md`, weil das Output-Template die Felder noch erzeugt.
Die Instruction, die ein Agent beim direkten Aufruf von
`givn instructions design` bekommt, ist jedoch nicht mehr allein ausreichend.
Das verletzt die Trennung aus der kanonischen Authoring-Policy in die andere Richtung: Artefakt-
spezifische Choreografie darf nicht aus der Instruction verschwinden, nur weil
normative Specs-Policy zentralisiert wird.

## S3a: Stale Retrieval-Choreografie

`75b0216` fuegte den korrekten Zwischenzustand ein: Solange Retrieval nicht
existiert, sollen keine `givn spec index`- oder Semantic-Search-Schritte in
die Specs-Anleitung gelangen. Seit dem archivierten `spec-retrieval`-Change
existieren `givn spec index`, `givn spec search` und die Review-Integration
jedoch (`src/main.rs:174,255`, `src/spec/retrieval.rs:330-383`).

Die Formulierung in `assets/commands/givn-spec.md:94-95`,
`assets/skills/givn-spec/SKILL.md.tmpl:50-54` und allen drei generierten
Skill-Mirrors ist deshalb nur noch fuer die Feature-freie Variante korrekt.
`Cargo.toml:14-18` unterscheidet `spec-retrieval`,
`spec-similarity-gate` und den Cross-Encoder. `AGENTS.md:15-19` installiert
den lokalen Binary dagegen bereits mit `--features spec-retrieval`. Die
Guidance muss deshalb build-aware sein: Feature-freie Builds sollen die
explizite Unavailable-Meldung behalten, Retrieval-faehige Builds sollen den
entsprechenden Workflow nicht verbieten.

Das ist kein Verlust einer alten Regel, sondern fehlende Pflege nach Abschluss
des Changes.

## S3b: Retrieval-Authoring-Guidance unvollstaendig

`assets/instructions/specs.md:67-83` dokumentiert bereits den positiven
Blocking-Review-Pfad: `givn check review --change <id>`,
`semantic-review.md`, Klassifikationen und das Verhalten bei nicht verfuegbarer
Infrastruktur. Die fruehere Aussage, es gebe gar keine positive Retrieval-
Instruction, war deshalb zu weitgehend.

Es fehlt trotzdem die Authoring-Entscheidung, wann ein Autor
`givn spec index`, `givn spec search` oder `givn spec explore` vor dem
Schreiben neuer Szenarien verwenden soll. Ebenso sollte die Instruction die
Abgrenzung zwischen advisory `givn spec review` und dem feature-abhaengigen
blocking `givn check review` ausdruecklich dokumentieren. Das ist besonders
relevant, weil `spec-similarity-gate` den Review-Pfad erneut verschaerft.
Die aktuelle Architektur-Dokumentation bestaetigt diese Feature-Grenze in
`docs/arc42/08-crosscutting-concepts.md:465-467`.

## S4: Remediation-Verifikation fehlt in der generischen Review-Schicht

### Beobachtung

PR #18 beschreibt die Semantic-Review-Klassifikation. PR #19 erweitert den
blocking Path: Eine Klassifikation ist nur noch eine Arbeitsliste, kein
Nachweis, dass die notwendige Spec-Aenderung umgesetzt wurde. Der aktuelle
Code prueft den vorherigen Report gegen den projizierten permanenten Tree und
recomputet auch verschwundene oder unter die E5-Grenze gefallene Paare:

- `DUPLIKAT`: redundanter Endpoint muss verschwinden, der retained Endpoint
  bleibt erhalten;
- `VARIANTE`: ein parameterisiertes `Scenario Outline` mit nicht leerer
  `Examples`-Tabelle muss die Variante abdecken und der recomputete Score muss
  strikt unter `0.9500` liegen;
- `VALID-BOUNDARY`: beide Endpoints bleiben, die beobachtbare Evidenz muss sich
  unterscheiden und der Score muss strikt unter `0.9500` liegen;
- veraenderte, stale oder nicht matchbare Vor-Eintraege bleiben blockierend.

Nachweis: `src/spec/retrieval.rs:347-383`,
`givn/archive/semantic-remediation/design.md:20-50,90-101` und
`docs/arc42/08-crosscutting-concepts.md:475-485`.

Die generische `assets/instructions/review.md:204-212` beschreibt dagegen nur
Klassifikation und Begruendung. `assets/commands/givn-review.md:148-155` tut
dasselbe. `assets/templates/review.md:186-221` hat keine Semantic-Review-
oder Remediation-Evidenzsektion. Die archivierten Semantic-Review- und
Semantic-Remediation-Reviews enthalten solche Tabellen nur als
change-spezifische Artefakte.

### Bewertung

**Hohe aktuelle Luecke, aber keine Regression von PR #8 oder PR #9.** Die
Implementation ist strenger als die generische Review-Anleitung. Ein Agent
kann die Klassifikation korrekt abschliessen, ohne im allgemeinen Review-
Template einen Remediation-Status, Failure-Evidence und die
`semantic remediation verified`-Receipt zu dokumentieren. Die generische
Review-Schicht sollte Klassifikation und Remediation explizit trennen.

## S5: Vorbestehende widerspruechliche E2E-Grenzen

Diese Luecke ist keine Regression der Zentralisierungs-Commits, muss aber in
den Instruction-Review aufgenommen werden, weil sie die kanonische Policy
direkt widerspricht:

- `assets/instructions/specs.md:108-110` erlaubt eine echte
  Datenbankverbindung als moegliche E2E-Grenze.
- `assets/instructions/specs.md:129-132` verwendet das Erscheinen eines
  Datensatzes in der Datenbank als E2E-Beispiel.
- `assets/instructions/review.md:32-43` stuft Repository-/Datenbank-only-
  Assertions als downgraded ein.
- `assets/instructions/design.md:118-128` nennt einen cookie-aware
  HTTP-Client als moegliche Loesung fuer ein Browser-Session-Problem.
- `assets/instructions/design.md:140-146` und
  `assets/templates/design.md:139-144` verbieten HTTP als Driving Mechanism
  fuer eine Browser-UI.

Die konsistente Regel sollte lauten: Datenbankzustand ist nur dann die primaere
E2E-Grenze, wenn die Datenbank tatsaechlich die oeffentliche Consumer-
Schnittstelle des Systems ist. Bei Browser, CLI, API, Library oder Message Bus
ist er sekundare Evidenz. Fuer Browser-UI-Hindernisse muss ein
Browser-Kontext-/Session-Fix beschrieben werden; HTTP-Clients sind nur fuer
API-only Capabilities zulaessig.

## S6: Effektive Aufloesung, Policy-Schichten und Freshness

### Effektive Policy-Aufloesung

Die verwendete Instruction ist nicht zwingend die Datei im Checkout:

- Embedded Assets werden beim Build in den Binary eingebettet
  (`src/config/embedded.rs:3-6`).
- `givn/artifacts/<artifact>/instructions.md` ersetzt das Embedded-Asset
  (`src/config/cascade.rs:48-82`).
- Skill-Templates koennen ueber `givn/skills/<name>/SKILL.md.tmpl` ueberschrieben
  werden (`src/skills/render.rs:20-26`).
- Commands koennen ueber `givn/commands/<name>.md` ueberschrieben werden
  (`src/skills/targets.rs:193-200`).
- `.opencode/.agents/.claude` sind gerenderte Snapshots, die erst durch
  `givn skills sync` aktualisiert werden. Slash-Commands werden dabei nur fuer
  das `.opencode`-Target geschrieben (`src/skills/targets.rs:131-145`).

Im aktuellen Checkout existieren keine relevanten Specs-, Skill- oder
Command-Overrides; `givn/artifacts/` enthaelt nur das leere `arc42-docs`-
Verzeichnis. Fuer ein konsumierendes Projekt gilt aber: Ein Override ersetzt
die effektive Policy, es erweitert sie nicht automatisch. Deshalb beweist
Paritaet der generierten Skills nicht Paritaet der aufgeloesten Instruction.
Der aktuelle Report sollte explizit zwischen Embedded-Default,
Projekt-Override, installiertem Binary und gerendertem Snapshot unterscheiden.

`skill-review.md:416-419` beschreibt die Aufloesung bisher als "zur Laufzeit
durch `givn instructions`". Praeziser ist: zur Laufzeit aus dem Embedded-Asset
des verwendeten Binary oder aus einem Projekt-Override.

### Policy versus Choreografie

Die Review muss folgende Schichten auseinanderhalten:

| Schicht | Rolle | Darf neue E2E-Semantik definieren? |
|---|---|---|
| Effektive `specs`-Instruction | Normative Interaction- und E2E-Policy | Ja |
| Instructions anderer Artefakte | Artefakt-spezifische Anforderungen und Audit-Verfahren | Nur explizit abgeleitete Pruefungen, keine konkurrierende Policy |
| Skills und Slash-Commands | Choreografie und Ausfuehrungsmechanik | Nein |
| Artifact-Templates | Pflichtfelder, Tabellen und Evidenzstruktur | Nur abgeleitete Checks, klar als solche markiert |
| Generierte Targets | Gerenderte Snapshots | Keine eigene Autoritaet |

Dass eine Regel in `assets/templates/design.md`, `assets/templates/tasks.md`
oder `assets/templates/review.md` noch vorkommt, beweist deshalb nicht, dass
die kanonische Instruction vollstaendig ist. Diese Kopien muessen entweder als
bewusst abgeleitete Evidence-Checks markiert oder in die Konsolidierungsliste
aufgenommen werden.

Das generische `assets/templates/review.md` benoetigt ausserdem eine eigene
Semantic-Review-/Remediation-Sektion. Die aktuellen Tabellen pruefen E2E-
Fidelity und Overlap, aber nicht `DUPLIKAT`, `VARIANTE`, `VALID-BOUNDARY`,
Remediation-Status und die Verification-Receipt.

## S7: `givn instructions`-Resolververtrag ist nicht deckungsgleich

Dies ist ein Produkt-Follow-up und keine Regression der genannten
Zentralisierungs-Commits. Die Commands weisen Agenten an, aus
`givn instructions` Metadaten wie `output`, `template`, `context`, `rules` und
`dependencies` zu lesen (`assets/commands/givn-spec.md:31-39`).

`src/cli/instructions.rs:57-67` gibt fuer Textausgabe jedoch nur die
aufgeloeste Instruction aus. JSON enthaelt nur `artifact` und `instruction`
(`src/cli/instructions.rs:59-64`). Der explizite `--change`-Wert wird in
`src/cli/instructions.rs:33-46` fuer die Artefakt-Ausgabe nicht wirklich
verwendet; die Instruction selbst ist global und nicht change-spezifisch.

Die Review sollte daher nicht behaupten, `givn instructions` liefere bereits
den von den Commands beschriebenen vollstaendigen Resolververtrag. Entweder
die Command-Erwartung ist zu reichhaltig oder der CLI-Vertrag ist unvollstaendig.

## Was bewusst geaendert wurde

### CLI-Flag-Normalisierung

`ea1c252` ersetzt "jede output-aendernde Flag-Kombination ist eine eigene
Inventory-Aktion" durch "jede distinct user action; output-aendernde Flags
sind Varianten, ausser sie aendern Aktion oder Side Effect". Das ist eine
bewusste fachliche Entscheidung gegen eine kombinatorische E2E-Matrix. Sie ist
gerechtfertigt, weil die alte Inventory-Formulierung mit der bereits geltenden
Regel "nicht ein E2E-Szenario pro Variante" kollidierte.

Der verbleibende Trade-off muss akzeptiert werden: Output-spezifische Flags
werden nicht mehr automatisch durch ein eigenes E2E-Szenario abgedeckt. Sie
muessen als regulare Szenarien weiterhin observable geprueft werden.

### Entfernen des E2E-Beispiels aus `propose`

`9ab7910` entfernt das E2E-Beispiel aus `assets/commands/givn-propose.md`.
Das ist gerechtfertigt. Proposal beschreibt WHAT/WHY; konkrete Interaction-
Inventory- und E2E-Szenarien gehoeren in die Specs-Phase. Die Propose-
Choreografie verweist weiterhin auf die Specs-Instruction und die Design-
Choreografie.

### Review-Dispositionen

`dde897d` fuegt keine bisherige Regel weg. Es macht eine zuvor menschlich
implizite Entscheidung explizit und maschinenpruefbar: Shape-Matches,
lange Szenarien und Removed+Added-Paare muessen disponiert werden.

### Black-Box-First

`01b2a25` fuegt eine neue Begruendungspflicht hinzu, statt eine bestehende
Regel zu ersetzen. Sie ist in Design und Step-Schreiben korrekt verankert.

## Generierte Integrationen

`43fc90b` war der erste Commit von PR #18 und synchronisierte die
Template-Aenderungen nach `.opencode/`, `.agents/` und `.claude/`. Die
generierten Skill-Dateien verweisen tatsaechlich auf
`givn instructions specs --change <change-id>`; zum Beispiel:

- `.opencode/skills/givn-spec/SKILL.md:37-48`;
- `.opencode/skills/givn-design/SKILL.md:84-95`;
- `.opencode/skills/givn-review/SKILL.md:36-72`.

Es gibt keinen Hinweis, dass die Synchronisierung weitere Regeln aus den
Source-Assets entfernt hat. Sie verteilt den bestehenden, teilweise
unvollstaendigen Stand korrekt. `9fc9fe1` und `dc7cfbd` aktualisierten danach
die Review-/Semantic-Review-Guidance, ohne die generischen Skill-Mirrors erneut
zu erzeugen. Slash-Commands werden nur fuer `.opencode` gerendert; `.agents`
und `.claude` erhalten Skills, aber keine eigenen Command-Dateien.

Die Instruction-Dateien werden nicht in die Skills gerendert, sondern aus dem
Embedded-Asset oder einem Projekt-Override zur Laufzeit durch
`givn instructions` aufgeloest. Deshalb ist die Luecke in
`assets/instructions/specs.md` operativ relevant.

## Weitere vorbestehende Drift

Eine bereits vorhandene Formulierung verdient eine spaetere eigene
Entscheidung, ist aber keine Regression von `ea1c252` oder `9ab7910`:

- `assets/templates/spec.feature:36` sagt weiterhin "one per inventory entry",
  waehrend andere Stellen "normalized distinct action" sagen. Nach der
  Normalisierung ist das semantisch gleich, aber die Terminologie sollte
  vereinheitlicht werden.
- Die kanonische Specs-Instruction kennt Web UI, CLI, HTTP API, Library/SDK
  und Message Bus (`assets/instructions/specs.md:44-50`), waehrend das
  Design-Template nur Browser UI, CLI und API-only ausweist
  (`assets/templates/design.md:110-114`).
- Das Review-Template nennt feste Beispiele wie Page Content, HTTP Response,
  Redirect und CLI Output (`assets/templates/review.md:35-43`). Es sollte
  interface-neutral von Output, Return Value, Acknowledgement oder sichtbarem
  State der im Design benannten Interface-Art sprechen.

## Resolution

| Finding | Fix | Evidence |
|---|---|---|
| S1 canonical policy incomplete | Added primary-interface assertions, browser-driver fidelity, no HTTP/`fetch()` UI substitution, and obstacle-as-design-gap rules to `assets/instructions/specs.md`. | `d20b164`; 11 change scenarios and 4 CLI E2E scenarios pass. |
| S2 design guidance incomplete | Restored the design-owned E2E runner, step location, infrastructure, framework, strictness, matrix, and API-only obstacle distinction. | `bef6724`; design guidance regular and E2E scenarios pass. |
| S3a/S3b retrieval drift | Replaced the stale pre-retrieval prohibition with feature-aware index/search/explore, advisory, blocking, and explicit unavailable guidance. Regenerated all configured targets. | `0e89485`; root and harness mirrors synced; retrieval guidance scenario passes. |
| S4 remediation evidence missing | Added class-specific projected-tree contracts and verification receipt to review instructions and the review template. | `68df916`; review guidance regular and E2E scenarios pass. |
| S5 interface contradictions | Canonicalized database/API-only boundaries, prohibited lower-level browser substitutes, corrected the design obstacle example, and made templates interface-neutral. | `d20b164`, `bef6724`, template follow-up; full repository suite passes. |
| S6 effective-policy freshness | Added override/snapshot acceptance coverage and documented Embedded-vs-Override-vs-generated-target semantics in Arc42 and design artifacts. | `ac8f487`; override scenario passes after `givn skills sync`. |
| S7 resolver contract | `givn instructions --json` now emits `{id, generates, requires, instruction}`; Commands consume those fields and unsafe change IDs are rejected while global unknown contexts remain compatible. | `d4d2a16`, `fefa780`; JSON and path-safety scenarios plus existing suite pass. |

## Final Verification

- Change suite: `11 scenarios`, `47 steps`, all passed.
- Change E2E suite: `4 scenarios`, `13 steps`, all passed.
- Repository suite: `282 scenarios`, `1754 steps`, all passed.
- Repository E2E suite: `37 scenarios`, `224 steps`, all passed.
- Coverage: merged `7518/11904` lines (`63.2%`), branches `0/0`.
- `cargo fmt --all -- --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- Retrieval-enabled binary installed; root and harness guidance synchronized.
- `givn check review --change fix-skill-instruction-integrity`: mandatory hooks,
  integrity, and overlap dispositions pass.

The report is now an audit record of the original findings and their fixes.
No further user decision is pending in this review.
