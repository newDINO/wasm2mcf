# Params: a0: mem_id
# Modifies: args1, args

# Copy a char into args.a1
data modify storage wasm:c args1.a0 set from value 1
$data modify storage wasm:c args1.a1 set from value $(a0)
execute store result storage wasm:c args1.a2 int 1 run scoreboard players get v0 wasm
function wasmcore:mem_to_args with args1

# Concat rets.r0 and args.a1
data modify storage wasm:c args.a0 set from storage wasm:c rets.r0
function string_cat with storage wasm:c args

scoreboard players operation v0 wasm += one wasm
$execute if score v0 wasm < v1 wasm run function bytes_to_str_inner {a0: $(a0)}