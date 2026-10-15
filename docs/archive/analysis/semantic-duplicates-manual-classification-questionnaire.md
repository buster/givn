# Manual Classification Questionnaire: 20 High-Impact Candidates

> **Status: historical.** Measurement basis: a deliberately selected 20-case
> stress set taken from the old 100-E5-candidate baseline, before PR #27. It is
> retained as historical label context and must not be read as a current corpus
> or threshold result. The reference evaluation
> (`semantic-duplicates-manual-evaluation.md`) was removed with the semantic
> engine.

## Ziel

Dieser Katalog dient nicht zur CPU-Zeitmessung. Er soll klären:

- Welche Kombination aus E5, BGE und NLI behält die relevanten Candidates?
- Welche Methode erzeugt False Positives oder False Negatives?
- Welche Schwellenwerte sollten angepasst werden?
- Ob NLI tatsächlich zu aggressiv filtert oder nur semantisch präziser arbeitet.

Die Auswahl ist absichtlich nicht zufällig. Sie enthält alle realen
Methodenstreitfälle aus dem 100-Candidate-Baseline-Lauf:

| Gruppe | Bedeutung | Anzahl |
|---|---|---:|
| BGE-only | BGE empfiehlt, NLI filtert | 13 |
| NLI-only | BGE filtert, NLI empfiehlt | 6 |
| BGE+NLI | Beide empfehlen | 1 |

Die vollständige Rohmatrix mit allen 100 E5-Candidates steht in
`semantic-duplicates-real-model-analysis.md`.

## Klassifikationsschema

Für jedes Szenario genau eine primäre manuelle Bewertung auswählen:

| Wert | Bedeutung für die Threshold-Analyse |
|---|---|
| `DUPLIKAT` | Gleiche beobachtbare Wirkung; eines der Szenarien sollte entfernt oder zusammengeführt werden. |
| `VARIANTE` | Gemeinsamer Kern, aber ein bewusster Unterschied bleibt relevant. |
| `VALID-BOUNDARY` | Ähnliche Oberfläche, aber unterschiedliche Invariante oder Grenze; beide Szenarien sind fachlich gerechtfertigt. |
| `UNRELATED` | Kein relevanter semantischer Zusammenhang trotz E5-Nähe. |
| `UNCERTAIN` | Ohne weitere Domänenklärung keine sichere Entscheidung. Für die Sicherheitsauswertung als relevant behandeln. |

`UNRELATED` ist eine Analyseklasse. Sie ersetzt keine automatische Givn-
Klassifikation.

## Antwortbogen

Für jede `Q`-Zeile ausfüllen:

```text
ID:
Manuelle Klasse: DUPLIKAT | VARIANTE | VALID-BOUNDARY | UNRELATED | UNCERTAIN
Review-relevant: ja | nein | unklar
Gemeinsame Invariante:
Entscheidender Unterschied:
BGE-Bewertung: richtig | falsch | unklar
NLI-Bewertung: richtig | falsch | unklar
E5-Bewertung: richtig | falsch | unklar
E5-Schwelle: erhöhen | beibehalten | senken
BGE-Schwelle: erhöhen | beibehalten | senken
NLI-Schwelle: erhöhen | beibehalten | senken
Begründung:
```

Threshold-Richtung:

- Schwelle senken: mehr Candidates behalten, Recall steigt, Review-Aufwand steigt.
- Schwelle erhöhen: mehr Candidates filtern, Review-Aufwand sinkt, False-Negative-Risiko steigt.
- Bei `UNCERTAIN` keine aggressive Filterung als Erfolg werten.
- NLI bewertet hier symmetrische Entailment-Wahrscheinlichkeit; Neutralität und Contradiction bleiben als erklärende Evidenz erhalten.

## Bewertungsregeln

Für die Sicherheitsauswertung gelten `DUPLIKAT`, `VARIANTE`,
`VALID-BOUNDARY` und `UNCERTAIN` als geschützte Candidates. Ein geschützter
Candidate, den eine Methode filtert, ist ein potenzieller False Negative.

| Kennzahl | Berechnung |
|---|---|
| Protected recall | Geschützte empfohlene Candidates / alle geschützten Candidates |
| Protected precision | Geschützte empfohlene Candidates / alle empfohlenen Candidates |
| False negatives | Geschützter Candidate mit Filterstatus |
| False positives | `UNRELATED`-Candidate mit Empfehlungsstatus |
| Worklist reduction | `1 - empfohlene semantische Candidates / 100` |

Die 20 Fälle sind ein bewusst verzerrtes Stress-Set. Thresholds nicht allein
auf diesem Set endgültig kalibrieren. Nach jeder Änderung die vollständigen
100 Candidates erneut bewerten.

## Candidate Matrix

`BGE F/R/S` bedeutet Forward-/Reverse-/Symmetric-Rohlogit.
`NLI S E/N/C` bedeutet symmetrische Entailment-/Neutral-/Contradiction-
Wahrscheinlichkeit. Die NLI-Spalte stammt aus dem unabhängigen NLI-Lauf über
alle 100 E5-Candidates. `Combined` beschreibt den aktuellen sequenziellen
Combined-Pfad.

| ID | Gruppe | E5 | BGE F/R/S | BGE | NLI S E/N/C | NLI | Combined | Pair |
|---|---|---:|---|---|---|---|---|---|
| Q1 | BGE-only | 0.9845 | 4.5406/2.7871/3.6639 | BGE_RECOMMENDED | 0.2114/0.3385/0.4502 | NLI_FILTERED | BGE_RECOMMENDED + NLI_FILTERED | `spec-explore-permanent`: Existing spec review keeps its current behavior ↔ `spec-retrieval`: Review reports an advisory retrieval candidate |
| Q2 | BGE-only | 0.9788 | 3.7857/3.7856/3.7857 | BGE_RECOMMENDED | 0.1999/0.7519/0.0483 | NLI_FILTERED | BGE_RECOMMENDED + NLI_FILTERED | `remove-dead-skip-list-code`: givn status JSON still reports skipped artifacts after dead code removal ↔ givn status still reports skipped artifacts with the skip marker after dead code removal |
| Q3 | BGE-only | 0.9709 | 3.9668/3.4150/3.6909 | BGE_RECOMMENDED | 0.4292/0.4519/0.1189 | NLI_FILTERED | BGE_RECOMMENDED + NLI_FILTERED | `spec-explore-permanent`: Existing spec review keeps its current behavior ↔ `spec-retrieval`: Review gate keeps retrieval advisory |
| Q4 | BGE-only | 0.9677 | 2.8532/3.4471/3.1502 | BGE_RECOMMENDED | 0.2690/0.4050/0.3260 | NLI_FILTERED | BGE_RECOMMENDED + NLI_FILTERED | `init-agent-target-selection`: Interactive init accepts the default selection with an empty answer ↔ Interactive init can select the .agents target |
| Q5 | BGE-only | 0.9664 | 3.9486/2.9069/3.4278 | BGE_RECOMMENDED | 0.1034/0.0670/0.8296 | NLI_FILTERED | BGE_RECOMMENDED + NLI_FILTERED | `deduplicate-change-dir-detection`: Integrity check detects a placeholder COMMIT hash in tasks.md and fails ↔ `givn-workflow-enforcement`: Integrity hook fails on placeholder commit hash |
| Q6 | BGE-only | 0.9626 | 4.3659/4.4078/4.3869 | BGE_RECOMMENDED | 0.1034/0.4464/0.4501 | NLI_FILTERED | BGE_RECOMMENDED + NLI_FILTERED | `validate-artifact-id-input`: givn check with unknown artifact validates against manifest and provides helpful error ↔ givn eject instruction with unknown artifact validates against manifest and provides helpful error |
| Q7 | BGE-only | 0.9603 | 4.8774/3.6359/4.2567 | BGE_RECOMMENDED | 0.0770/0.1589/0.7640 | NLI_FILTERED | BGE_RECOMMENDED + NLI_FILTERED | `new-command`: Id beginning with a hyphen is rejected by argument parsing ↔ Id beginning with a hyphen passed after the argument separator is rejected by validation |
| Q8 | BGE-only | 0.9601 | 4.1281/2.0631/3.0956 | BGE_RECOMMENDED | 0.0707/0.0386/0.8908 | NLI_FILTERED | BGE_RECOMMENDED + NLI_FILTERED | `init-agent-target-selection`: Consumer selects agent targets during interactive init ↔ Interactive init can select the .agents target |
| Q9 | BGE-only | 0.9598 | 3.0469/3.4491/3.2480 | BGE_RECOMMENDED | 0.0131/0.0637/0.9232 | NLI_FILTERED | BGE_RECOMMENDED + NLI_FILTERED | `givn-workflow-enforcement`: Archive blocked when merged permanent specs fail verify ↔ Archive re-runs verify against merged permanent specs and passes |
| Q10 | BGE-only | 0.9581 | 1.8620/4.7565/3.3092 | BGE_RECOMMENDED | 0.0061/0.8406/0.1534 | NLI_FILTERED | BGE_RECOMMENDED + NLI_FILTERED | `new-command`: Id beginning with a hyphen is rejected by argument parsing ↔ Id ending with a hyphen is rejected |
| Q11 | BGE-only | 0.9580 | 3.5355/4.6515/4.0935 | BGE_RECOMMENDED | 0.2095/0.6161/0.1744 | NLI_FILTERED | BGE_RECOMMENDED + NLI_FILTERED | `bge-reranking-evidence`: Approval writer rejects a missing Rust parity attestation ↔ Approval writer rejects an incomplete local BGE artifact set |
| Q12 | BGE-only | 0.9539 | 3.2733/3.1907/3.2320 | BGE_RECOMMENDED | 0.2850/0.2377/0.4773 | NLI_FILTERED | BGE_RECOMMENDED + NLI_FILTERED | `bge-reranking-evidence`: The E5 similarity gate remains available without BGE ↔ The independent E5 gate remains separate from BGE evidence |
| Q13 | BGE-only | 0.9520 | 4.0942/3.6835/3.8888 | BGE_RECOMMENDED | 0.2503/0.2294/0.5203 | NLI_FILTERED | BGE_RECOMMENDED + NLI_FILTERED | `validate-artifact-id-input`: Consumer invokes givn check with an unknown artifact and gets a clear error ↔ givn check with unknown artifact validates against manifest and provides helpful error |
| Q14 | NLI-only | 0.9695 | 1.1625/2.6490/1.9057 | BGE_FILTERED | 0.6288/0.2036/0.1676 | NLI_RECOMMENDED | BGE_FILTERED; NLI_NOT_SCORED | `init-agent-target-selection`: Skills and commands are not installed to an agent excluded from the selection ↔ givn skills sync installs to the persisted selection only |
| Q15 | NLI-only | 0.9599 | 2.1638/2.2128/2.1883 | BGE_FILTERED | 0.5515/0.4432/0.0053 | NLI_RECOMMENDED | BGE_FILTERED; NLI_NOT_SCORED | `coverage-readme-example`: Coverage-enabled project receives a complete README coverage example ↔ `readme-coverage-badge`: Coverage-enabled project is guided to place the coverage badge directly after the README title |
| Q16 | NLI-only | 0.9595 | 1.2322/-1.0887/0.0718 | BGE_FILTERED | 0.5707/0.2755/0.1538 | NLI_RECOMMENDED | BGE_FILTERED; NLI_NOT_SCORED | `spec-explore-change`: A missing change is rejected without changing the permanent index ↔ Change exploration does not persist delta scenarios in the permanent index |
| Q17 | NLI-only | 0.9580 | -1.9840/-0.5072/-1.2456 | BGE_FILTERED | 0.9255/0.0667/0.0078 | NLI_RECOMMENDED | BGE_FILTERED; NLI_NOT_SCORED | `spec-explore-permanent`: File exploration finds a semantically similar permanent scenario ↔ Stdin exploration returns the same candidates as file exploration |
| Q18 | NLI-only | 0.9580 | 2.1165/3.3588/2.7377 | BGE_FILTERED | 0.9208/0.0647/0.0145 | NLI_RECOMMENDED | BGE_FILTERED; NLI_NOT_SCORED | `enforce-post-merge-e2e-gate`: Archive is blocked when merged permanent specs gain @e2e scenarios and e2e_command is unconfigured ↔ Consumer archives a delta that adds e2e scenarios and is blocked without a configured runner |
| Q19 | NLI-only | 0.9533 | 2.6462/1.8385/2.2423 | BGE_FILTERED | 0.5844/0.3821/0.0335 | NLI_RECOMMENDED | BGE_FILTERED; NLI_NOT_SCORED | `install-extras`: givn skills sync excludes addon skills and commands when disabled ↔ givn skills sync includes skill and command addons when enabled |
| Q20 | BGE+NLI | 0.9536 | 3.9635/3.3535/3.6585 | BGE_RECOMMENDED | 0.9162/0.0609/0.0229 | NLI_RECOMMENDED | BGE_RECOMMENDED + NLI_RECOMMENDED | `validate-change-id-input`: Consumer invokes givn status with a path traversal change id and gets a clear error ↔ Path traversal id is rejected by status |

## Manuelle Fragen

Die folgende Frage pro Zeile beantworten. Die Gruppe darf die manuelle
Bewertung nicht vorwegnehmen.

| ID | Manuelle Frage / Fokus |
|---|---|
| Q1 | Sind advisory review und bestehendes spec review dieselbe beobachtbare Review-Funktion, oder trennt die Ausgabesemantik sie fachlich? |
| Q2 | Sind JSON-Ausgabe und Skip-Marker-Ausgabe dieselbe Status-Invariante mit nur anderem Ausgabeformat? |
| Q3 | Ist advisory review gegenüber Review-Gate eine Variante derselben Funktion oder ein anderer Workflow-Vertrag? |
| Q4 | Ist die leere Standardauswahl dieselbe relevante Agent-Auswahl wie die explizite `.agents`-Auswahl? |
| Q5 | Sind beide Placeholder-COMMIT-Hash-Szenarien dasselbe Integritätsverhalten trotz unterschiedlicher Capability? |
| Q6 | Ist die unbekannte Artifact-ID in `check` und `eject` dieselbe Validierungsinvariante oder eine fachlich relevante Command-Variante? |
| Q7 | Sind Parser-Rejection und Validation-after-argument-separator dieselbe Grenze des Identifier-Vertrags? |
| Q8 | Ist die interaktive Agent-Auswahl dasselbe Verhalten wie die Auswahl des konkreten `.agents`-Targets? |
| Q9 | Sind blockiertes Verify und erfolgreiches Verify zwei Zustände desselben Archive-Gates oder bewusst getrennte Szenarien? |
| Q10 | Sind führender und abschließender Bindestrich dieselbe Identifier-Regel oder unterschiedliche Fehlersituationen? |
| Q11 | Sind fehlende Parity-Attestation und inkompletter BGE-Artefaktsatz dieselbe Approval-Voraussetzung oder getrennte Failure-Modes? |
| Q12 | Sind E5-Gate-Verfügbarkeit ohne BGE und die explizite Trennung von E5-Gate/BGE dieselbe Invariante? |
| Q13 | Sind `check`-Fehler aus dem Consumer-Workflow und manifestbasierte `check`-Validierung dieselbe relevante Fehlermeldung? |
| Q14 | Ist die Agent-Auswahl-Invariante wichtig genug, dass BGE-gefiltertes, aber NLI-empfohlenes Verhalten nicht verloren gehen darf? |
| Q15 | Sind Coverage-Beispiel und Badge-Platzierung dieselbe README-Coverage-Funktion oder zwei unabhängige Anforderungen? |
| Q16 | Sind fehlende Change und nicht persistierte Delta-Szenarien dieselbe Index-Sicherheitsinvariante? |
| Q17 | Sind File- und stdin-Exploration dieselbe Nutzeraktion mit zwei Eingabekanälen? |
| Q18 | Sind die beiden E2E-Gates dieselbe Regel für @e2e nach dem Merge oder unterscheiden sich Delta- und Permanent-Scope fachlich? |
| Q19 | Sind deaktivierte und aktivierte Addon-Synchronisation eine gemeinsame Options-Variante oder gegensätzliche, separat zu testende Verträge? |
| Q20 | Bestätigen beide Modelle dieselbe relevante Path-Traversal-Validierung, sodass dieser Candidate als gemeinsamer Goldstandard gelten kann? |

## Threshold-Entscheidung

Nach der manuellen Klassifikation pro Zeile:

| Beobachtung | Parameterhypothese |
|---|---|
| Geschützter Candidate ist `BGE_FILTERED`, NLI aber `NLI_RECOMMENDED` | BGE-Schwelle senken oder BGE-Scoring neu kalibrieren; zuerst prüfen, ob BGE-False-Negatives gehäuft auftreten. |
| Geschützter Candidate ist `BGE_RECOMMENDED`, NLI aber `NLI_FILTERED` | NLI-Schwelle senken oder NLI als diagnostische Evidenz behandeln; nicht automatisch NLI als „zu aggressiv“ interpretieren. |
| `UNRELATED` wird von beiden empfohlen | Entsprechende Schwelle erhöhen oder E5-Grenze erhöhen; prüfen, ob die Candidate-Generierung zu breit ist. |
| `DUPLIKAT` oder `VARIANTE` wird von beiden gefiltert | Sicherheitskritischer False Negative; Schwelle nicht blind erhöhen, sondern Modell-/Feature-Semantik und Candidate-Boundary untersuchen. |
| Viele `UNCERTAIN`-Fälle in einer Gruppe | Keine Threshold-Optimierung aus diesen Fällen ableiten; erst Domänenlabel und Invariante klären. |

Die Entscheidung für einen Ansatz sollte anschließend mit dem vollständigen
100-Candidate-Satz wiederholt werden. Der 20er-Katalog ist ein gezielter
Disagreement-Schnitt, kein unverzerrter Benchmark.
