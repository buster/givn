# Tasks — review-preflight-and-cli-evidence

Release note: Enforce one real revision per scenario before review, surface task-integrity and README-impact requirements early, and preserve plain-text CLI interaction evidence without forcing a visual-review sign-off.

## Setup

- [x] Add the SCM commit inspection seam, the scenario parser, and the blocking check.

## Scenario tasks

- [x] Scenario: A checked scenario without a commit fails the task check
  - [x] COMMIT: 024043f3b12921f933c1a40f0d8e4fa02d6d5179 -- feat(givn-workflow-enforcement): enforce one real revision per scenario

- [x] Scenario: A checked scenario reusing another revision fails the task check
  - [x] COMMIT: 88f919610f94edd5fe6c9fa9d6bb1ea3fe35a00c -- test(givn-workflow-enforcement): reject a reused revision

- [x] Scenario: A commit message that omits the scenario title fails the task check
  - [x] COMMIT: a4d0a5d9f35265dd2e29559256225e0bcbaa2e91 -- test(givn-workflow-enforcement): require the scenario title

- [x] Scenario: One real revision per scenario passes the task check
  - [x] COMMIT: 3760b69b2a8587d05f3e76ca69176cdb22051e6b -- test(givn-workflow-enforcement): pair a scenario with its revision

- [x] Scenario: A machine-only CLI records not applicable and passes review
  - [x] COMMIT: 99ec0b80dacec345968a434afd937b06c8a0d322 -- test(visual-review-gate): accept not-applicable

- [x] Scenario: CLI interaction transcripts are declared evidence
  - [x] COMMIT: d21eb8824985c012b904392492adde31c1c4fc85 -- test(archive-proof): accept a CLI transcript

## Readiness

- [x] Run the full suites and commit the review artifacts.
