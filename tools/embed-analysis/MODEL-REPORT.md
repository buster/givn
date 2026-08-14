# givn Szenario-Duplikaterkennung: Tool + Modellreport

Datum: 2026-08-14
Ort: `tools/embed-analysis/` in diesem Repository

## Zusammenfassung

Ziel: prüfen, ob CPU-Satz-Embeddings dabei helfen, Duplikate und semantische
Überschneidungen von givn-Gherkin-Szenarien zu erkennen — und welches Embedding-
Modell dafür zu empfehlen ist. Korpus: die 223 aktiven Szenarien von watn
(`givn/specs`), plus ein synthetischer deutscher Testdatensatz.

Zentrale Befunde:

1. **Embeddings taugen zum Kandidaten-Surfacing, nicht zum harten Block.**
   Die Trennschärfe zwischen echten Duplikaten und legitimen Boundary-Paaren
   reicht nicht für einen Schwellwert-Gate (siehe Separation).
2. **Es gibt zwei Dispositionen, keine "Differenzierungs"-Pflicht.**
   Nah-Duplikate sind entweder zu **mergen** (Duplicate / Variant → Examples-Row,
   Parameter, löschen) oder — nur wenn es eine andere Produktions-Boundary ist —
   **als eigenes Szenario zu behalten und umzuformulieren** (KEEP-AS-BOUNDARY).
   Das Tool erzwingt genau diese Entscheidung am Retrieval-Treffer.
3. **Modell-Empfehlung: `intfloat/multilingual-e5-large`.** Bestes Ergebnis auf
   dem englischen Korpus und zugleich perfekt auf dem deutschen Testset. givn
   ist bewusst nicht englisch-only; deutsche Szenarien (mit englischem
   Fachvokabular) sind ein explizites Requirement, daher ist ein multilinguales
   Modell der sichere Weg.

## Das Tool

`tools/embed-analysis/analyze.py` (Läuft mit `uv run`):

- `index --watn <watn>` — parst und embeddet alle aktiven Szenarien, schreibt
  `index.npz` + `index.json`. Optional `--with-step-defs` embeddet zusätzlich
  die Implementierungs-Bodies der zugeordneten Cucumber-Bindings aus
  `<watn>/tests/steps/*.rs`.
- `query` — embeddet neuen Szenariotext (drei Eingabemodi: `--text`, gepipte
  stdin, interaktive REPL) und zeigt die top-k ähnlichsten Szenarien. Treffer
  über `--diff-threshold` bekommen die MERGE / KEEP-AS-BOUNDARY-Entscheidung
  als Hinweis.
- `evaluate` — das labelte Experiment (8 Dup- vs 10 Legit-Paare, transkribiert
  aus `docs/feature-step-overlap-report.md` von watn) mit Hold-one-out-Retrieval
  und Near-Duplicate-Zonen-Analyse. `--german` wechselt auf den synthetischen
  deutschen Satz.

Index und Modell-Embeddings sind generierte Artefakte und werden **nicht**
committet (`.gitignore`): der Index ist per `index` reproduzierbar, das Modell
lädt fastembed beim ersten Lauf herunter.

## Separation (englischer Korpus, beste Schwellwert-Fehlerquote)

Überlappungsband (max legit − min dup) ist in allen Modellen > 0: kein
Schwellwert trennt Duplikate und legitime Boundary-Paare vollständig.

| Modell | Dim | Fehler best. Schwellwert | dup/legit Median |
|---|---|---|---|
| BAAI/bge-base-en-v1.5 | 768 | 4/18 | 0.938 / 0.878 |
| jinaai/jina-embeddings-v2-base-de | 768 | 3/18 | 0.916 / 0.772 |
| paraphrase-multilingual-mpnet-base-v2 | 768 | 3/18 | 0.920 / 0.774 |
| intfloat/multilingual-e5-large | 1024 | **2/18** | 0.979 / 0.950 |

Deterministische Baseline: normalisierter Token-Jaccard 5/18, exakter Titel
fängt nur 2/8 Dup-Paare. Dichte Cluster (Reasoning-Familie) bleiben auch mit
Embeddings unauflösbar — dort ist der echte Duplikat von legitimen Nachbarn
nicht unterscheidbar.

Retrieval (würde ein neues Szenario seinen Duplikat-Partner finden?): dup hit@1
= 6/8 und hit@5 = 8/8 für e5-large; legit hit@1 = 4/10 (wenigste
Falschkollisionen, beste Differenzierung).

## Deutsche Szenarien

Der watn-Korpus selbst ist vollständig englisch; deutscher Inhalt wird über ein
synthetisches Set getestet (echte Umlaute, englische Fachbegriffe). Deutsch ist
ein echtes givn-Requirement, daher ist dieser Test Teil der Modellauswahl.

| Modell | DE Fehler | DE Band |
|---|---|---|
| BAAI/bge-base-en-v1.5 | 0/8 | −0.011 |
| jinaai/jina-embeddings-v2-base-de | 1/8 | +0.006 |
| paraphrase-multilingual-mpnet-base-v2 | 1/8 | +0.094 |
| intfloat/multilingual-e5-large | **0/8** | **−0.014** |

Überraschung: die "deutsch-nativen" Modelle verfehlen gerade die harten
deutschen Duplikatfälle (Subset bei jina-de, Paraphrase bei mpnet); der
englisch-only bge und e5-large trennen perfekt. Grund: deutsche Szenarien sind
von englischen Fachbegriffen durchsetzt. Das e5-large ist das einzige Modell,
das auf beiden Achsen (EN und DE) gewinnt.

## Fazit

- **Nicht blocken, sondern surfacen und entscheiden lassen**: Embedding-
  Ähnlichkeit liefert die Near-Duplicate-Zone; die Klassifikation in
  MERGE (Duplicate/Variant) vs KEEP-AS-BOUNDARY bleibt beim Review.
- **Modell: `intfloat/multilingual-e5-large`** — robust für EN und DE,
  beste Trennschärfe. Langsamer im interaktiven Query (1024-dim), bei dieser
  Korpusgröße unkritisch. Mögliche Verbesserung: expliziter E5-`query:`-Präfix
  im Query-Pfad (fastembed ergänzt ihn ggf. nicht automatisch).

## Ergebnisdateien

- `results.txt` — englischer Korpus, bge-base-en (Baseline)
- `results-jina-de.txt`, `results-mpnet-multi.txt`, `results-e5-large.txt`
- `results-with-defs.txt` — englischer Korpus mit Step-Definitionen
- `de-bge.txt`, `de-jina.txt`, `de-mpnet.txt`, `de-e5.txt` — deutscher Test
