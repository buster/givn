# Proposal: semantic-reranker-budget

## Problem / Opportunity

Die E5-Suche erzeugt im aktuellen permanenten Korpus mehr semantische
Kandidaten, als die nachgelagerten BGE- und NLI-Pruefungen bisher in einem Lauf
verarbeiten duerfen. Sobald diese Menge die feste Grenze ueberschreitet, wird
keine zweite Stufe ausgefuehrt. Dadurch bleiben alle E5-Kandidaten ohne BGE- oder
NLI-Evidenz, obwohl die Modelle lokal verfuegbar sind.

## Proposed Solution

Die zweite Stufe verwendet ein konfigurierbares Kandidatenbudget. Die
Kandidaten werden nach ihrem E5-Score absteigend geordnet; bis zum eingestellten
Budget werden sie an BGE beziehungsweise NLI weitergegeben. Kandidaten hinter
diesem Budget bleiben als explizit nicht bewertete E5-Kandidaten sichtbar und
werden nicht als durch das Modell gefiltert dargestellt.

Das Standardbudget betraegt `2048`. Liegt die Kandidatenmenge darunter, werden
alle Kandidaten bewertet. Berichte zeigen das aktive Budget sowie die Anzahl
bewerteter, empfohlener, gefilterter und nicht bewerteter Kandidaten. Ein
ungueltiger Budgetwert wird mit einer verstaendlichen Fehlermeldung abgelehnt.

## Out of Scope

- E5-Schwellenwert, E5-Serialisierung und E5-Tokenlimit werden nicht geaendert.
- BGE- und NLI-Modelle, ihre Bewertungsgrenzen und ihre Richtungslogik werden
  nicht geaendert.
- Deterministische Exact- und Structural-Overlaps werden weiterhin vor den
  Modellstufen behandelt.
- Automatische Szenarioaenderungen, Chunking und eine fachliche Klassifikation
  durch BGE oder NLI sind nicht Bestandteil dieses Changes.

## Open Questions

Keine. Das gemeinsame Budget gilt fuer BGE und NLI; die bestehende
unabhaengige Bewertung beider Stufen bleibt erhalten.
