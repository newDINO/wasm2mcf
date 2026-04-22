# Params:
# $(si)
# l0: string start addr
# l1: string size

$scoreboard players operation l1 wasm$(si) += l0 wasm$(si)
data modify storage wasm:s temp.str set value ""

$execute if score l0 wasm$(si) < l1 wasm$(si) run function wasmlow:bytes_to_str { si: $(si) }
function wasmlow:exec with storage wasm:s temp

$scoreboard objectives remove wasm$(si)