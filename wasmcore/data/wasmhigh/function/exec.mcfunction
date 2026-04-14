# Params:
# locals.0: string start addr
# locals.1: string size

execute store result score v0 wasm run data get storage wasm:c locals[-1][0]
execute store result score v1 wasm run data get storage wasm:c locals[-1][1]
scoreboard players operation v1 wasm += v0 wasm

data modify storage wasm:c rets.str set value ""

execute if score v0 wasm < v1 wasm run function wasmlow:bytes_to_str
function wasmlow:exec with storage wasm:c rets

data remove storage wasm:c locals[-1]