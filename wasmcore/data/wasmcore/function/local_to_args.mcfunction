# Params: a0: arg_id, a1: local_id
# Modifies: args1, args

execute store result storage wasm:c args1.a0 int 1 run scoreboard players get local_index wasm
$data modify storage wasm:c args1.a1 set value $(a1)
$data modify storage wasm:c args1.a2 set value $(a0)
function wasmcore:local_to_args_inner with storage wasm:c args1