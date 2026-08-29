# This change is intentionally guarded by no scenario tests.
#
# It is a build-profile and CI-infrastructure change, not an observable
# command-behaviour change: the development profile stops emitting full
# debug information and CI stops producing incremental compilation
# artifacts, so the acceptance suite fits the runner disk.
#
# The two observable contracts this change must preserve are already
# pinned by permanent scenarios, which act as the regression guards:
#   - the development/installed build distinction
#     (givn/specs/release-strict-model-overrides/…: a development build
#      still honours fixture requests; an installed build still ignores
#      and announces them),
#   - coverage still measuring line coverage
#     (givn/specs/coverage-script-workflow/…).
# The literal disk-footprint outcome is not observable through any givn
# command, mirroring the zero-scenario precedent of
# semantic-scoring-throughput.

@givn.delta @ci-debuginfo-optimization

Feature: The CI acceptance suite fits the runner disk without changing observable build behaviour
