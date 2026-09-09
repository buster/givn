const sessions = new Map()

export const GivnAttributionPlugin = async () => {
  return {
    "chat.message": async (input) => {
      sessions.set(input.sessionID, {
        providerID: input.providerID,
        modelID: input.modelID,
        variant: input.variant,
      })
    },
    "shell.env": async (input, output) => {
      output.env.GIVN_ATTRIBUTION_ACTIVE = "1"
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
