# Params:
# regs.v0: start addr
# regs.v1: end addr
# rets.str

execute store result storage wasm:c args.addr int 1 run scoreboard players get v0 wasm
execute store result storage wasm:c args.charcode int 1 run function wasmlow:mem_get with storage wasm:c args
function wasmlow:push_char with storage wasm:c args

scoreboard players add v0 wasm 1
execute if score v0 wasm < v1 wasm run function wasmlow:bytes_to_str