# Proposal: nli-evidence

## Problem / Opportunity

Die semantische Suche erzeugt aus den E5-Treffern eine begrenzte, aber noch
immer zu breite Arbeitsliste. BGE kann bereits Kandidaten als nachrangige
Evidenz aus der empfohlenen Liste herausnehmen. Die vorhandene NLI-Funktion
zeigt dagegen nur diagnostische Werte in einem einzelnen Review-Pfad und
verändert die Kandidatenmenge nicht. Dadurch bleibt dieselbe Menge an
Falschpositiven in Exploration, Review und Duplikat-Audit sichtbar, obwohl
eine zusätzliche Sicht auf Entailment, Neutralität und Widerspruch verfügbar
ist.

## Proposed Solution

Eine separat aktivierbare lokale NLI-Evidenzstufe bewertet dieselben
begrenzten E5-Kandidaten wie BGE und niemals den vollständigen Paarraum. Für
jeden bewerteten Kandidaten werden die Wahrscheinlichkeiten für Entailment,
Neutralität und Widerspruch in beiden Richtungen sowie eine symmetrische
Zusammenfassung ausgegeben.

NLI reduziert die empfohlene Arbeitsliste, wenn die symmetrische Evidenz die
festgelegte Zulassungsregel nicht erfüllt. Herausgefilterte Kandidaten bleiben
als explizite NLI-Evidenz sichtbar, erhalten keine Givn-Klassifikation und
blockieren keine Review allein aufgrund dieses Filters. Kandidaten, die NLI
empfiehlt, bleiben der bestehenden agentengeführten Klassifikation und
Remediation unterworfen. Wenn BGE und NLI gemeinsam aktiviert sind, muss ein
Kandidat beide Evidenzstufen bestehen, um empfohlen zu werden; die E5-Rohmenge
bleibt zum Vergleich unverändert.

Die Evidenz erscheint konsistent in permanenter Exploration,
change-bewusster Exploration, advisory und blocking Change-Review sowie im
permanenten Duplicate-Audit. Nicht aktivierte oder nicht verfügbare Modelle
werden ausdrücklich ausgewiesen und führen nicht zu einer stillen Behauptung
erfolgreicher Modellteilnahme.

## Out of Scope

- Eine automatische Löschung, Zusammenführung oder fachliche Klassifikation
  durch NLI.
- Eine direkte Zuordnung der NLI-Labels zu `DUPLIKAT`, `VARIANTE`,
  `VALID-BOUNDARY` oder `UNCERTAIN`.
- Eine Änderung der E5-Kandidatenquelle, der deterministischen Ausschlüsse
  oder der bestehenden Remediation-Regeln.
- Ein vollständiger NLI-Lauf über alle permanenten Szenario-Paare.
- Netzwerkzugriff oder Modelldownloads während der Tests.
- Training oder Kalibrierung eines Gherkin-spezifischen Klassifikators.

## Open Questions

- Welche feste, nicht konfigurierbare Zulassungsregel für die symmetrische NLI-
  Evidenz wird durch die Tests und das Design verbindlich gemacht?
- Wie werden Laufzeit, Artefaktidentität und Verfügbarkeitsfehler in den
  bestehenden BGE-Ausgabeformen additiv und deterministisch dargestellt?
