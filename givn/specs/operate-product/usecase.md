<!-- givn:base-sha256:254386e0a93fd765e52f9817c3955608987104d431cf89b6085a7429dd6abb7f -->
# Use case: operate-product

## Level

!

## Actors

- Product operator

## Goal

Run the declared product to a reported outcome.

## Trigger

The operator starts the declared product.

## Preconditions

- The project declares a valid product start plan.

## Main flow

1. The operator starts the declared product; the system resolves the declared start plan.
2. The system starts the product.
3. The system waits for the declared readiness.
4. The system reports the product outcome.

## Extensions

- 1: the declaration is invalid → the product is not started and the declaration is named; protects the minimal guarantee.

## Rules

- A server has exactly one port source.
- Readiness is confirmed only by a successful response through the declared boundary before its deadline.
- Connection, request write, response read, and retry waits all share one finite total readiness deadline. Every operation is bounded by the remaining time, never a fresh full timeout after the deadline.
- A stalled connection, unconsumed request, absent response, or incomplete response cannot wait indefinitely or print ready.
- Readiness expiry exits with code 2 within the deadline plus bounded existing cleanup time; the started product is stopped and reaped. This adds no supervision framework.

## Examples

- An operator starts a server and receives a ready line.

## Minimal guarantee

The product is never reported ready for an invalid declaration, failed start, or expired readiness deadline; stalled readiness exchanges terminate with bounded product cleanup.

## Success guarantee

The operator observes the product's lifecycle outcome, with declared readiness bounded across connection and exchange and failures cleaned up rather than left waiting indefinitely.

## Personas

- developer--primary-user

## Capabilities

- declare-the-product-start
- start-the-declared-product

## Interactions

| Capability | Consumer action | E2E scenario |
|---|---|---|
| start-the-declared-product | start a server | The started server reports readiness and forwards a clean exit |
| start-the-declared-product | hand over a caller port | A caller-provided port is handed to the product |
| start-the-declared-product | print the start plan | Print mode emits the start plan without starting |
| start-the-declared-product | interrupt a started product | An interrupt stops the started product |
| declare-the-product-start | scaffold a run declaration | Scaffolding ships an unconfigured run declaration |
| declare-the-product-start | check an unconfigured declaration | The review check reports an unconfigured run declaration |
| declare-the-product-start | archive a configured project | Archive passes once the run declaration is complete |
| declare-the-product-start | upgrade a project declaration | Upgrade backfills the run declaration into an existing project |
| declare-the-product-start | inspect a damaged README block | The check names a damaged run managed block |
| declare-the-product-start | invoke the sentinel command | The sentinel start command prints setup guidance |

## Includes

- fragment: support

## Extends

- none

## Out of scope

Choosing a product architecture or inventing a start command from repository content.

## Diagram

```mermaid
flowchart LR
  Operator((Operator)) --> Plan[[Resolve start plan]]
  Plan --> Product[[Start product]]
  Product --> Ready[[Observe readiness]]
```
