# Params: a0: mem_id, v0: start, v1: size

scoreboard players operation v1 wasm += v0 wasm
data modify storage wasm:c rets.r0 set value ""
$execute if score v0 wasm < v1 wasm run function bytes_to_str_inner {a0: $(a0)}