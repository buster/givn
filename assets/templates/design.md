# Design: {{CHANGE_ID}}

## Overview

> How will we implement this change? Summarise the approach.

## Technology Decisions

> What language, framework, libraries, or patterns will be used?
> Justify decisions made here (why this approach over alternatives?).

## Step Definitions

> Location of step definition files:
> Where do the Cucumber/Gherkin step definitions live in the codebase?
>
> Test runner command (from `verify.command` in givn/config.yaml):
> ```
> {{VERIFY_COMMAND}}
> ```

## E2E Smoke Test Infrastructure

> E2e scenarios (`@e2e`) exercise the real interaction surface. Document:
>
> **E2E runner command** (becomes `verify.e2e_command` in givn/config.yaml):
> ```
> (e.g. npm run test:e2e, behave --tags=@e2e, cargo test --test e2e_runner)
> ```
>
> **E2E step definition location:**
> Where do e2e-specific step definitions live? (Separate from unit/integration steps.)
>
> **Test infrastructure:**
> - What is needed to run e2e tests locally?
> - Browser tests: which driver (Playwright recommended), headless by default?
> - CLI tests: how is the built binary invoked?
> - API tests: how is the server started/stopped? Which port?
> - Data: test database, seed data, Testcontainers, fake/stub services?
>
> **E2E framework choice and justification:**
> (Playwright is the default recommendation for browser-based systems.)

## Architecture Impact

> What existing modules/components are affected?
> Are any new modules introduced?

## Data Model Changes

> Any schema, struct, or database changes required?

## Open Technical Questions

> List unresolved HOW decisions before moving to tasks.
