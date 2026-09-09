# Use Case: support

## Actor

Any Givn actor who creates or reviews commits in a project assisted by an LLM.

## Goal

Make the LLM and harness involved in a commit visible in the commit's native
identity and message without disrupting human commits.

## Main flow

1. A person initializes a project with Givn.
2. The project receives shared commit-attribution plumbing and selected agent
   integrations.
3. An agent harness exposes the attribution context active for a commit.
4. The commit records the available attribution context in its committer
   identity and message.
5. A reviewer can inspect the commit through ordinary Git tools.

## Interactions

| Capability | Consumer action | E2E scenario |
|---|---|---|
| llm-attribution | Initialize a Givn project with OpenCode and Claude Code targets | Consumer initializes a project and receives shared attribution plumbing |
| llm-attribution | Create a commit while attribution context is available | Consumer creates an attributed commit and inspects it with Git |

## Includes

- none

## Extends

- none

## Out of scope

- Complete or tamper-proof provenance
- Server-side enforcement or signature verification
- Selecting the LLM, provider, reasoning policy, or harness
