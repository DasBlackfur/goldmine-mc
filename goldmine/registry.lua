type internal_Registry = {
    pl_registry: Registry?,
    api_registry: Registry?,
    lm_registry: Registry?
}

local registry: internal_Registry = {}

export type Registry = {register: (string, any) -> (), get: (string) -> any, values: () -> {}}

return registry