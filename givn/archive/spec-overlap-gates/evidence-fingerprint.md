# Evidence: Fingerprint-Messung auf den 18 watn-Paaren

Messung vom 2026-08-14 mit `tools/overlap-measure/` (deterministischer
Fingerprint dieser Change als Bibliothek) gegen den watn-Baum
(`~/projects/watn`, 223 aktive Szenarien). Ground Truth: die 18 gelabelten
Paare aus `docs/feature-step-overlap-report.md` (8 Dup-Paare, 10 legitime
Boundary-Paare).

## Ergebnis

| Metrik | Wert |
|---|---|
| Dup-Paare gefunden (Titel | Shape | Subset) | **5/8** |
| davon exakter Titel (Hard Error) | 2/8 |
| davon Subset-Warnung | 3/8 |
| Legitime Paare fälschlich geflaggt (Warnungen, keine Hard Errors) | 2/10 |

## Details

Gefundene Dup-Paare:

- F1 (identischer Titel, credential-sources vs provider-setup): Titel + Subset.
- F2 (identischer Titel, model-autosuggest vs search-concurrency): Titel.
- F3 (missing-config subset): Subset.
- F4 (Bash-Widget subset): Subset.
- F6 (empty search subset): Subset.

Verpasste Dup-Paare (alle Paraphrasen mit **verschiedener**
Assertion-Folge):

- F5 "Failed or empty output preserves the original command line" vs
  "Failed or empty generation preserves the original buffer" —
  gleiche Bedeutung, andere Assertionen (buffer vs command line).
- F12a "Unknown persisted reasoning remains active and is sent" vs
  "Existing unknown reasoning remains active after rerunning setup".
- F12b "Minimal reasoning is persisted and sent" vs
  "Free-form reasoning survives persistence and request construction".

Fälschlich geflaggte legitime Paare (nur Warnungen; Review-Entscheid
bleibt beim Menschen):

- "DONE event completes a stream" vs "EOF without DONE is a truncated
  stream" (Subset-Warnung — Assertion-Folgen überlappen real).
- "Partial network reads are reassembled" vs "Malformed nonessential
  events do not discard valid content" (Shape-Match-Warnung).

## Schlussfolgerung für die PLAN-Entscheidung

1. Der deterministische Fingerprint verbessert die Dup-Auffindrate
   gegenüber reinem Titelvergleich von 2/8 auf **5/8**.
2. Die drei verpassten Paare sind Paraphrasen (Lücke 1 der analysis.md):
   gleiche Bedeutung, andere Wortwahl, andere Assertion-Struktur. Genau
   diese Fälle kann nur semantische Suche (Embeddings) finden.
3. **Entscheidung: Change 4 (`spec-retrieval`) wird gebaut.** Das
   Retrieval ergänzt die Gates als Authoring-Werkzeug und
   Review-Dispositions-Hilfe; es wird kein Block auf Scores.
