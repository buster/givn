# Design: {{CHANGE_ID}}

## Overview

> How will we implement this change? Summarise the approach.

## Technology Decisions

> What language, framework, libraries, or patterns will be used?
> Justify decisions made here (why this approach over alternatives?).

## Step Definitions

> Location of step definition files — **one file per capability**, never one
> file for the whole change:
> | Capability | Step definition file |
> |---|---|
> | | |
>
> Test runner command (from `verify.command` in givn/config.yaml):
> ```
> {{VERIFY_COMMAND}}
> ```
>
> Single-scenario run command (exact invocation to run ONE named scenario,
> not the whole suite — used by every RED/GREEN check in tasks.md):
> ```
> (e.g. cargo test <name> -- --exact, mvn test -Dcucumber.filter.name="<title>",
>  cucumber-js --name "<title>", behave -n "<title>")
> ```

## Strict Mode (mandatory)

> Undefined/pending steps — and, in some languages, even empty step method
> bodies — must FAIL the runner, never silently pass or skip. State the exact
> mechanism for the chosen runner:
> ```
> (e.g. cucumber-rs: .fail_on_skipped() on the Cucumber builder
>       cucumber-js: --strict flag, pinned in cucumber.js
>       cucumber-jvm CLI: --strict flag
>       cucumber-jvm JUnit Platform Suite: no native strict flag — enforced by
>         NEVER leaving a step body empty (see tasks.md) plus <plugin/listener
>         name> failing the build on UNDEFINED/PENDING results
>       behave: every new step body starts with raise NotImplementedError(...))
> ```
>
> **Not-implemented stub pattern for this language** (used for every step
> without a real assertion yet — never an empty body):
> ```
> (e.g. Java: throw new io.cucumber.java.PendingException("TODO: implement");
>       Python: raise NotImplementedError("TODO: implement")
>       JS/TS: return "pending";  or  throw new Error("TODO: implement")
>       Rust: unimplemented!()  or  todo!()
>       Go: return godog.ErrPending
>       Ruby: pending("TODO: implement"))
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
