# Semantic Serialization Token Budget Analysis

> **Status: historical.** Measurement basis: the combined Givn/Watn corpus of
> 425 scenarios, all-pairs tokenizer measurements, and the old strict E5 pool of
> 199 pairs. The pair-cap measurements below predate PR #27 and are retained as
> prior evidence, not as the current operating point. See the current
> [`semantic-duplicates-e5-threshold-vs-serialization-analysis.md`](../../semantic-duplicates-e5-threshold-vs-serialization-analysis.md)
> for the refreshed 394-scenario run.

## Ergebnis

Die geplante vollständige Szenario-Serialisierung wurde gegen die aktiven
Givn-/Watn-Korpora gemessen. Das Messprogramm verwendet die echten lokalen
Tokenizers für E5, BGE und NLI und deaktiviert jede interne Truncation, damit
die Zahlen Rohtokenlängen darstellen.

| Messobjekt | E5 | BGE | NLI |
|---|---:|---:|---:|
| Einzelne vollständige Szenarien `>512` | 0/425 | 0/425 | 0/425 |
| Maximum eines vollständigen Szenarios | 444 | 442 | 454 |
| Alle möglichen Pairs `>512` | - | 766/90100 (`0.85%`) | 1100/90100 (`1.22%`) |
| Alle möglichen Pairs `==512` | - | 7/90100 | 11/90100 |
| Bestehender Strict-Candidate-Pool `>512` | - | 7/199 (`3.52%`) | 16/199 (`8.04%`) |

Die Grenze ist daher für einzelne Dokumente aktuell kein Problem. Sie wird
aber für Pair-Eingaben relevant. Eine vollständige Serialisierung darf BGE/NLI
nicht einfach still abschneiden.

## Modelle und Limits

| Modell | Limitquelle | Effektives Limit |
|---|---|---:|
| E5 `intfloat/multilingual-e5-large` | lokaler Tokenizer `model_max_length` | 512 |
| BGE `rozgo/bge-reranker-v2-m3` | `BGE_MAX_LENGTH` in `src/spec/retrieval.rs` | 512 |
| NLI `mDeBERTa-v3-base-xnli-multilingual-nli-2mil7` | `NLI_MAX_LENGTH` in `src/spec/retrieval.rs` und Modellconfig | 512 |

E5 erhält bei der Einzelserialisierung den Präfix `passage: `. BGE und NLI
erhalten zwei Szenariotexte als ein Pair mit den jeweiligen Tokenizer-
Sondertokens. Die Pair-Zahl enthält diese Sondertokens.

## Corpus

| Quelle | Feature-Dateien | Szenarien |
|---|---:|---:|
| Givn | 26 | 208 |
| Watn | 25 | 217 |
| Gesamt | 51 | 425 |

Die AST-Struktur des aktuellen Korpus enthält:

| Struktur | Anzahl |
|---|---:|
| Rules | 0 |
| Feature-Backgrounds | 0 |
| Rule-Backgrounds | 0 |
| Scenario-Beschreibungen | 0 |
| Step-Tabellen | 13 |
| Docstrings | 74 |
| Scenario-Examples | 0 |

Die Messung enthält deshalb reale Tabellen und Docstrings, aber noch keine
realen Background-, Rule- oder Examples-Fälle. Diese müssen in einer späteren
Messung zusätzlich durch Fixtures abgedeckt werden.

## Einzelne Szenarien

Die vollständige geplante Serialisierung enthält Feature-Kontext,
Scenario-Titel, Rollen (`Given`/`When`/`Then`), Step-Inhalte, Tabellen,
Docstrings und Examples. Die Whitespace-komprimierte aktuelle Serialisierung
enthält nur Titel, Step-Werte und Docstrings.

| Corpus | Tokenizer | Vollständig p50 | Vollständig p90 | Vollständig p95 | Vollständig p99 | Vollständig max | Kurz p50 | Kurz max |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Givn | E5 | 126 | 214 | 238 | 264 | 277 | 96 | 235 |
| Givn | BGE | 124 | 212 | 236 | 262 | 275 | 94 | 233 |
| Givn | NLI | 132 | 228 | 254 | 275 | 328 | 100 | 275 |
| Watn | E5 | 122 | 192 | 221 | 305 | 444 | 91 | 375 |
| Watn | BGE | 120 | 190 | 219 | 303 | 442 | 89 | 373 |
| Watn | NLI | 125 | 199 | 226 | 309 | 454 | 94 | 364 |
| Gesamt | E5 | 124 | 202 | 235 | 288 | 444 | 94 | 375 |
| Gesamt | BGE | 122 | 200 | 233 | 286 | 442 | 92 | 373 |
| Gesamt | NLI | 129 | 213 | 241 | 300 | 454 | 97 | 364 |

Das längste vollständige Szenario ist:

```text
watn/unified-setup-wizard/unified-setup-wizard.feature
Setup wizard guides provider and model configuration page by page
E5: 444, BGE: 442, NLI: 454 Tokens
```

## Pair-Eingaben

Alle `425` Szenarien ergeben `90,100` mögliche ungerichtete Pairs. Diese Zahl
ist eine konservative Obergrenze; der Produktionspfad verwendet nur den
bounded E5-Candidate-Pool.

| Pair-Scope | Pairs | BGE `>512` | BGE max | NLI `>512` | NLI max |
|---|---:|---:|---:|---:|---:|
| Givn intern | 21,528 | 23 (`0.11%`) | 546 | 125 (`0.58%`) | 627 |
| Watn intern | 23,436 | 337 (`1.44%`) | 798 | 425 (`1.81%`) | 834 |
| Givn ↔ Watn | 45,136 | 406 (`0.90%`) | 717 | 550 (`1.22%`) | 781 |
| Gesamt | 90,100 | 766 (`0.85%`) | 798 | 1,100 (`1.22%`) | 834 |

Watn enthält sowohl die längsten Einzeldokumente als auch die höchste
Pair-Überschreitungsrate. Cross-source-Pairs stellen absolut die größte Zahl
der Überschreitungen, weil sie die größte Pair-Menge bilden.

## Bestehender Strict-Pool

Als Näherung für einen realen Candidate-Pool wurde der vorhandene Strict-
Baseline-Report mit `199` E5-Pairs verwendet. Dieser Pool basiert noch auf der
alten Kurzserialisierung; nach Einführung der vollständigen Serialisierung muss
der E5-Index und der Candidate-Pool neu berechnet werden.

| Scope | Pairs | BGE `>512` | BGE max | NLI `>512` | NLI max |
|---|---:|---:|---:|---:|---:|
| Givn intern | 160 | 4 (`2.50%`) | 522 | 13 (`8.13%`) | 574 |
| Watn intern | 39 | 3 (`7.69%`) | 589 | 3 (`7.69%`) | 599 |
| Givn ↔ Watn | 0 | 0 | - | 0 | - |
| Gesamt | 199 | 7 (`3.52%`) | 589 | 16 (`8.04%`) | 599 |

Die Strict-Pool-Zahlen sind keine endgültige Produktionszahl. Sie zeigen aber,
dass die Pair-Grenze im realistischen Kandidatenbereich deutlich häufiger
erreicht wird als in der All-Pairs-Mittelung.

## Aktueller Givn-E5-Pool

Der echte aktuelle Givn-Lauf erzeugt `100` semantische E5-Candidates. Für genau
diese Candidates ergibt die vollständige geplante Serialisierung:

| Kategorie | Anzahl |
|---|---:|
| Passt vollständig in BGE und NLI | 93 |
| Überschreitet nur NLI | 3 |
| Überschreitet BGE und NLI | 4 |
| Überschreitet nur BGE | 0 |
| Überschreitet BGE insgesamt | 4 |
| Überschreitet NLI insgesamt | 7 |

Damit können `93/100` aktuelle E5-Candidates vollständig an beide Pair-Modelle
übergeben werden. `7/100` würden bei einer vollständigen Serialisierung für
mindestens ein Modell über das Fenster hinausgehen.

| E5-Score | BGE Tokens | NLI Tokens | Betroffene Methode | Pair |
|---:|---:|---:|---|---|
| 0.9878 | 585 | 622 | BGE + NLI | `spec-duplicates`: Exact duplicates across features are reported together ↔ Exact duplicates in one feature are reported together |
| 0.9625 | 522 | 574 | BGE + NLI | `review-disposition-gates`: Review blocks when a delta scenario shape-matches a permanent scenario without disposition ↔ Review passes when a shape match exists only between permanent scenarios |
| 0.9578 | 513 | 562 | BGE + NLI | `review-disposition-gates`: Review blocks when a shape match involving the change's delta is undisposed ↔ Review passes when a shape match exists only between permanent scenarios |
| 0.9562 | 519 | 530 | BGE + NLI | `spec-explore-change`: Change-aware JSON exploration labels candidate sources ↔ Change-aware file exploration finds permanent and active change candidates |
| 0.9562 | 488 | 532 | NLI only | `review-disposition-gates`: Review blocks when a delta scenario shape-matches a permanent scenario without disposition ↔ Review passes when every shape match is disposed and reports the net-delta receipt |
| 0.9689 | 479 | 520 | NLI only | `review-disposition-gates`: Review blocks when a shape match involving the change's delta is undisposed ↔ Review passes when every shape match is disposed and reports the net-delta receipt |
| 0.9614 | 463 | 513 | NLI only | `review-disposition-gates`: Review blocks with a clear message when review.md is missing ↔ Review passes when a shape match exists only between permanent scenarios |

In the current implementation, BGE and NLI tokenizers truncate pair input to
their configured `512` tokens. These seven Pairs therefore do not fail the
command; they are currently scored with incomplete right/left context. The
token-cap handling must make this explicit before complete serialization is
enabled.

## Konsequenz

Die vollständige Serialisierung kann für einzelne Szenarien zunächst ohne
Chunking verwendet werden. Für BGE/NLI-Pairs ist eine explizite Strategie
notwendig:

- Ein Pair bis `512` Tokens wird direkt bewertet.
- Ein längeres Pair darf nicht still abgeschnitten werden.
- Die Implementierung muss entweder deterministisch chunking verwenden oder den
  Candidate als `BGE_UNSCORED`/`NLI_UNSCORED` mit einem nachvollziehbaren Grund
  ausweisen.
- `content_hash` und Evidence-Hash müssen auf der vollständigen
  Serialisierung basieren.
- `chunk_count`, `token_count_left`, `token_count_right` und der verwendete
  Aggregationsmodus müssen in der Evidence sichtbar sein.

Die sicherste erste Implementierung ist ein vollständiger Single-Chunk-Pfad
für alle aktuell passenden Szenarien und ein expliziter `pair-cap`-Fallback für
überschreitende Pairs. Danach kann Chunking separat gemessen werden. Ein
stilles Kürzen würde die fachliche Vollständigkeit verletzen und wäre für die
manuelle Threshold-Analyse nicht akzeptabel.
