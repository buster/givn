const sessions = new Map()
const UNAVAILABLE = "unavailable"

const rememberModel = (sessionID, model, variant) => {
  const previous = sessions.get(sessionID) ?? {}
  sessions.set(sessionID, {
    providerID: model?.providerID ?? previous.providerID,
    modelID: model?.modelID ?? previous.modelID,
    variant: variant ?? previous.variant,
  })
}

export const GivnAttributionPlugin = async () => {
  return {
    "chat.message": async (input) => {
      rememberModel(input.sessionID, input.model, input.variant)
    },
    "chat.params": async (input) => {
      rememberModel(input.sessionID, input.model)
    },
    "shell.env": async (input, output) => {
      output.env.GIVN_ATTRIBUTION_ACTIVE = "1"
      output.env.GIVN_ATTRIBUTION_LLM = UNAVAILABLE
      output.env.GIVN_ATTRIBUTION_REASONING = UNAVAILABLE
      output.env.GIVN_ATTRIBUTION_HARNESS = "opencode"

      const model = sessions.get(input.sessionID)
      if (!model?.modelID) return

      const identity = model.providerID
        ? `${model.providerID}/${model.modelID}`
        : model.modelID
      output.env.GIVN_ATTRIBUTION_LLM = identity
      output.env.GIT_COMMITTER_NAME = `LLM: ${identity} via opencode`
      if (model.variant) output.env.GIVN_ATTRIBUTION_REASONING = model.variant
    },
  }
}
