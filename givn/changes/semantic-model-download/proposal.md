# Proposal: semantic-model-download

## Problem / Opportunity

Ein frisch eingerichtetes Projekt kann die semantischen Givn-Befehle mit dem
Standardumfang nicht ausführen. Die erste Indexierung bricht ab, bevor die
benötigten E5-Artefakte beschafft werden können. Die nachgelagerten BGE- und
NLI-Evidenzstufen verlangen ebenfalls manuell vorbereitete Dateien, obwohl sie
mit dem Standardumfang ausgeliefert werden.

Dadurch ist das Verhalten vom Zustand lokaler Caches abhängig und ein fehlender
Index wird je nach semantischem Befehl mit unterschiedlichen oder irreführenden
Fehlern gemeldet. Ein Autor erhält keine klare Handlungsanweisung, ob er den
Index erzeugen, ein Modell bereitstellen oder eine Umgebungsvariable setzen
muss.

## Proposed Solution

Wenn ein Autor einen expliziten semantischen Befehl ausführt, stellt Givn die
benötigten Modellartefakte automatisch im XDG-Benutzercache bereit. Die
Artefakte liegen standardmäßig unter `XDG_CACHE_HOME/givn` beziehungsweise
`~/.cache/givn`, werden zwischen Projekten wiederverwendet und nicht in jedes
Projekt kopiert. Der semantische Index bleibt dagegen projektbezogen.
Bei der ersten Verwendung werden fehlende E5-Artefakte geladen, bevor das
Tokenlimit geprüft wird; anschließend erzeugt `givn spec index` den permanenten
Index erfolgreich. Die Cache-Inhalte werden bei späteren Aufrufen
wiederverwendet. Ein explizit angegebener lokaler Modell- oder Cachepfad darf
den Standardcache ersetzen.

Mit dem Standardumfang können auch BGE- und NLI-Evidenzstufen ihre benötigten
Artefakte automatisch in den jeweiligen XDG-Unterverzeichnissen beziehen,
sobald die jeweilige Stufe explizit ausgewählt oder durch die bestehende
Zulassung aktiviert wird. Die BGE-Zulassung bleibt
eine eigenständige Sicherheits- und Qualitätsentscheidung; fehlende Zulassung
oder fehlgeschlagene Beschaffung wird getrennt und verständlich ausgewiesen.

Alle semantischen Oberflächen verwenden bei einem fehlenden Index denselben
Hinweis mit dem konkreten Pfad und dem nächsten Befehl:
`spec index is missing at <path>; run \`givn spec index\` first`.
Schlägt eine Beschaffung fehl, nennt die Fehlermeldung den betroffenen Cache
sowie die lokale Artefakt- oder Umgebungsvariablen-Alternative. Ein unvollständig
heruntergeladenes Artefakt darf nicht als gültiger Cache verwendet werden.

Ein Build ohne Standard-Features behält den expliziten Fehler `retrieval
unavailable`. Die bestehenden E5-, BGE- und NLI-Bewertungsgrenzen sowie die
Ausgabeformate ändern sich durch die Bereitstellung der Artefakte nicht.

## Out of Scope

- Modelle, Schwellenwerte, Tokenlimits, Ranking und Evidenzklassifikation
  werden nicht fachlich neu kalibriert.
- Givn lädt keine Modelle beim Installieren, beim Start eines nichtsemantischen
  Befehls oder ohne ausdrückliche semantische Operation.
- Die bestehende BGE-Zulassungsentscheidung wird nicht stillschweigend durch
  einen Download ersetzt.
- Der Givn-Binary-Self-Upgrade und Änderungen am permanenten Indexschema sind
  nicht Bestandteil dieser Änderung.

## Open Questions

Keine. Downloads werden ausschließlich durch einen expliziten semantischen
Befehl ausgelöst; Zulassung und Artefaktverfügbarkeit bleiben getrennte
Zustände, und Offline- oder fehlerhafte Beschaffung endet mit einer konkreten
Handlungsanweisung.
