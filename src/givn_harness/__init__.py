"""givn evaluation harness.

Two deliberately separate modes share one accounting pipeline:

* Controlled evaluation runs a pinned benchmark, givn binary, environment and
  execution policy, then grades frozen snapshots independently.
* Retrospective analysis reads existing ``events.jsonl`` records without any
  model request or code execution.

The package has no import-time side effects and never contacts a provider.
"""

__version__ = "0.2.0"
