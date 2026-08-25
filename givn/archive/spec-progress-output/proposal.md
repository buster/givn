# Proposal: spec-progress-output

## Problem / Opportunity

`givn spec index` bleibt beim ersten Lauf oft mehrere Minuten stumm: Waehrend
der Befehl die Retrieval-Modelle herunterlaedt, laedt und die Szenarien
verarbeitet, erscheint keine einzige Ausgabe. Erst am Ende meldet er das
fertige Indexergebnis. Dasselbe gilt fuer andere Spec-Kommandos (`spec
review`, `spec duplicates`, `spec search`), die Modelle nachladen oder viele
Szenarien verarbeiten.

Nutzer halten den Befehl fuer haengengeblieben, brechen ab oder starten ihn
mehrfach. Der Befehl arbeitet aber korrekt; es fehlt lediglich jede
Rueckmeldung waehrend der Laufzeit.

## Proposed Solution

Waehrend laenger laufender Phasen meldet das Kommando kontinuierlich Status an
die Fehlerausgabe (stderr), sobald eine Phase beginnt und sobald sie
abgeschlossen ist:

- Beginn und Abschluss des Modellabrufs fuer jedes benoetigte Modell
  (Retrieval-Embedder, Reranker, NLI), einschliesslich des Ziels im Cache.
- Datei-Fortschritt waehrend jedes Modell-Downloads.
- Vorbereitung des Retrieval-Embedders vor der Szenario-Verarbeitung.
- Verarbeitung der Szenarien, einschliesslich der Anzahl zu
  verarbeitender und bereits verarbeiteter Szenarien.
- Die bestehenden Ergebnisausgaben auf stdout bleiben unveraendert,
  insbesondere maschinenlesbare Ausgaben wie das Duplicate-Audit-JSON.

Ein uebersprungenes Modell (bereits im Cache vorhanden) wird nicht als
Download gemeldet; fuer deaktivierte Evidence-Layer erscheint keine
Download-Meldung.

## Out of Scope

- Berechnungsergebnisse, Schwellenwerte, Indexinhalte und Modellwahl werden
  nicht geaendert.
- Die Ausgabe bei schnellen Kommandos ohne Modellbedarf bleibt unveraendert.
- Keine interaktiven Fortschrittsbalken, sofern Statuszeilen die
  Anforderung erfuellen.

## Open Questions

Geklaert:

- Statuszeilen pro Phase UND Datei-Fortschritt waehrend Downloads (beides).
- Statusmeldungen erscheinen immer, auch bei nicht-interaktiver Ausfuehrung
  (keine Terminal-Erkennung, keine Unterdrueckung).
