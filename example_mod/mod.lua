local gm = require("goldmine")

local function listener(packet)
    ---print(packet:toString())
    return packet
end

local function test_api()
    print("API called")
end

gm.register_mod({name="example_mod", version=000_001_000})

gm.registry.pl_registry.register("example_mod/listener", listener)

print("Print available registries")
for k,_ in gm.registry do
    print(k)
end

gm.registry.api_registry.register("example_mod/myapi", {test=test_api})
gm.registry.api_registry.get("example_mod/myapi").test()