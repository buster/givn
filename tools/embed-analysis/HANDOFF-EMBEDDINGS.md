# Handoff: Embedding-basierte Duplikat-/Überschneidungserkennung für givn

Adressat: Agent für die Optimierung von givn anhand von Embeddings.
Quelle der Zahlen: Experimente in `tools/embed-analysis/` (Korpus: 223 aktive
watn-Szenarien + synthetischer deutscher Satz), 2026-08-14.

## Kurzantwort (drei Entscheidungen)

1. **Lohnen sich Embeddings? Ja — als Retrieval/Kandidaten-Surfacing beim
   Authoring, nicht als harter Block.**
2. **Wie?** Index einmal in der Entwicklung bauen, bei jedem neuen Szenario die
   top-k ähnlichen anzeigen und eine explizite MERGE / KEEP-AS-BOUNDARY-
   Entscheidung erzwingen. Deterministische Checks zuerst, Embeddings für die
   semantischen Near-Misses.
3. **Welches Modell?** `intfloat/multilingual-e5-large` (beste Trennschärfe auf
   EN **und** DE). Leichtere Alternative: `BAAI/bge-base-en-v1.5`.

---

## 1. Lohnen sich Embeddings? (Befund)

- **Ja für Retrieval:** 6/8 bekannte Duplikat-Paare sind der nächste Nachbar
  ihres Partners, 8/8 in den Top-5 (e5-large). Ein neues, umformuliertes
  Szenario wird zuverlässig seinem echten Duplikat zugeordnet.
- **Nein als Gate:** Kein Schwellwert trennt Duplikate von legitimen
  Boundary-Paaren — selbst e5-large hat ein Überlappungsband > 0 (max legit −
  min dup = +0.075). Dichte Cluster (Reasoning-Familie) bleiben unauflösbar.
  Ein Auto-Block würde legitime Szenarien falsch treffen.
- **Kombination ist der Weg:** deterministische Checks (exakter normalisierter
  Titel/Fingerprint — fängt nur 2/8 Dup-Paare) für klare Fälle + Embedding-
  Retrieval für semantische Near-Misses, deren Klassifikation beim Menschen
  bleibt.

## 2. Wie einsetzen (Mechanismus / Deployment)

- **Dev-only:** `index` einmal bauen (Modell-Download + CPU-Inferenz, in der
  Entwicklung verfügbar), danach `query` oft (lädt den ~0.7 MB-Index, embeddet
  nur das neue Szenario). Nichts committen — Index und Modell-Cache bleiben
  lokal (gitignored). CI braucht kein Embedding.
- **Scope auf Gherkin:** nur Szenariotext (Titel + Given/When/Then). Step-
  Definitionen sind pro Projekt sprachspezifisch (Rust/Java/Python/…) und
  **nicht generisch extrahierbar** → bleiben außen vor. Optionaler
  `--with-step-defs` nur als projekt-lokale Optimierung (kein generischer
  Sprach-Parser). Aufwand dieses Weglassens: nur Boundary-Enttäuschung, kein
  Verlust beim Duplikat-Finden.
- **Disposition am Treffer:** jedes Nah-Duplikat ist entweder
  **MERGE** (Duplicate/Variant → Examples-Row/Parameter/löschen) oder
  **KEEP-AS-BOUNDARY** (andere Produktions-Boundary → behalten, Boundary im
  Titel umformulieren). Der Mensch entscheidet; Embeddings liefern nur die Zone.
- **E5-Prompt-Präfix:** E5-Modelle erwarten `query:` / `passage:`-Präfixe.
  Prüfen, ob fastembed sie ergänzt; ggf. `query:` im Query-Pfad explizit setzen.

## 3. Welche Modelle (Messergebnis)

Englischer Korpus (Fehler bester Schwellwert; dup/legit hit@1):

| Modell | Dim | EN Fehler | EN dup/legit hit@1 | DE Fehler | DE Band |
|---|---|---|---|---|---|
| BAAI/bge-base-en-v1.5 | 768 | 4/18 | 6/8 / 5/10 | 0/8 | −0.011 |
| jinaai/jina-embeddings-v2-base-de | 768 | 3/18 | 6/8 / 5/10 | 1/8 | +0.006 |
| paraphrase-multilingual-mpnet-base-v2 | 768 | 3/18 | 6/8 / 6/10 | 1/8 | +0.094 |
| **intfloat/multilingual-e5-large** | 1024 | **2/18** | 6/8 / **4/10** | **0/8** | **−0.014** |

**Empfehlung: `intfloat/multilingual-e5-large`.** Robust für EN und DE; givn-
Szenarien dürfen deutsch sein (mit englischem Fachvokabular) — das ist ein
explizites Requirement. e5-large ist das einzige Modell, das auf beiden Achsen
gewinnt. Kosten: 2.24 GB Download, ~10 GB RAM, langsamer im interaktiven Query
(1024-dim) — bei Korpusgrößen im Bereich von watn unkritisch.

**Leichtere Alternative:** `BAAI/bge-base-en-v1.5` (210 MB). Qualitätsdifferenz
auf EN nur 4/18 vs 2/18; trotzdem perfekt (0/8) auf den deutschen Testpaaren,
weil die Szenarien von englischen Fachbegriffen durchsetzt sind. Sinnvoll, wenn
Modellgröße/Ladezeit den Ausschlag geben.

**Ausgeschlossen:** die "deutsch-nativen" jina-de / mpnet-multi verfehlen gerade
die harten deutschen Duplikatfälle (Subset/Paraphrase) und bringen nichts.

## Werkzeug

`tools/embed-analysis/analyze.py` (läuft mit `uv run`):
`index --watn <watn>`, `query` (REPL/stdin/`--text`, `--diff-threshold`),
`evaluate` (+ `--german`, optional `--with-step-defs`). README.md und
MODEL-REPORT.md dokumentieren Details und Zahlen.

## Wo in givn einbauen (Empfehlung)

- **Authoring:** beim Erstellen eines neuen Szenarios den Index abfragen und
  Nähe-Treffer anzeigen; bei Treffern über dem Schwellwert die MERGE /
  KEEP-AS-BOUNDARY-Entscheidung verlangen statt zu blocken.
- **Review/Archive:** Gherkin-only-Retrieval als Check, dass kein bereits
  ownedes/dupliziertes Szenario eingeführt wird (Ergänzung zur bestehenden
  Duplikat-Titel-Prüfung).
