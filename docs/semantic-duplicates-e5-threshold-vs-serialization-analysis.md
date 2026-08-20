# E5 Threshold Versus Serialization Analysis

## Current Full-Serialization Pool

The current real no-stub run uses the complete serialization and produces
`2292` semantic E5 candidates from `395` scanned scenarios. The BGE/NLI pair
limit is `512`, so both second-stage layers stop with `pair-cap` before model
inference.

The observed E5 score distribution is:

| E5 threshold | Semantic candidates |
|---:|---:|
| `0.9500` | 2292 |
| `0.9550` | 1799 |
| `0.9600` | 1359 |
| `0.9650` | 897 |
| `0.9700` | 526 |
| `0.9703` | 505 |
| `0.9750` | 284 |
| `0.9800` | 125 |
| `0.9850` | 41 |
| `0.9900` | 5 |

`0.9703` is the first tested operating point below the `512` pair bound. It
would remove `1787` of the `2292` candidates produced at the current `0.9500`
boundary. Without manually adjudicated labels, this is an uncontrolled recall
loss, not an evidence-based improvement.

## Serialization Signal

The prior compact run produced `100` semantic candidates on the earlier
385-scenario corpus. The current complete run produces `2292` on `395`
scenarios. This is a strong signal that the new serialization affects E5
similarity, but it is not yet a perfectly controlled A/B comparison because the
corpus also gained archived token-cap scenarios.

The current complete Rust serialization repeats technical section markers for
every scenario, including empty values such as:

```text
feature_description: []
feature_background: []
rule: []
rule_description: []
rule_background: []
scenario_description: []
outline_template: []
examples:
[]
```

It also emits role and empty-field syntax around each step. These repeated
tokens are not removed from the domain content, but they may increase common
boilerplate across documents and artificially raise cosine similarity.

The current Givn corpus contains `13` step tables, `74` docstrings, and no
Rules, Backgrounds, Scenario descriptions, or Examples. Therefore the observed
pool increase is more likely caused by repeated serializer structure and
complete test-prose context than by actual Background/Examples content.

## Decision

Do not raise the E5 threshold as the first fix. Keep the domain boundary at
`0.9500` and run a controlled serialization A/B experiment:

1. Full-content serializer with optional sections omitted when empty.
2. Full-content serializer with compact natural-language section labels.
3. Full-content serializer retaining all non-empty tables, docstrings,
   Backgrounds, Rules, descriptions, and Examples.
4. Compact legacy serializer as the baseline.

The comparison must use the same corpus, same E5 model, same deterministic
exclusions, and the same candidate ordering. Measure candidate counts at
`0.9500`, `0.9600`, `0.9700`, and the first threshold that produces at most
`512` semantic candidates.

Only after the serializer A/B result is known should a threshold change be
considered. The likely target is:

- preserve complete fachliche content;
- remove repeated empty/technical markers from model text;
- keep paths, tags, scores, and classifications as metadata;
- retain an explicit bounded E5 top-K pool before BGE/NLI;
- do not silently turn `0.9703` into the new product threshold without manual
  recall/precision evidence.

## Interpretation

The current BGE/NLI result is not “BGE and NLI filter nothing because they are
weak.” They did not run. The E5 pool exceeded the pair-cap before either model
could score a pair.

The correct order of investigation is therefore:

1. controlled serialization A/B;
2. deterministic bounded E5 candidate selection;
3. real BGE/NLI scoring on the bounded pool;
4. manual classification and recall/precision comparison;
5. only then threshold tuning.
