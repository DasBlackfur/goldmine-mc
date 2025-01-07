local gm_module = {}

local registry = require("@goldmine/registry")
gm_module.registry = registry

export type Mod = {name: string, version: number}
function gm_module.register_mod(mod: Mod): () end

return gm_module