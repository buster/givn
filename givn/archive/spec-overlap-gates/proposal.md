# Proposal: spec-overlap-gates

## Problem / Opportunity

Ein Projekt, das über Wochen mit givn arbeitet, akkumuliert Szenarien, die
dasselbe Verhalten mehrfach abdecken: identische Szenario-Titel in
verschiedenen Dateien, Teilmengen-Szenarien, dieselbe Invariante in mehreren
Feature-Familien. `givn lint` prüft heute nur Parse-Fehler und @wip-Tags —
kein einziger dieser Befunde wird mechanisch sichtbar oder blockiert. Die
Suite bleibt grün, weil Grün nur beweist, dass Bindings registrierbar sind,
nicht dass Szenarien unabhängig sind.

Zusätzlich lintet `givn lint` standardmäßig auch den Archiv-Baum, was
Befunde über historische, nicht-ausgeführte Szenarien erzeugt, die niemand
mehr ändern kann. Und wenn ein Autor ein schwächeres Szenario durch ein
stärkeres ersetzt (removed+added über zwei Delta-Dateien), gibt es keine
maschinelle Empfangsbestätigung, die den Netto-Effekt ausweist.

## Proposed Solution

- `givn lint` prüft die Szenario-Titel über den **ganzen aktiven Baum**:
  permanente Specs plus die Deltas **aller** aktiven Changes. Ein identischer
  Titel an zwei Stellen ist ein Hard Error, der Dateien und Titel benennt.
  Ausnahme: genau ein Paar "modifiziertes Szenario in einer Change-Delta vs.
  sein Ziel in der permanenten Spec derselben Capability" — sonst wäre jede
  Modifikation ein Fehler. Ein Titel-Rename ist kein modifiziertes Szenario,
  sondern removed+added mit Begründung in review.md.
- Szenarien, die **gleich aussehen** (gleiche Observable-Form: gleiche
  Assertion-Folge nach Normalisierung), aber verschiedene Titel tragen,
  erzeugen eine Warnung und eine **Pflicht-Entscheidung** in review.md
  (Dispositionstabelle): entweder zusammenlegen/löschen oder als eigene
  Boundary begründen. Ohne den Eintrag blockt der Review. Widersprechen sich
  die Facts zweier gleich aussehender Szenarien (Exit-Codes, contains vs.
  not-contains, [DONE], TTY), wird nie eine Zusammenlegung nahegelegt —
  stattdessen ein Hinweis auf das verwandte Szenario.
- Lange Szenarien (> 14 Steps) werden gewarnt; die Dispositionstabelle
  bekommt eine Spalte "split-or-keep", und ein langes Szenario ohne Eintrag
  blockt den Review.
- Der Default-Scope von `givn lint` schließt `givn/archive` nicht mehr ein.
- `givn lint` kann Befunde als JSON ausgeben, damit Werkzeuge und die
  eigenen E2E-Szenarien auf Befunde statt auf Prosa prüfen können.
- Beim Review (oder einem eigenen Kommando) wird ein
  **Net-Delta-Receipt** ausgegeben: wie viele Szenarien die Change addiert,
  modifiziert, entfernt.

## Out of Scope

- Modell-basierte Ähnlichkeitssuche (eigene Change, bedingt).
- Jeder Block auf Ähnlichkeits-Scores: Nur der identische Titel blockt
  hart; alles Weichere warnt und erzwingt eine Disposition. Namensqualität
  (ob zwei überlebende Szenarien besser benannt werden müssen) ist
  Reviewer-Urteil, kein maschineller Check.
- Rückwärtskompatibilität: Altlasten bestehender Projekte werden manuell
  bereinigt; der Gate wird nicht grandfathert.
- Parsen fremder Step-Definitionen oder Testcode.
- Längen-Hard-Fail (ein harter Block bei >19 Steps wurde verworfen —
  er setzt den Anreiz, sichtbare Steps in undurchsichtige Composite-Steps
  zu verpacken).
- Runner-Doppelausführung im Referenzprojekt (bewusst verschoben).

## Open Questions

- Dispositions-Volumen auf givns eigenem Baum: 154 Szenarien, Anzahl der
  Shape-Matches ungemessen. Falls die Zahl groß ist, muss die Change
  entscheiden, ob die eigene Baum-Disposition Teil der Change ist oder als
  Folge-Change läuft — der Review der Change selbst blockt sonst.
- Wortlaut der Ausgabe-Kategorien und das JSON-Schema (Warnung/Error/Hinweis,
  Datei, Titel, Befundart) — wird im Design fixiert und per E2E getestet.
- Position des Hard-Errors: `givn lint` (frühester Zeitpunkt) vs. `givn
  check` — Entscheidung im Design, Empfehlung aus dem Review: lint.
