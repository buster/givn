# Proposal: bge-reranking-evidence

## Problem / Opportunity

E5 findet Szenarien, die dieselbe ausführbare Invariante beschreiben, und
Szenarien, die nur an derselben Oberfläche oder an derselben Wortwahl liegen.
Die aktuelle semantische Evidenz zeigt diese Kandidaten, liefert aber keine
zweite, gemeinsam auf beide Szenariotexte gerichtete Relevanzsicht. Dadurch
müssen Autoren und Reviewer eine große und schlecht priorisierte Trefferliste
manuell einordnen. Besonders bei Kandidaten über `0.9500` ist nicht sichtbar,
ob E5 eine echte Duplikat- oder Variantenbeziehung nur ungenau eingeordnet hat
oder ob eine fachliche Grenze vorliegt. Eine reine Umsortierung würde die
Falschpositiven trotzdem auf der Arbeitsliste lassen und kann dadurch
unnötiges Umbenennen oder künstliche Textverlängerung auslösen.

Es besteht außerdem das Risiko, dass ein Autor oder ein LLM Szenarien unnötig
lang, spezifisch oder mit wiederholtem Setup formuliert, um einen
Similarity-Score zu senken. Das verschlechtert die Lesbarkeit und verschleiert
Invarianten, ohne eine sichere fachliche Entscheidung zu liefern.

## Proposed Solution

Eine optionale zweite lokale Evidenzstufe bewertet die von E5 gefundenen
Kandidaten gemeinsam. Sie ist in allen semantischen Suchflächen sichtbar:
bei der permanenten Exploration, der Exploration innerhalb einer laufenden
Change, dem advisory und blocking Change-Review sowie der permanenten
Duplikat-Suche.

Die Ausgabe macht für jeden bewerteten Kandidaten nachvollziehbar, welches
Modell verwendet wurde, welchen symmetrischen Raw-Score es geliefert hat und
wie lange die Bewertung gedauert hat. Die beiden Richtungen eines
Szenario-Paares werden berücksichtigt, damit die Evidenz nicht von der
Reihenfolge der Endpunkte abhängt.

Die zweite Evidenzstufe bleibt begrenzt und deterministisch. Sie bewertet nur
eine feste, nachvollziehbar ausgewählte Teilmenge der E5-Treffer und niemals
den vollständigen Raum aller permanenten Paare. Deterministische Ausschlüsse
greifen vor der zweiten Evidenzstufe. Unterhalb des durch die gelabelte
Stichprobe belegten Betriebswerts wird ein E5-Treffer ausdrücklich aus der
empfohlenen Review-Arbeitsliste gefiltert und als solche Evidenz ausgewiesen.
Die E5-Rohkandidaten und die Filterstatistik bleiben sichtbar. Ein
`BGE_FILTERED`-Kandidat ist ein expliziter, nicht-blockierender Evidenzstatus:
Er muss nicht als `DUPLIKAT`, `VARIANTE`, `VALID-BOUNDARY` oder `UNCERTAIN`
klassifiziert werden und darf die blocking Change-Review nicht blockieren.
BGE entscheidet dennoch keine Givn-Klasse, schreibt keine Feature-Datei und
bestätigt keine Remediation. Ein `BGE_RECOMMENDED`-Kandidat bleibt der
normalen agentengetragenen Klassifikation und Remediation unterworfen.

Ohne ausdrückliche Aktivierung bleibt die bisherige E5-Suche einschließlich
ihrer Kandidatenmenge unverändert. Ist die lokale zweite Evidenz nicht
verfügbar, wird dies ausdrücklich als nicht verfügbar ausgegeben und nicht als
erfolgreiche Modellteilnahme verschleiert. E5-only und die aktivierte
Kombination können dadurch getrennt verglichen werden. Die aktivierte
Kombination wird nur als Verbesserung gewertet, wenn die gelabelte Stichprobe
die Reduktion der Falschpositiven ohne unvertretbaren Top-K-Recall-Verlust
bestätigt.

Die Änderung liefert außerdem eine gelabelte Stichprobe mit Duplicate,
Variante, Valid-Boundary und Unrelated sowie Messungen für Top-K-Recall,
Kandidatenmenge, Laufzeit, Precision, Recall, False Positives und False
Negatives. Szenario-Zeichen, Steps, Assertions und Scores werden gemeinsam
ausgewertet, um sichtbare Verbosität nicht als zulässige
Similarity-Umgehung zu belohnen. Die Authoring- und LLM-Guidance fordert
präzise Invarianten und Boundary-Fakten und verhindert unnötige Länge,
dupliziertes Setup und irrelevante Qualifier.

## Out of Scope

 - Eine automatische Löschung, Zusammenführung oder fachliche Klassifikation
   durch die zweite Evidenzstufe.
  - Ein neuer Givn-Klassifikations- oder Remediation-Schwellenwert. Der
    gelabelte BGE-Betriebswert ist ausschließlich ein nicht-blockierender
    Admission-Status für die empfohlene Arbeitsliste.
 - Eine Implementierung von NLI oder eine Vermischung der NLI-Entscheidung mit
   dieser Change.
 - Ein vollständiger Lauf der zweiten Evidenz über alle permanenten Paare.
 - Netzwerkzugriff oder Modelldownloads während der Tests.
 - Änderungen an `givn/archive/spec-duplicates/` oder an der bereits gemergten
   permanenten Duplicate-Spec ohne konkrete Notwendigkeit.

## Open Questions

  - Die Ausgabeversionierung und die Zählung von E5-Rohkandidaten,
    BGE_RECOMMENDED-, BGE_FILTERED- und ungescorten Paaren werden im Design
    festgelegt.
  - Das Design muss die gemeinsame bounded Top-K-Kandidatenpipeline so wählen,
    dass ein späterer, unabhängig per `nli-evidence` aktivierbarer NLI-Slice
    dieselbe Kandidatenmenge wiederverwenden kann.
