# Params: a0: reg_name, a1: local_id
# Modifies: args

$data modify storage wasm:c args.a0 set value $(a0)
execute store result storage wasm:c args.a1 int 1 run scoreboard players get local_index wasm
$data modify storage wasm:c args.a2 set value $(a1)
function local_to_reg_inner with storage wasm:c args