execute store result storage wasm:c args.a0 int 1 run scoreboard players get local_index wasm
$data modify storage wasm:c args.a1 set value $(a0)
function wasmcore:local_get_inner with storage wasm:c args