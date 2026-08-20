# Proposal: semantic-compact-serialization

## Problem / Opportunity

Die semantische Duplikaterkennung vergleicht Szenarien anhand ihrer
serialisierten fachlichen Struktur. Aktuell tauchen dabei auch optionale
Abschnitte auf, die im jeweiligen Szenario keinen Inhalt haben. Diese
wiederholten Leer-Markierungen sind für den fachlichen Vergleich wertlos,
werden aber als gemeinsame Wörter berücksichtigt und können die Similarity
zwischen unabhängigen Szenarien künstlich erhöhen. Dadurch entsteht ein zu
großer Kandidatenpool und nachgelagerte Qualitätsfilter werden unnötig
blockiert.

## Proposed Solution

Die Duplikaterkennung verwendet weiterhin alle vorhandenen fachlichen Inhalte
eines Szenarios, einschließlich Kontext, Regeln, Beschreibungen, Schritte,
Tabellen, Docstrings und Beispielen. Ein optionaler Abschnitt wird nur dann in
den Vergleich einbezogen, wenn er tatsächlich Inhalt besitzt.

Die resultierende Darstellung enthält keine Platzhalter oder Abschnittsmarker
für leere optionale Inhalte. Szenarien ohne solche Inhalte werden dadurch
nicht über gemeinsame Leer-Strukturen ähnlicher bewertet. Nichtleere Inhalte
bleiben vollständig und in stabiler Reihenfolge vergleichbar.

## Out of Scope

- Die fachlichen Szenarien, ihre Dateien und ihre Bedeutung werden nicht
  verändert.
- Die Similarity-Schwellenwerte und die BGE-/NLI-Filterlogik werden nicht
  geändert.
- Die Behandlung tatsächlich überlanger E5-Passagen wird nicht geändert.
- Eine Neubewertung bereits gespeicherter Reports außerhalb eines regulären
  Index-Neuaufbaus ist nicht Bestandteil dieses Changes.

## Open Questions

Keine. Die Regel gilt für alle optionalen Abschnitte: leere Inhalte werden
weggelassen, vorhandene Inhalte werden vollständig übernommen.
