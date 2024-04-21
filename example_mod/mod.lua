local gm = require("goldmine")

local function listener(packet)
    print(packet)
end

gm.register_mod({name="example_mod", version=000_001_000})

gm.registry.pl_registry.register("example_mod/listener", listener)