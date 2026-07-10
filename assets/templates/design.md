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

> Undefined/pending steps must FAIL the runner, never silently pass or skip.
> Exact mechanism for the chosen runner:
> ```
> (e.g. cucumber-rs: .fail_on_skipped(); cucumber-js: --strict, pinned in
>  cucumber.js; cucumber-jvm CLI: --strict; cucumber-jvm JUnit Platform
>  Suite: no native flag — never leave a step body empty, plus
>  <plugin/listener> failing on UNDEFINED/PENDING; behave: every new step
>  starts with raise NotImplementedError(...))
> ```
>
> **Not-implemented stub for this language** (every step without a real
> assertion yet — never an empty body):
> ```
> (e.g. Java: throw new io.cucumber.java.PendingException("TODO");
>  Python: raise NotImplementedError("TODO");
>  JS/TS: return "pending"; or throw new Error("TODO");
>  Rust: unimplemented!() or todo!(); Go: return godog.ErrPending;
>  Ruby: pending("TODO"))
> ```

## E2E Smoke Test Infrastructure

> **E2E runner command** (→ `verify.e2e_command`):
> ```
> (e.g. npm run test:e2e, behave --tags=@e2e, cargo test --test e2e_runner)
> ```
> **E2E step location** (separate from unit/integration, one file per capability):
>
> **Local test infrastructure:** browser driver (Playwright default,
> headless), CLI invocation, server start/stop/port, test DB/seed
> data/Testcontainers/fakes.
>
> **E2E framework choice and justification:**

## Architecture Impact

> What existing modules/components are affected?
> Are any new modules introduced?

## Data Model Changes

> Any schema, struct, or database changes required?

## Open Technical Questions

> List unresolved HOW decisions before moving to tasks.
