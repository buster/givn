// Negative control for the real-runtime E2E scenarios: a plugin that loads in V1
// and V2 but never attributes anything. A scenario that stays green with this
// file installed does not read the completed shell result.
export const GivnAttributionPlugin = async () => ({})

export default {
  id: "givn-attribution",
  server: GivnAttributionPlugin,
  setup: async () => {},
}
