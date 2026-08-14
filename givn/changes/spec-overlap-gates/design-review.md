# Design Review: spec-overlap-gates

## Grilling

Grilling durchgeführt von frischem Subagenten (unabhängiger Kontext) gegen
proposal.md, specs, design.md und den echten Code. 12 Findings, alle
aufgelöst:

1. **RED/GREEN-Kommando unausführbar** — cucumber 0.23 verbietet
   `--name` + `--tags` kombiniert (`conflicts_with`,
   cucumber-0.23.0/src/cli.rs:113). Auflösung: `--name` allein
   (Titel baumweit eindeutig); @e2e während der Entwicklung über
   `--tags @e2e`. Design korrigiert.
2. **Leere Assertion-Folgen** — Net-Delta-Szenario mit Given-only-Fixtures
   würde sich selbst shapen. Auflösung: Szenarien ohne Then-typisierten
   Schritt nehmen an Shape-/Subset-Vergleichen nicht teil (Designregel 1).
3. **JSON-Schema-Widerspruch Spec vs. Design** — Auflösung:
   pretty-printed JSON, ein Objekt pro Finding
   (`kind`, `level`, `files[]`, `titles[]`); Spec-Assertion auf
   `"titles"`-Array korrigiert; Parse-/Wip-Findings sind JSON-Findings.
4. **Vier Szenarien können nicht RED** (zwei modified/removed-"allowed",
   expliziter Archiv-Pfad, split-or-keep-pass) — Auflösung: in tasks.md
   als Charakterisierungs-Szenarien klassifiziert (GREEN-verifizierte
   Regression-Guards nach Feature-Bau, kein RED möglich — Verhalten ist
   heute schon korrekt, der Test schützt es vor Regression).
5. **Zentrale Regel ungetestet** (Delta-vs-permanent → Block;
   nur-permanent → Exempt). Auflösung: zwei Szenarien ergänzt.
6. **Fehlende Fehlerpfade** — review.md fehlt (jetzt: bail mit Nennung
   von review.md, Szenario ergänzt); --json + Parse-Error (Szenario
   ergänzt); unparsebare Datei liefert keine Szenarien (Exit 1 dominiert,
   im Design festgeschrieben); modified ohne Merge-Target (kein Paar,
   Merge-Gate fängt, im Design festgeschrieben).
7. **arc42 fehlt** — Addon aktiv, arc42.md fehlte. Auflösung: arc42.md mit
   03/05/06/08/09/11/12 als Affected: Yes + echte Kapitel-Edits, in
   Design und Tasks aufgenommen.
8. **Dispositions-Volumen ungemessen** — Tracked Open Question: Messung
   direkt nach Implementierung des Fingerprints (eigenes Binär gegen den
   eigenen Baum), Ergebnis dokumentiert in der Change; Review.md trägt
   die generierten Zeilen.
9. **Rule-nested Szenarien** — nicht im Scan (konsistent mit
   feature.scenarios-Walks), im Design festgeschrieben.
10. **CLI-Surface-Lücke** — `--json`-Flag an `Commands::Lint` in
    src/main.rs ergänzt; Verhalten mit `--change`/Pfaden festgeschrieben.
11. **StepType statt Keyword-Strings** — gherkin 0.16 löst And/But bereits
    kontextuell auf (`Step.ty`); führendes `And` ohne Vorgänger ist
    `Given`. Design auf `Step.ty` umgestellt.
12. **Exit 3 in Hooks** — post:lint-Hooks erben den neuen Hard Error;
    gewollt, im Design dokumentiert (`--json` für Maschinenkonsum).

## Hardening

- design.md: Fingerprint-Regeln (StepType, leere Folgen, Scope-Grenzen),
  JSON-Schema, Review-Gate (review.md-Pflicht, Receipt), Single-Scenario-
  Kommando, Risiken (Hook-Exit 3, Dispositionsvolumen), arc42-Abschnitt.
- specs: JSON-Assertion korrigiert; +5 Szenarien (delta-vs-permanent-Block,
  nur-permanent-Exempt, review.md fehlt, --json-Parse-Error, — plus das
  bereits vorhandene Net-Delta-Szenario).
- `givn lint --change spec-overlap-gates`: exit 2 (nur @wip-Findings),
  keine Parse-Fehler — im erwarteten Rahmen (0 oder 2).

## Open Questions (tracked)

- **Dispositions-Volumen im eigenen Baum**: Messung nach Implementierung
  des Fingerprints (Bedingung: vor Review-Gate-GREEN). Falls > ~60
  erforderliche Zeilen: Entscheidung dokumentieren, ob die
  review.md-Zeilen maschinell aus der Gate-Ausgabe generiert werden
  (vorgesehen) und ob die Dispositionstabelle der eigenen Change als
  Folge-Change ausgelagert werden muss. Grenzwert: Die Review-Change
  selbst muss dispositions-sauber bleiben, sonst blockt sie sich selbst.

DESIGN-REVIEW: PASS
