# Instruction: steps-go

`godog` mechanics. The universal rules stay in `givn instructions steps`.

- Step definition registration: `ctx.Step(`; one expression per binding.
- Unimplemented body: `return godog.ErrPending`.
- Strict mode: `godog.ErrPending` fails by default; no flag is needed.
- Single scenario: `go test ./... -run TestFeatures`.
